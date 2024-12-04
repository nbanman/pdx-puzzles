package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.gvs

private typealias Operation = (Long, Long) -> Long

class Y22D21(private val input: String) : Day {

    // Manages various maps. The maps are populated with the initialize fun. They cannot be prepopulated because
    // the dispatcher is passed by reference to the monkeys that it tracks.
    class NumberDispatcher {
        // Tracks what each monkey says, updated when it can be ascertained what the monkey says
        val monkeySay = mutableMapOf<String, Long>()

        // Tracks the relationships between monkeys
        val leftCallRegister = mutableMapOf<String, MutableList<String>>()
        val rightCallRegister = mutableMapOf<String, MutableList<String>>()

        // Ensures that initialize is called and does not allow updates before that.
        private var initializeRun = false

        // runs updateNumberRegister for every monkey whose numbers are known, to be called after all the 
        // monkeys are created.
        fun initialize(monkeys: Map<String, Monkey>) {
            if (initializeRun) throw IllegalStateException("Initialize has already been called once.")
            initializeRun = true
            monkeySay.toList().forEach { (name, number) ->
                updateMonkeySay(name, number, monkeys)
            }
        }

        // updates monkeySay to include the known number of the monkey, then calls update on every monkey that
        // is waiting for that monkey's number to see if that is enough information 
        fun updateMonkeySay(name: String, number: Long, monkeys: Map<String, Monkey>) {
            if (!initializeRun) throw IllegalStateException("Initialize has not been called yet.")
            monkeySay[name] = number // redundant in case of initialization, but important update otherwise
            leftCallRegister[name]?.forEach { monkey ->
                monkeys[monkey]?.update(monkeys)
            }
            rightCallRegister[name]?.forEach { monkey ->
                monkeys[monkey]?.update(monkeys)
            }
        }
    }

    class Monkey(
        val name: String,
        private val left: String,
        private val right: String,
        private val operation: Operation,
        private val dispatcher: NumberDispatcher,
    ) {

        init {
            dispatcher.leftCallRegister.getOrPut(left) { mutableListOf() }.add(name)
            dispatcher.rightCallRegister.getOrPut(right) { mutableListOf() }.add(name)
        }

        fun update(monkeys: Map<String, Monkey>): Boolean {
            val leftNumber = dispatcher.monkeySay[left] ?: return false
            val rightNumber = dispatcher.monkeySay[right] ?: return false
            dispatcher.updateMonkeySay(name, operation(leftNumber, rightNumber), monkeys)
            return true
        }

        fun toEquation(monkeys: Map<String, Monkey>): Equation {

            fun String.toEquation() = dispatcher.monkeySay[this]
                ?.let { Equation.Number(it) } // if known, it's a number
                ?: (monkeys[this]?.toEquation(monkeys) ?: Equation.X) // else, if in monkey list, a Composite
            //otherwise, it's X.

            return dispatcher.monkeySay[name] // Check if the number is known
                ?.let { Equation.Number(it) } // if known, it's a Number
                ?: Equation.Composite( // otherwise it's a composite of two equations
                    left.toEquation(),
                    right.toEquation(),
                    operation
                )
        }

        override fun toString(): String {
            return "Monkey::Monkey(left='$left', right='$right', operation=$operation)"
        }
    }

    sealed class Equation {
        object X : Equation() {
            // gets called when the equation is done and x is isolated on one side, returns the other side
            override fun solveForX(to: Equation) = to
            override fun containsX() = true // well duh
            override fun calculate() = throw IllegalArgumentException("x should be removed from equation")

            override fun toString() = "x"
        }

        class Number(private val value: Long) : Equation() {
            override fun solveForX(to: Equation) = throw IllegalArgumentException("number cannot be rearranged")
            override fun containsX() = false // well duh
            override fun calculate() = value // very simple calculation!

            override fun toString() = "$value"
        }

        class Composite(
            private val left: Equation,
            private val right: Equation,
            private val operation: Operation
        ) : Equation() {

            // Very clumsy backwards hack to find the operation used. Could not get reflection working.
            private fun opString() = when (operation(6, 2)) {
                8L -> "+"
                4L -> "-"
                12L -> "*"
                else -> "/"
            }

            // Assuming the equation has x somewhere in it, keep breaking the equation into component parts and 
            // move the component that doesn't have x to the other side ("to"). Keep building up the "to" side,
            // then recursively calling on the component that has x in it, until x is isolated.
            // This returns an equation that solves for x without any variables.
            override fun solveForX(to: Equation): Equation {
                val moveRight = left.containsX()
                val (move, stay) = if (moveRight) right to left else left to right
                val newOp: Operation = when (opString()) {
                    "+" -> Long::minus
                    "-" -> if (moveRight) Long::plus else ::oddMinus
                    "*" -> Long::div
                    else -> if (moveRight) Long::times else ::oddDiv
                }
                val newTo = Composite(to, move, newOp)
                return stay.solveForX(newTo)
            }

            // Once x is removed, recursively calculate the equation.
            override fun calculate() = operation(left.calculate(), right.calculate())

            // Recursively determine if either component contains x. 
            override fun containsX() = left.containsX() || right.containsX()

            override fun toString(): String {
                return "($left ${opString()} $right)"
            }

            companion object {
                fun oddMinus(a: Long, b: Long): Long = -(a - b)

                fun oddDiv(a: Long, b: Long): Long = b / a
            }
        }

        abstract fun containsX(): Boolean

        abstract fun solveForX(to: Equation): Equation

        abstract fun calculate(): Long
    }

    // gets the dispatcher and the waiting monkeys. The monkey list is tweaked in part 2 and the dispatcher is 
    // modified accordingly.
    private fun getComponents(part2: Boolean = false): Pair<NumberDispatcher, Map<String, Monkey>> {
        val dispatcher = NumberDispatcher()
        val monkeys = input.gvs("""(\w+): (\d+)?(?:(\w+) ([-+*/]) (\w+))?""")
            .mapNotNull { gv ->
                val number = gv[1].toLongOrNull()
                if (number == null) {
                    val operation = if (part2 && gv[0] == "root") Long::div else gv[3].toOperation()
                    Monkey(gv[0], gv[2], gv[4], operation, dispatcher)
                } else {
                    if (!part2 || gv[0] != "humn") dispatcher.monkeySay[gv[0]] = number
                    null
                }
            }.associateBy { it.name }
        dispatcher.initialize(monkeys)
        return dispatcher to monkeys
    }

    override fun part1(): Long {
        val (dispatcher, _) = getComponents()
        return dispatcher.monkeySay.getValue("root")
    }

    override fun part2(): Long {
        val (_, monkeys) = getComponents(true)
        return monkeys
            .getValue("root")// start with the monkey named "root"
            .toEquation(monkeys) // turn into equation
            .solveForX(Equation.Number(1L)) // change equation to solve for x
            .calculate() // calculate equation
    }

    companion object {
        private fun String.toOperation(): Operation = when (this) {
            "+" -> Long::plus
            "-" -> Long::minus
            "*" -> Long::times
            else -> Long::div
        }
    }
}

fun main() = Day.runDay(Y22D21::class)

//    Class creation: 2ms
//    Part 1: 309248622142100 (35ms)
//    Part 2: 3757272361782 (13ms)
//    Total time: 52ms
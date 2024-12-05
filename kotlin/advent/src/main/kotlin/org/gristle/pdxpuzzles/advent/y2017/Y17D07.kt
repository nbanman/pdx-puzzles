package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.groupValues

class Y17D07(input: String) : Day {

    data class Program(val name: String, val weight: Int, val childNames: List<String>) {
        companion object {
            // register is a mutable hashmap shared by all instances used to get a parent from the parent's name
            private val register = mutableMapOf<String, Program>()

            // public getter of register
            operator fun get(name: String): Program? = register[name]
        }

        // init registers the instance to the shared register.
        init {
            register[name] = this
        }

        // lazy variables because we need to wait until all the programs are created before we can
        // link the programs together
        private val children: List<Program> by lazy {
            childNames.map { register[it] ?: throw IllegalStateException("child not in register") }
        }

        private val totalWeight: Int by lazy { weight + children.sumOf { it.totalWeight } }

        fun rebalance(idealWeight: Int = 0): Int {
            // if all children have the same total weight...
            if (children.all { it.totalWeight == children.first().totalWeight }) {
                // ...then this particular program's weight needs to change, then we can RETURN that value
                // calculated by taking the ideal weight and subtracting the weight of the children
                return idealWeight - children.sumOf { it.totalWeight }
            }
            // otherwise we continue...
            // we calculate the weight that each child needs to be. There is a starting case and a general case.
            val idealChildWeight = if (idealWeight == 0) {
                // starting case where we don't know what's unbalanced yet, but we definitely have one child
                // that's unlike at least two others that are the same. Thus, the weight that is shared between
                // at least two children is the ideal child weight.
                children.groupingBy { it.totalWeight }.eachCount().entries.first { it.value > 1 }.key
            } else {
                // The "starting case" algorithm works in cases where there are 3+ children, but it doesn't
                // know which branch to correct when there are two children.
                // However, we now know the amount needed to balance down below, so the ideal child weight is the
                // ideal weight minus the current node's weight, then divided evenly among the children.
                (idealWeight - weight) / children.size
            }
            // Find the child that does not have the proper weight and rebalance it recursively.
            return children.first { it.totalWeight != idealChildWeight }.rebalance(idealChildWeight)
        }
    }

    // Parse input to a list of 'Program's

    private val pattern = """(\w+) \((\d+)\)(?: -> (.*))?""".toRegex()
    private val programs = input
        .groupValues(pattern)
        .map { (name, weight, childStr) ->
            val childNames = childStr.split(", ").let { if (it.first() == "") emptyList() else it }
            Program(name, weight.toInt(), childNames)
        }

    // The bottom program has no programs that reference it as a child.
    private val bottomProgram = let {
        // All program names
        val pgNames = programs.map(Program::name).toSet()
        // All program names that are referenced as a child
        val childNames = programs.flatMap(Program::childNames).toSet()
        // The program whose name is left over once the child references are removed
        Program[(pgNames - childNames).first()] ?: throw IllegalArgumentException()
    }

    override fun part1() = bottomProgram.name

    override fun part2() = bottomProgram.rebalance()
}

fun main() = Day.runDay(Y17D07::class)

//    Class creation: 35ms
//    Part 1: airlri (0ms)
//    Part 2: 1206 (3ms)
//    Total time: 39ms
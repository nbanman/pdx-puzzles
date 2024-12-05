package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y15D07(input: String) : Day {
    
    // each instruction stored as a list of arguments
    private val instructions = input.lines().map { it.split(' ') }

    // global register of wire values
    private val register = mutableMapOf<String, Int>()

    // utility function that takes an argument and returns an Int. Converts digits to an int; failing that, looks
    // up the wire value in the register. Otherwise, returns null. 
    private fun String.value(): Int? = toIntOrNull() ?: register[this]

    // tries to execute the instruction, returning true if successful and false if one of the constituent wires is
    // not yet defined
    private fun List<String>.runInstruction(): Boolean {
        when (size) {
            3 -> this[0].value()
                ?.let { register[this[2]] = it } // ASSIGN
                ?: return false

            4 -> this[1].value()
                ?.let { register[this[3]] = it xor Int.MAX_VALUE and 65535 } // NOT
                ?: return false

            else -> when (this[1]) {
                "AND" -> this[0].value()?.let { arg1 ->
                    this[2].value()?.let { arg2 ->
                        register[this[4]] = arg1 and arg2
                    } ?: return false
                } ?: return false

                "OR" -> this[0].value()?.let { arg1 ->
                    this[2].value()?.let { arg2 ->
                        register[this[4]] = arg1 or arg2
                    } ?: return false
                } ?: return false

                "LSHIFT" -> this[0].value()?.let {
                    register[this[4]] = it shl this[2].toInt() and 65535
                } ?: return false

                "RSHIFT" -> this[0].value()?.let {
                    register[this[4]] = it shr this[2].toInt()
                } ?: return false

                else -> throw IllegalArgumentException("Command not recognized: ${this[1]}")
            }
        }
        return true
    }

    // executes instructions in a loop until all are executed
    fun solve(instructions: List<List<String>>): Int {

        // not all instructions can be immediately executed, so use a var to be able to revise the instruction list
        var instructionList = instructions

        // use a mutable list to track instuctions that were not executed
        val unexecutedInstructions = mutableListOf<List<String>>()

        // run until all instructions have been executed
        while (instructionList.isNotEmpty()) {

            // try to execute every instruction; add any failures to unexecutedInstructions
            instructions.forEach { if (!it.runInstruction()) unexecutedInstructions.add(it) }

            // after each pass, transfer the failures to the instructionList to try again
            instructionList = unexecutedInstructions.toList()
            unexecutedInstructions.clear() // cleanup
        }
        return register.getValue("a")
    }

    override fun part1(): Int {
        register.clear()
        return solve(instructions)
    }

    override fun part2(): Int {
        // run solve once to get value for 'b'
        register.clear()
        val b = solve(instructions)

        // rerun solve, this time adding the value for 'b' and ignoring the instruction assigning a value to 'b'
        register.clear()
        register["b"] = b
        val instructions = instructions.filter { it.last() != "b" }
        return solve(instructions)
    }
}

fun main() = Day.runDay(Y15D07::class)

//    Class creation: 20ms
//    Part 1: 46065 (15ms)
//    Part 2: 14134 (13ms)
//    Total time: 49ms

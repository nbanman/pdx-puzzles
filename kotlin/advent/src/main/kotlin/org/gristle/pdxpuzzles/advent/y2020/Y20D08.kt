package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y20D08(input: String) : Day {

    data class Instruction(val operation: String, val argument: Int)

    private fun Instruction.execute() {
        when (operation) {
            "acc" -> {
                acc += argument; parser++
            }

            "jmp" -> parser += argument
            else -> parser++
        }
    }

    var acc = 0
    private var parser = 0

    private fun reset() {
        acc = 0
        parser = 0
    }

    private val instructions = input
        .lineSequence()
        .map {
            val (operation, amt) = it.split(' ')
            Instruction(operation, amt.toInt())
        }.toList()

    private fun solve(flippedIndex: Int = -1): Int {
        val pastStates = BooleanArray(instructions.size)

        while (parser in instructions.indices) {
            if (pastStates[parser]) return acc
            pastStates[parser] = true
            val current = instructions[parser]
            val fCurrent = if (parser == flippedIndex) {
                when (current.operation) {
                    "nop" -> Instruction("jmp", current.argument)
                    "jmp" -> Instruction("nop", current.argument)
                    else -> current
                }
            } else current
            fCurrent.execute()
        }

        return -acc // no infinite loop, so return negative number to denote that
    }

    override fun part1() = solve()

    override fun part2(): Int {
        for (flippedInstruction in instructions.indices) {
            if (instructions[flippedInstruction].operation == "acc") continue
            reset()
            val answer =
                -solve(flippedInstruction) // inverse means negative answer means infinite loop encountered
            if (answer >= 0) return answer
        }
        return -1
    }
}

fun main() = Day.runDay(Y20D08::class)

//    Class creation: 3ms
//    Part 1: 1915 (1ms)
//    Part 2: 944 (5ms)
//    Total time: 10ms
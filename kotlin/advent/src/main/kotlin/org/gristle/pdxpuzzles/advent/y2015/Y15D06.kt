package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getIntList
import kotlin.math.abs
import kotlin.math.max

class Y15D06(input: String) : Day {

    data class Instruction(val command: Int, val x1: Int, val y1: Int, val x2: Int, val y2: Int) {
        companion object {
            fun of(line: String): Instruction {
                val command = when {
                    line.startsWith("turn on") -> 1
                    line.startsWith("turn off") -> -1
                    line.startsWith("toggle") -> 0
                    else -> throw IllegalArgumentException("Error parsing input: $line")
                }
                val (x1, y1, x2, y2) = line.getIntList()
                return Instruction(command, x1, y1, x2, y2)
            }
        }
    }

    val instructions = input
        .lineSequence()
        .map(Instruction::of)
        .toList()

    val length = 1_000

    fun solve(operation: (lights: IntArray, index: Int, command: Int) -> Int): Int {
        val lights = IntArray(length * length)
        fun IntArray.execute(instruction: Instruction) {
            for (y in instruction.y1..instruction.y2) for (x in instruction.x1..instruction.x2) {
                val index = y * length + x
                this[index] = operation(lights, index, instruction.command)
            }
        }
        instructions.forEach { lights.execute(it) }
        return lights.sum()
    }

    override fun part1(): Int {
        fun operation(lights: IntArray, index: Int, command: Int) = when (command) {
            1 -> 1
            -1 -> 0
            else -> abs(lights[index] - 1)
        }
        return solve(::operation)
    }

    override fun part2(): Int {
        fun operation(lights: IntArray, index: Int, command: Int) = when (command) {
            1 -> lights[index] + 1
            -1 -> max(0, lights[index] - 1)
            else -> lights[index] + 2
        }
        return solve(::operation)
    }
}

fun main() = Day.runDay(Y15D06::class)

//    Class creation: 28ms
//    Part 1: 569999 (34ms)
//    Part 2: 17836115 (37ms)
//    Total time: 100ms
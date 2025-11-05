package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit

object Y25D01 : Day {
    private fun parse(input: String): Pair<List<String>, List<Pair<Char, Int>>> {
        val (nameStr, commandStr) = input.trimEnd().blankSplit()
        val names = nameStr.split(',')
        val commands = commandStr
            .splitToSequence(',')
            .map { command ->
                val dir = command[0]
                val num = command.drop(1).toInt()
                dir to num
            }.toList()
        return names to commands
    }

    override fun part1(input: String): String {
        val (names, commands) = parse(input)
        val maximum = names.lastIndex
        var index = 0;
        for ((dir, num) in commands) {
            when (dir) {
                'L' -> index = (index - num).coerceAtLeast(0)
                'R' -> index = (index + num).coerceAtMost(maximum)
            }
        }
        return names[index]
    }
    override fun part2(input: String): String {
        val (names, commands) = parse(input)
        var index = 0;
        for ((dir, num) in commands) {
            when (dir) {
                'L' -> index -= num
                'R' -> index += num
            }
        }
        return names[index.rem(names.size)]
    }
    override fun part3(input: String): String {
        val (namesList, commands) = parse(input)
        val names = namesList.toMutableList()
        for ((dir, num) in commands) {
            val index = when (dir) {
                'L' -> -num
                'R' -> num
                else -> 0
            }
            val modIndex = ((index % names.size) + names.size) % names.size

            names[0] = names[modIndex].also { names[modIndex] = names[0] }
        }
        return names[0]
    }
}

fun main() = Day.runDay(Y25D01::class)

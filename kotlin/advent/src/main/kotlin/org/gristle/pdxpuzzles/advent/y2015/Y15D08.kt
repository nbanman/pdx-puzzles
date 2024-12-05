package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y15D08(input: String) : Day {
    private val lines = input.lines()

    private val replaceRx = """\\\\|\\"|\\x[\da-f]{2}""".toRegex()

    private fun String.charsInMemory(): Int {
        return drop(1).dropLast(1).replace(replaceRx, "X").length
    }

    private fun String.encodedLength() = length + 2 + count { it in """\"""" }

    private val totalLength = lines.sumOf(String::length)

    override fun part1() = totalLength - lines.sumOf { it.charsInMemory() }

    override fun part2() = lines.sumOf { it.encodedLength() } - totalLength
}

fun main() = Day.runDay(Y15D08::class)

//    Class creation: 23ms
//    Part 1: 1333 (10ms)
//    Part 2: 2046 (1ms)
//    Total time: 36ms
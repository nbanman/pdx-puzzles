package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y21D01(input: String) : Day {
    private val measurements = input.getInts()

    private fun Sequence<Int>.countIncreased() = zipWithNext().count { (a, b) -> a < b }

    override fun part1() = measurements.countIncreased()

    override fun part2() = measurements
        .windowed(3) { it.sum() }
        .countIncreased()
}

fun main() = Day.runDay(Y21D01::class)

//    Class creation: 30ms
//    Part 1: 1342 (0ms)
//    Part 2: 1378 (5ms)
//    Total time: 37ms
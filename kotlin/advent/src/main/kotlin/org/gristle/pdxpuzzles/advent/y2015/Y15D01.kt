package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y15D01(input: String) : Day {
    private val floorChanges: Sequence<Int> = input.asSequence().map { if (it == '(') 1 else -1 }
    override fun part1(): Int = floorChanges.sum()
    override fun part2(): Int = floorChanges.runningFold(0, Int::plus).indexOfFirst { it == -1 }
}

fun main() = Day.runDay(Y15D01::class)

//    Class creation: 20ms
//    Part 1: 280 (2ms)
//    Part 2: 1797 (5ms)
//    Total time: 28ms

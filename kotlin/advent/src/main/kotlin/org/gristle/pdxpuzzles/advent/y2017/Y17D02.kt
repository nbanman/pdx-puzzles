package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.combinatorics.getPairs
import org.gristle.pdxpuzzles.utilities.iteration.minMax

class Y17D02(input: String) : Day {
    private val spreadsheet: List<List<Int>> = input.lines().map { it.split('\t').map(String::toInt) }

    private inline fun solve(lineOperation: (List<Int>) -> Int): Int = spreadsheet.sumOf(lineOperation)

    override fun part1() = solve { row -> row.minMax().let { (min, max) -> max - min } }

    override fun part2() = solve { row ->
        row.getPairs().sumOf {
            val (smaller, larger) = it.minMax()
            if (larger % smaller == 0) larger / smaller else 0
        }
    }
}

fun main() = Day.runDay(Y17D02::class)

//    Class creation: 5ms
//    Part 1: 45972 (0ms)
//    Part 2: 326 (3ms)
//    Total time: 9ms
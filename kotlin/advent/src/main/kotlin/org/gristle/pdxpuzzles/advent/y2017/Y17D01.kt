package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y17D01(input: String) : Day {
    private val numbers = input.map(Char::digitToInt)

    private inline fun solve(comparisonIndex: List<Int>.(index: Int) -> Int): Int = numbers
        .filterIndexed { index, i -> numbers[(numbers.comparisonIndex(index)) % numbers.size] == i }
        .sum()

    override fun part1() = solve { it + 1 }

    override fun part2() = solve { it + size / 2 }
}

fun main() = Day.runDay(Y17D01::class)

//    Class creation: 3ms
//    Part 1: 1182 (0ms)
//    Part 2: 1152 (0ms)
//    Total time: 4ms
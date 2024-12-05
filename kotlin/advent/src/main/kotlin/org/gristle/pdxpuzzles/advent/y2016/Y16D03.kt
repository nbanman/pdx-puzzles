package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.utilities.iteration.collate
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y16D03(private val input: String) : Day {

    private fun isValid(triangle: List<Int>) = triangle.sorted().let { (a, b, c) -> a + b > c }

    private inline fun solve(rearrange: Sequence<Int>.() -> Sequence<Int> = { this }) = input
        .getInts()
        .rearrange()
        .chunked(3)
        .count(::isValid)

    override fun part1() = solve()
    override fun part2() = solve { collate(3).flatten() }
}

fun main() = Day.runDay(Y16D03::class)

//    Class creation: 17ms
//    Part 1: 1032 (10ms)
//    Part 2: 1838 (5ms)
//    Total time: 33ms
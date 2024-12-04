package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.utilities.iteration.collate
import org.gristle.pdxpuzzles.utilities.parsing.getIntList
import kotlin.math.abs

class Y24D1(private val input: String) : Day {

    override fun part1() = input
        .getIntList()
        .collate(2)
        .map { it.sorted() }
        .let { (a, b) -> a zip b }
        .sumOf { (a, b) -> abs(a - b) }

    override fun part2(): Int {
        val (a, b) = input.getIntList().collate(2)
        val freq = b.groupingBy { it }.eachCount()
        return a.sumOf { it * (freq[it] ?: 0) }
    }
}

fun main() = Day.runDay(Y24D1::class)

//    Class creation: 1ms
//    Part 1: 1222801 (11ms)
//    Part 2: 22545250 (6ms)
//    Total time: 20ms
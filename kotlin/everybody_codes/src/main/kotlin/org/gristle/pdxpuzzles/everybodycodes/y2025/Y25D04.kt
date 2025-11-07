package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import kotlin.math.ceil

object Y25D04 : Day {
    private fun firstAndLast(input: String): Pair<Double, Double> {
        val first = input.takeWhile { it.isDigit() }.toDouble()
        val last = input.takeLastWhile { it.isDigit() }.toDouble()
        return first to last
    }

    override fun part1(input: String) = firstAndLast(input)
        .let { (first, last) -> (2025.0 * (first / last)).toLong() }

    override fun part2(input: String) = firstAndLast(input)
        .let { (first, last) -> ceil(10_000_000_000_000.0 * (last / first)).toLong() }

    override fun part3(input: String) = input
        .getInts()
        .chunked(2) { (a, b) -> a.toDouble() / b }
        .fold(100.0, Double::times)
        .toLong()
}

fun main() = Day.runDay(Y25D04::class)

//    Quest 1: 12980 (0ms)
//    Quest 2: 2394789579159 (2ms)
//    Quest 3: 220503433846 (2ms)
//    Total time: 5ms
package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getIntList
import kotlin.math.max

class Y19D01(input: String) : Day {
    private val modules = input.getIntList()
    private fun baseFuel(weight: Int): Int = weight / 3 - 2
    private tailrec fun totalFuel(remaining: Int, total: Int = 0): Int {
        return if (remaining == 0) {
            total
        } else {
            val fuel = max(0, baseFuel(remaining))
            totalFuel(fuel, total + fuel)
        }
    }
    override fun part1() = modules.sumOf(::baseFuel)
    override fun part2() = modules.sumOf(::totalFuel)
}

fun main() = Day.runDay(Y19D01::class)

//    Class creation: 2ms
//    Part 1: 3325347 (0ms)
//    Part 2: 4985145 (0ms)
//    Total time: 2ms
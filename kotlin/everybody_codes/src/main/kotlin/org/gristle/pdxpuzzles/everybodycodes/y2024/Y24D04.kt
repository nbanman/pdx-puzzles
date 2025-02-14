package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getIntList
import kotlin.math.abs

object Y24D04 : Day {
    private fun lowest(nails: List<Int>) = nails.min()
    private fun least(nails: List<Int>): Int = nails.sorted().let { it[it.size / 2] }
    private fun solve(input: String, getTarget: (List<Int>) -> Int): Int {
        val nails = input.getIntList()
        val target = getTarget(nails)
        return nails.sumOf { nail -> abs(nail - target) }
    }

    override fun part1(input: String): Int = solve(input, ::lowest)
    override fun part2(input: String): Int = solve(input, ::lowest)
    override fun part3(input: String): Int = solve(input, ::least)
}

fun main() = Day.runDay(Y24D04::class)

//    Quest 1: 70 (0ms)
//    Quest 2: 910737 (3ms)
//    Quest 3: 125485323 (1ms)
//    Total time: 4ms
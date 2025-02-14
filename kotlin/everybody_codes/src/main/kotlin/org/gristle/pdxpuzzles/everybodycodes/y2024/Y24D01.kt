package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day

object Y24D01 : Day {
    private fun solve(input: String, groups: Int): Int = input
        .chunked(groups)
        .sumOf { baddies ->
            val numberOfBaddies: Int = baddies.count { it != 'x' }
            val potions: (Char) -> Int = { baddie ->
                when (baddie) {
                    'A' -> 0
                    'B' -> 1
                    'C' -> 3
                    'D' -> 5
                    else -> 0
                }
            }
            val bonusPotions: Int = numberOfBaddies * (numberOfBaddies - 1)
            bonusPotions + baddies.sumOf(potions)
        }

    override fun part1(input: String) = solve(input, 1)
    override fun part2(input: String) = solve(input, 2)
    override fun part3(input: String) = solve(input, 3)
}

fun main() = Day.runDay(Y24D01::class)

//    Quest 1: 1354 (93ms)
//    Quest 1: 5639 (2ms)
//    Quest 1: 28180 (2ms)
//    Total time: 98ms
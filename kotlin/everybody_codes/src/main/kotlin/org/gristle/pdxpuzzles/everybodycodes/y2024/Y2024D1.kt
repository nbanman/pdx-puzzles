package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
//
//class Y2024D1 : Day {
//    private fun solve(part: Int): Int = inputs[part - 1]
//        .chunked(part)
//        .sumOf { baddies ->
//            val numberOfBaddies: Int = baddies.count { it != 'x' }
//            val potions: (Char) -> Int = { baddie ->
//                when (baddie) {
//                    'A' -> 0
//                    'B' -> 1
//                    'C' -> 3
//                    'D' -> 5
//                    else -> 0
//                }
//            }
//            val bonusPotions: Int = numberOfBaddies * (numberOfBaddies - 1)
//            bonusPotions + baddies.sumOf(potions)
//        }
//
//    override fun part1(): Any? {
//        TODO("Not yet implemented")
//    }
//
//    override fun part2(): Any? {
//        TODO("Not yet implemented")
//    }
//
//    override fun part3(): Any? {
//        TODO("Not yet implemented")
//    }
//}
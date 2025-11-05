package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getIntList
import org.gristle.pdxpuzzles.utilities.parsing.getInts

object Y25D03 : Day {
    override fun part1(input: String): Int = input.getInts().sorted().distinct().sum()
    override fun part2(input: String): Int = input.getInts().sorted().distinct().take(20).sum()
    override fun part3(input: String): Int = input.getIntList().groupingBy { it }.eachCount().values.max()
}

fun main() = Day.runDay(Y25D03::class)

//    Quest 1: 2569 (1ms)
//    Quest 2: 296 (3ms)
//    Quest 3: 3204 (5ms)
//    Total time: 10ms
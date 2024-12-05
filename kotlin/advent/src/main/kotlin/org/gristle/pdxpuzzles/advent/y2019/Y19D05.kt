package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.y2019.Intcode.FintCode

class Y19D05(private val input: String) : Day {
    override fun part1() = FintCode(input).run(listOf(1)).last()
    override fun part2() = FintCode(input).run(listOf(5)).last()
}

fun main() = Day.runDay(Y19D05::class)

//    Class creation: 19ms
//    Part 1: 7839346 (0ms)
//    Part 2: 447803 (0ms)
//    Total time: 20ms
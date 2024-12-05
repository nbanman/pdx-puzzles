package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.y2019.Intcode.FintCode


class Y19D09(private val input: String) : Day {

    override fun part1() = FintCode(input).run(listOf(1L))[0]

    override fun part2() = FintCode(input).run(listOf(2L))[0]
}

fun main() = Day.runDay(Y19D09::class)

//    Class creation: 19ms
//    Part 1: 2870072642 (2ms)
//    Part 2: 58534 (84ms)
//    Total time: 105ms
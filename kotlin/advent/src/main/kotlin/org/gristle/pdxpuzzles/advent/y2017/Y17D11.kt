package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Hexagon

class Y17D11(input: String) : Day {
    private val path: List<Hexagon> by lazy { input.split(',').runningFold(Hexagon(), Hexagon::hexAt) }
    override fun part1() = path.last().distance()
    override fun part2() = path.maxOf(Hexagon::distance)
}

fun main() = Day.runDay(Y17D11::class)

//    Class creation: 2ms
//    Part 1: 747 (4ms)
//    Part 2: 1544 (2ms)
//    Total time: 9ms
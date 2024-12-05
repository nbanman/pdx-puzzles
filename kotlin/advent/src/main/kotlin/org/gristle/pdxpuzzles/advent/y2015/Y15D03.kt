package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.utilities.iteration.collate
import org.gristle.pdxpuzzles.utilities.objects.Coord

class Y15D03(private val input: String) : Day {

    private fun deliver(s: String) = s
        .runningFold(Coord.ORIGIN) { santa, dir -> santa.move(dir) }
        .toSet()

    override fun part1() = deliver(input).size

    override fun part2(): Int = input
        .collate(2)
        .map(::deliver)
        .reduce(Set<Coord>::union)
        .size
}

fun main() = Day.runDay(Y15D03::class)

//    Class creation: 21ms
//    Part 1: 2081 (8ms)
//    Part 2: 2341 (6ms)
//    Total time: 36ms
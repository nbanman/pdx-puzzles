package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import org.gristle.pdxpuzzles.utilities.combinatorics.getPairs

class Y15D02(input: String) : Day {

    private class Box(dimensions: List<Int>) {
        private val dimensions = dimensions.sorted()

        private fun surfaceArea() = dimensions.getPairs().sumOf { (a, b) -> 2 * a * b }
        private fun smallestSideArea() = dimensions[0] * dimensions[1]
        fun paperNeeded() = surfaceArea() + smallestSideArea()

        private fun cubicVolume() = dimensions.reduce(Int::times)
        private fun ribbonToWrap() = 2 * (dimensions[0] + dimensions[1])
        fun ribbonNeeded() = cubicVolume() + ribbonToWrap()
    }

    private val boxes = input
        .getInts()
        .chunked(3, ::Box)

    override fun part1() = boxes.sumOf(Box::paperNeeded)

    override fun part2() = boxes.sumOf(Box::ribbonNeeded)
}

fun main() = Day.runDay(Y15D02::class)

//    Class creation: 20ms
//    Part 1: 1588178 (16ms)
//    Part 2: 3783758 (3ms)
//    Total time: 39ms

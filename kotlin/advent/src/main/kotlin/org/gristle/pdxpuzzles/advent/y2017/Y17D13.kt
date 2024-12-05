package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y17D13(input: String) : Day {

    private fun Pair<Int, Int>.severity(): Int {
        val (depth, range) = this
        return if (depth % ((range - 1) * 2) == 0) {
            depth * range
        } else 0
    }

    private fun Pair<Int, Int>.isTriggered(offset: Int): Boolean {
        val (depth, range) = this
        return (depth + offset) % ((range - 1) * 2) == 0
    }

    private val layers = input
        .getInts()
        .chunked(2) { (depth, range) -> depth to range }
        .toList()

    override fun part1() = layers.sumOf { it.severity() }

    override fun part2() = generateSequence(0) { it + 1 }
        .first { index -> layers.none { it.isTriggered(index) } }
}

fun main() = Day.runDay(Y17D13::class)

//    Class creation: 13ms
//    Part 1: 1528 (0ms)
//    Part 2: 3896406 (149ms)
//    Total time: 162ms
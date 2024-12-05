package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y17D05(input: String) : Day {
    private val jumps = input.lines().map(String::toInt)

    private inline fun solve(increment: (Int) -> Int): Int {
        val offsets = jumps.toIntArray()
        var i = 0
        var steps = 0
        while (i < offsets.size) {
            steps++
            i += offsets[i].also { offsets[i] += increment(offsets[i]) }
        }
        return steps
    }

    override fun part1(): Int = solve { 1 }

    override fun part2(): Int = solve { if (it >= 3) -1 else 1 }
}

fun main() = Day.runDay(Y17D05::class)

//    Class creation: 22ms
//    Part 1: 373160 (29ms)
//    Part 2: 26395586 (275ms)
//    Total time: 327ms

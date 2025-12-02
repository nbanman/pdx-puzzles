package org.gristle.pdxpuzzles.advent.y2025

import org.gristle.pdxpuzzles.advent.utilities.Day
import kotlin.math.abs

class Y25D01(input: String) : Day {
    val turns = input.lineSequence()
        .map { line -> line.drop(1).toInt() * if (line[0] == 'L') -1 else 1 }
        .toList()

    override fun part1() = turns
        .runningFold(50) { acc, i -> (acc + i).mod(100) }
        .count { it == 0 }

    override fun part2(): Int {
        var dial = 50
        var clicks = 0
        for (n in turns) {
            dial += n
            if (dial <= 0 && n != dial) {
                clicks++
            }
            clicks += abs(dial) / 100
            dial = dial.mod(100)
        }
        return clicks
    }
}

fun main() = Day.runDay(Y25D01::class)

//    Class creation: 7ms
//    Part 1: 1102 (3ms)
//    Part 2: 6175 (3ms)
//    Total time: 13ms
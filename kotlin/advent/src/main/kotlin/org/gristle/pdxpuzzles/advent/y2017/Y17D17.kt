package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y17D17(input: String) : Day {
    private val inputNum = input.toInt()

    override fun part1(): Int {
        val buffer = mutableListOf(0)
        var currPos = 0
        for (n in 1..2017) {
            currPos = (currPos + inputNum) % buffer.size + 1
            buffer.add(currPos, n)
        }
        return buffer[currPos + 1]
    }

    override fun part2(): Int {
        var currPos = 0
        var result = 0
        var n = 0
        val limit = 50_000_000
        while (n < limit) {
            if (currPos == 1) result = n
            val fits = (n - currPos) / inputNum
            n += fits + 1
            currPos = (currPos + (fits + 1) * (inputNum + 1) - 1) % n + 1
        }
        return result
    }
}

fun main() = Day.runDay(Y17D17::class)

//    Class creation: 3ms
//    Part 1: 1547 (1ms)
//    Part 2: 31154878 (0ms)
//    Total time: 5ms
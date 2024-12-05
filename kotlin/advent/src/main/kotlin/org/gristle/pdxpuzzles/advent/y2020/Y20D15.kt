package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y20D15(input: String) : Day {

    val start = input.split(',').map(String::toInt)

    private fun lastNumberSpoken(iterations: Int): Int {

        val bounds = iterations / 10
        
        val low = IntArray(bounds)
        val high: MutableMap<Int, Int> = HashMap(bounds / 2)
        
        for ((turn, n) in start.withIndex()) {
            low[n] = turn + 1
        }
        
        var current = 0
        for (turn in start.size + 1 until iterations) {
            if (current < bounds) {
                val prev = low[current]
                low[current] = turn
                current = if (prev == 0) 0 else turn - prev
            } else {
                current = high.put(current, turn)
                    ?.let { prev -> turn - prev }
                    ?: 0
            }
        }
        return current
    }

    override fun part1() = lastNumberSpoken(2020)

    override fun part2() = lastNumberSpoken(30_000_000)
}

fun main() = Day.runDay(Y20D15::class)

//    Class creation: 3ms
//    Part 1: 929 (0ms)
//    Part 2: 16671510 (878ms)
//    Total time: 882ms

package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getIntList

class Y15D25(input: String) : Day {

    private val row: Int
    private val col: Int

    init {
        input.getIntList().let { (a, b) ->
            row = a
            col = b
        }
    }

    private fun getPlace(): Int {
        val extra = if (col == 1) {
            0
        } else {
            (row until row + col).reduce(Int::plus) - row
        }
        return (1 until row).reduce(Int::plus) + 1 + extra
    }

    override fun part1() = (2..getPlace()).fold(20151125L) { acc, _ -> (acc * 252533) % 33554393 }
    override fun part2() = true
}

fun main() = Day.runDay(Y15D25::class)

//    Class creation: 16ms
//    Part 1: 8997277 (167ms)
//    Total time: 183ms

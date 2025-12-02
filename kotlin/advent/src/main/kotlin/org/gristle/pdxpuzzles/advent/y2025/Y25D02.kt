package org.gristle.pdxpuzzles.advent.y2025

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.pow
import org.gristle.pdxpuzzles.utilities.parsing.getLongs

class Y25D02(input: String) : Day {

    private val ids = input.getLongs().chunked(2).toList()

    private fun getNextInvalid1(n: Long): Long {
        val digits = getDigits(n)
        return if (digits and 1 == 1) {
            nextIfOdd(digits)
        } else {
            getNextByPortion(n, 2, digits)
        }
    }

    private fun getDigits(n: Long): Int {
        var n = n
        var count = 0
        while (n > 0) {
            n /= 10
            count++
        }
        return count
    }

    private fun nextIfOdd(digits: Int): Long = when (digits) {
        1 -> 11
        3 -> 1010
        5 -> 100100
        7 -> 10001000
        9 -> 1000010000
        else -> error("$digits is invalid! Must be odd between 1-9")
    }

    private fun getNextByPortion(n: Long, portion: Int, digits: Int): Long {
        val top = n / (10L.pow((digits / portion * (portion - 1)).toLong()))
        val candidate = (1 until portion).fold(top) { acc, _ ->
            acc * 10L.pow((digits / portion).toLong()) + top
        }
        return if (candidate >= n) {
            candidate
        } else {
            (1 until portion).fold(top + 1) { acc, _ ->
                acc * 10L.pow((digits / portion).toLong()) + top + 1
            }
        }
    }

    private fun getNextInvalid2(n: Long): Long {
        val digits = getDigits(n)
        if (digits == 1) {
            return 11
        }
        val portions = when (digits) {
            2 -> listOf(2)
            3 -> listOf(3)
            4 -> listOf(2)
            5 -> listOf(5)
            6 -> listOf(2, 3)
            7 -> listOf(7)
            8 -> listOf(2)
            9 -> listOf(3)
            10 -> listOf(2, 5)
            else -> error("$digits is invalid!")
        }

        return portions.minOf { portion -> getNextByPortion(n, portion, digits) }
    }

    private fun solve(getNextInvalid: (n: Long) -> Long): Long {
        var invalidIds = 0L
        for ((lo, hi) in ids) {
            var n = lo
            while (n <= hi) {
                val nextInvalid = getNextInvalid(n)
                if (nextInvalid <= hi) {
                    invalidIds += nextInvalid
                }
                n = nextInvalid + 1
            }
        }
        return invalidIds
    }

    override fun part1() = solve(::getNextInvalid1)

    override fun part2() = solve(::getNextInvalid2)
}

fun main() = Day.runDay(Y25D02::class)

//    Class creation: 2ms
//    Part 1: 28846518423 (3ms)
//    Part 2: 31578210022 (3ms)
//    Total time: 9ms
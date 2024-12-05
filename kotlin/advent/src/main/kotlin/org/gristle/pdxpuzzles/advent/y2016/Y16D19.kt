package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.pow
import kotlin.math.log10

class Y16D19(input: String) : Day {
    private val elves = input.toInt()

    override fun part1(): Int {
        val exponent = (log10(elves.toDouble()) / log10(2.0)).toInt()
        return (elves - (2.pow(exponent).toInt())) * 2 + 1
    }

    override fun part2(): Int {
        val exponent = (log10(elves.toDouble()) / log10(3.0)).toInt()
        val lastUp = 3.pow(exponent).toInt()
        val diff = elves - 3.pow(exponent).toInt()
        val ones = minOf(diff, lastUp)
        val twos = maxOf(diff - ones, 0)
        return ones + twos * 2
    }
}

// Class creation: 5ms
// Part 1: 1816277 (0ms)
// Part 2: 1410967 (0ms)
fun main() = Day.runDay(Y16D19::class) 
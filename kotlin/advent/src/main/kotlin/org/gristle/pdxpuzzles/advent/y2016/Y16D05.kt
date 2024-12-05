package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.Md5
import org.gristle.pdxpuzzles.utilities.math.Md5.toHex

class Y16D05(input: String) : Day {
    private val zero = 0.toByte()
    private val md5Sequence = generateSequence(0) { it + 1 }
        .map { Md5.getDigest(input + it) }
        .filter { it[0] == zero && it[1] == zero && ((it[2].toInt() shr 4) == 0) }
        .map { it.toHex() }

    override fun part1() = md5Sequence
        .map { it[5] }
        .take(8)
        .joinToString("")

    override fun part2() = md5Sequence
        .filter { it[5] in '0'..'7' }
        .distinctBy { it[5] }
        .take(8)
        .sortedBy { it[5] }
        .map { it[6] }
        .joinToString("")
}

fun main() = Day.runDay(Y16D05::class)

//    [2016 Day 5]
//    Class creation: 16ms
//    Part 1: 4543c154 (2525ms)
//    Part 2: 1050cbbd (6370ms)
//    Total time: 8912ms
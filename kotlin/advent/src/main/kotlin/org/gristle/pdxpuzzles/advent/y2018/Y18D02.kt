package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.combinatorics.getPairSequence

class Y18D02(input: String) : Day {

    private val boxIds = input.lines()

    override fun part1(): Int {
        val frequencies = boxIds.map { boxId -> boxId.groupingBy { it }.eachCount().values }
        return frequencies.count { it.contains(2) } * frequencies.count { it.contains(3) }
    }

    override fun part2(): String = boxIds
        .getPairSequence()
        .map { (a, b) -> a zip b }
        .first { pairs -> pairs.count { (a, b) -> a != b } == 1 }
        .filter { (a, b) -> a == b }
        .map { it.first }
        .joinToString("")
}

fun main() = Day.runDay(Y18D02::class)

//    Class creation: 9ms
//    Part 1: 7688 (8ms)
//    Part 2: lsrivmotzbdxpkxnaqmuwcchj (14ms)
//    Total time: 31ms
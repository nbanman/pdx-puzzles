package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y18D01(input: String) : Day {

    private val changes = input.lines().map(String::toInt)

    override fun part1() = changes.sum()

    override fun part2(): Int {
        val record = mutableSetOf<Int>()
        return generateSequence(0) { (it + 1) % changes.size }
            .map { changes[it] }
            .runningReduce(Int::plus)
            .first { !record.add(it) }
    }
}

fun main() = Day.runDay(Y18D01::class)

//    Class creation: 22ms
//    Part 1: 433 (0ms)
//    Part 2: 256 (24ms)
//    Total time: 46ms
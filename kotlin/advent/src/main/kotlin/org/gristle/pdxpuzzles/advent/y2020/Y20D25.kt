package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y20D25(input: String) : Day {

    private val divisor = 20201227

    private val cardKey: Long
    private val doorKey: Long

    init {
        val (cardKey, doorKey) = input.lines().map(String::toLong)
        this.cardKey = cardKey
        this.doorKey = doorKey
    }

    override fun part1(): Long {
        val loopSize = generateSequence(1L) { value -> (value * 7) % divisor }
            .indexOfFirst { value -> value == cardKey }

        return generateSequence(doorKey % divisor) { (it * doorKey) % divisor }
            .take(loopSize)
            .last()
    }

    override fun part2() = "Merry Xmas!!!"
}

fun main() = Day.runDay(Y20D25::class)

//    Class creation: 18ms
//    Part 1: 296776 (50ms)
//    Total time: 68ms

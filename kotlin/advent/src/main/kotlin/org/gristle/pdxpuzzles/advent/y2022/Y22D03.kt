package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y22D03(input: String) : Day {
    private val rucksacks = input.lineSequence()

    private fun Char.priority() = if (isLowerCase()) this - 'a' + 1 else this - 'A' + 27

    // for each character, obtain priority n and then place a '1' bit n to the left of 1.
    private fun CharSequence.toBitSet() = fold(0L) { acc, c -> acc or (1L shl c.priority()) }

    override fun part1() = rucksacks.sumOf { sack ->
        sack
            // splits each sack into two halves and turns each half into a bitset
            .chunked(sack.length / 2) { it.toBitSet() }
            // find common bit and convert to priority
            .let { (a, b) -> (a and b).countTrailingZeroBits() }
    }

    override fun part2() = rucksacks
        .map { it.toBitSet() } // turn each string into a bitset
        .chunked(3) // chunk in groups of three
        .sumOf { it.reduce(Long::and).countTrailingZeroBits() } // find common bit and convert to priority
}

fun main() = Day.runDay(Y22D03::class)

//    Class creation: 18ms
//    Part 1: 7428 (8ms)
//    Part 2: 2650 (4ms)
//    Total time: 31ms
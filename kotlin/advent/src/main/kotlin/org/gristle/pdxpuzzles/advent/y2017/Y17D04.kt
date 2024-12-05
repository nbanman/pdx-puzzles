package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y17D04(input: String) : Day {

    private val passphrases = input.lines().map { it.split(' ') }

    private fun <T> List<List<T>>.countUnique() = count { it.size == it.distinct().size }

    override fun part1() = passphrases.countUnique()

    override fun part2() = passphrases
        .map { phrase -> phrase.map { word -> word.groupingBy { it }.eachCount() } } // convert words into letter distributions
        .countUnique()
}

fun main() = Day.runDay(Y17D04::class)

//    Class creation: 24ms
//    Part 1: 455 (2ms)
//    Part 2: 186 (13ms)
//    Total time: 40ms

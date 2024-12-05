package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.toInt

class Y21D03(input: String) : Day {

    private val codes = input
        .lineSequence()
        .map { line -> BooleanArray(line.length) { i -> line[i] == '1' } }
        .toList()

    private fun findRate(codes: List<BooleanArray>, target: Boolean): Int =
        BooleanArray(codes.first().size) { i ->
            codes.count { it[i] == target } * 2 >= codes.size
        }.toInt()
    
    private fun findRating(codes: List<BooleanArray>, predicate: (Int) -> Boolean): Int {
        val codeFilter = generateSequence(codes to 0) { (codes, i) ->
            val filteredCodes = codes.filter { code ->
                val criteria = predicate((codes.count { it[i] } * 2).compareTo(codes.size))
                code[i] == criteria
            }
            filteredCodes to i + 1
        }
        return codeFilter
            .first { (codes, _) -> codes.size == 1 }
            .let { (codes, _) -> codes.first() }
            .toInt()
    }

    override fun part1(): Int {
        val gamma: Int = findRate(codes, true)
        val epsilon: Int = findRate(codes, false)
        return gamma * epsilon
    }

    override fun part2(): Int {
        val o2Gen = findRating(codes) { it >= 0 }
        val co2Scrubber = findRating(codes) { it < 0 }
        return o2Gen * co2Scrubber
    }
}

fun main() = Day.runDay(Y21D03::class)

//    Class creation: 16ms
//    Part 1: 3969000 (18ms)
//    Part 2: 4267809 (1ms)
//    Total time: 35ms
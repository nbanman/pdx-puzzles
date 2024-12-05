package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y16D18(input: String) : Day {

    // Convert input to BooleanArray
    private val start = BooleanArray(input.length) { i -> input[i] == '.' }

    // Takes a row and returns the next one using the rules in the problem. 
    private fun BooleanArray.nextRow() = BooleanArray(size) { i ->
        when (i) {
            0 -> this[1] // no previous so answer is true if next is true
            lastIndex -> this[i - 1] // no next so answer is true if previous is true
            else -> this[i - 1] == this[i + 1] // base case: true if previous and next are the same
        }
    }

    // Creates a sequence of the rows and sums the number of true values in each row.
    private fun solve(rows: Int) = generateSequence(start) { it.nextRow() }
        .take(rows)
        .sumOf { row -> row.count { it } }

    override fun part1() = solve(40)

    override fun part2() = solve(400_000)
}

fun main() = Day.runDay(Y16D18::class)

//    Class creation: 2ms
//    Part 1: 1987 (2ms)
//    Part 2: 19984714 (283ms)
//    Total time: 288ms

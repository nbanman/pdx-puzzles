package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.quadraticFormula
import org.gristle.pdxpuzzles.utilities.parsing.getLongList
import kotlin.math.ceil
import kotlin.math.floor

class Y23D6(private val input: String) : Day {

    override fun part1() = solve { it.getLongList() }

    override fun part2() = solve { listOf(it.filter(Char::isDigit).toLong()) }

    private inline fun solve(parseLine: (String) -> List<Long>): Long {
        
        // parses input by parsing the digits in each input line according to the rules for each part, then
        // transposing the list of lists so that the vertical columns are grouped together.
        val races: List<Pair<Long, Long>> = input
            .lines()
            .map(parseLine)
            .let { (times, distances) -> times zip distances }
        
        return races.map(::waysToWin).reduce(Long::times)
    }

    private fun waysToWin(race: Pair<Long, Long>): Long {
        val (time, distance) = race
        
        // equation can be expressed as quadratic equation
        // D = distance, T = time, t = time pushing button
        // D = t(T - t) => (-1)t^2 + (T)t + (-D) = 0
        val (root1, root2) = quadraticFormula(-1, time, -distance)
        return (ceil(root2 - 1.0) - floor(root1 + 1.0)).toLong() + 1L
    }
}

fun main() = Day.runDay(Y23D6::class)

//    Class creation: 2ms
//    Part 1: 2374848 (2ms)
//    Part 2: 39132886 (0ms)
//    Total time: 5ms

@Suppress("unused")
private val sampleInput = listOf(
    """Time:      7  15   30
Distance:  9  40  200
""" to "288",
)
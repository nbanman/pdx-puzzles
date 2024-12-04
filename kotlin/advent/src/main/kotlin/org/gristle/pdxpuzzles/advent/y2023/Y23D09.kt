package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y23D9(input: String) : Day {

    private val patterns = input.lines().map { line -> line.split(' ').map(String::toInt) }

    private fun findNext(pattern: List<Int>) = generateSequence(pattern) { it.zipWithNext { a, b -> b - a } }
        .takeWhile { next -> next.any { it != 0 } }
        .sumOf { it.last() }
    
    override fun part1() = patterns.sumOf { pattern -> findNext(pattern) }

    override fun part2() = patterns.sumOf { pattern -> findNext(pattern.reversed()) }
}

fun main() = Day.runDay(Y23D9::class)

//    Class creation: 17ms
//    Part 1: 1974913025 (6ms)
//    Part 2: 884 (3ms)
//    Total time: 27ms

@Suppress("unused")
private val sampleInput = listOf(
    """0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45""",
)
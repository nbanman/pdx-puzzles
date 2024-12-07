package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.gvs

class Y24D03(val input: String) : Day {

    override fun part1(): Long = input
        .gvs(Regex("""mul\((\d+),(\d+)\)""")) { it.toLong() }
        .sumOf { (a, b) -> a * b }

    override fun part2() = Regex("""(?s)don't\(\)(?:[^d]++|d(?!o\(\)))*+(?:do\(\)|${'$'})|mul\((\d{1,3}),(\d{1,3})\)""")
        .findAll(input)
        .sumOf { mr -> (mr.groupValues[1].toLongOrNull() ?: 0L) * (mr.groupValues[2].toLongOrNull() ?: 0L) }
}

fun main() = Day.runDay(Y24D03::class)

@Suppress("unused")
private val test = listOf(
    """xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))""",
    """xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))""",
    """xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))""",
)

//    Class creation: 1ms
//    Part 1: 191183308 (14ms)
//    Part 2: 92082041 (9ms)
//    Total time: 25ms
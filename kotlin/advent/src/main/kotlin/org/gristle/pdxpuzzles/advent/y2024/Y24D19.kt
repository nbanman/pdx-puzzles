package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit

class Y24D19(input: String) : Day {
    private val cache = mutableMapOf("" to 1L)
    private val stanzas = input.blankSplit().map { it.split(", ", "\n") }
    private fun String.countTheWays(): Long = cache.getOrPut(this) {
        stanzas[0].sumOf { towel -> if (!startsWith(towel)) 0 else drop(towel.length).countTheWays() }
    }
    override fun part1(): Int = stanzas[1].count { design -> design.countTheWays() > 0 }
    override fun part2(): Long = stanzas[1].sumOf { design -> design.countTheWays() }
}

fun main() = Day.runDay(Y24D19::class)

//    Class creation: 5ms
//    Part 1: 238 (62ms)
//    Part 2: 635018909726691 (2ms)
//    Total time: 70ms

@Suppress("unused")
private val test = listOf("""r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb""")
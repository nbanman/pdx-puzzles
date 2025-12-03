package org.gristle.pdxpuzzles.advent.y2025

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y25D03(private val input: String) : Day {
    private fun solve(digits: Int): Long = input
        .lineSequence()
        .sumOf { line ->
            var left = 0
            (0 until digits).reversed().fold(0L) { acc, i ->
                var highest = 0
                var highPos = 0
                for ((idx, c) in line.substring(left, line.length - i).withIndex()) {
                    val n = c.digitToInt()
                    if (n > highest) {
                        highest = n
                        highPos = idx
                    }
                    if (n == 9) break
                }
                left += highPos + 1
                acc * 10 + highest
            }
        }
    override fun part1() = solve(2)
    override fun part2() = solve(12)
}

fun main() = Day.benchmarkDay(Y25D03::class)

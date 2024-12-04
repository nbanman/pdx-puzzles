package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getIntList

class Y24D2(input: String) : Day {
    private val levels: List<List<Int>> = input.lines().map { it.getIntList() }

    private fun List<Int>.isSafe(): Boolean {
        val rng = if (first() < last()) -3..-1 else 1..3
        return asSequence().zipWithNext().all { (a, b) -> a - b in rng }
    }

    private fun List<Int>.isSomewhatSafe(): Boolean {
        val diffs = zipWithNext { a, b -> b - a }.count { it > 0 }
        val rng = when (diffs){
            0, 1 -> 1..3
            lastIndex, lastIndex - 1 -> -3..-1
            else -> return false
        }
        var removed = false
        var i = 0

        while(i < lastIndex) {
            if (get(i) - get(i + 1) !in rng) {
                if (removed) return false
                removed = true
                // i + 1 could be the culprit
                if (i != 0) {
                    if (get(i - 1) - get(i + 1) !in rng) {
                        // i is valid
                        if (i != lastIndex - 1 && get(i) - get(i + 2) !in rng) return false
                        i++
                    }
                } else if (get(0) - get(2) in rng) {
                    i++
                }
            }
            i++
        }
        return true
    }

    override fun part1() = levels.count { level -> level.isSafe() }
    override fun part2() = levels.count { level -> level.isSomewhatSafe() }
}
fun main() = Day.runDay(Y24D2::class)

@Suppress("unused")
private const val test = """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""

//    Class creation: 20ms
//    Part 1: 591 (5ms)
//    Part 2: 621 (9ms)
//    Total time: 34ms
package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import java.util.ArrayDeque

object Y25D13 : Day {
    private fun solve(input: String, totalTurns: Long): Int {
        val lock = ArrayDeque<IntRange>(500)
        lock.addLast(1..1)
        var start = 0
        var total = 1
        var forward = true

        for (rng in ranges(input)) {
            total += rng.last - rng.first + 1
            if (forward) {
                lock.addLast(rng)
            } else {
                start++
                lock.addFirst(rng)
            }
            forward = !forward
        }

        val totalTurns = (totalTurns + 1) % total
        var turns = 0

        for (i in start..start + lock.size) {
            val idx = i % lock.size
            val rng = lock.elementAt(idx % lock.size)
            turns += rng.last - rng.first + 1
            if (turns >= totalTurns) {
                val diff = (turns - totalTurns).toInt()
                return if (idx >= start) {
                    rng.last - diff
                } else {
                    rng.first + diff
                }
            }
        }
        error("Unreachable!")
    }

    private fun ranges(input: String): Sequence<IntRange> = input
        .lineSequence()
        .map { line ->
            val rng = line.split('-').map { it.toInt() }
            val lo = rng[0]
            val hi = rng.getOrElse(1) { lo }
            lo..hi
        }

    override fun part1(input: String): Int = solve(input, 2025)
    override fun part2(input: String): Int = solve(input, 20_252_025)
    override fun part3(input: String): Int = solve(input, 202_520_252_025)
}

fun main() = Day.runDay(Y25D13::class)

//    Quest 1: 353 (1ms)
//    Quest 2: 7613 (4ms)
//    Quest 3: 217823 (3ms)
//    Total time: 10ms
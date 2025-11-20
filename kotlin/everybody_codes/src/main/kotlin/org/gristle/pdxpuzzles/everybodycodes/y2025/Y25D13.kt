package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.isEven
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import java.util.ArrayDeque

object Y25D13 : Day {
    private fun solve(ranges: Sequence<IntRange>, totalTurns: Long): Int {
        val lock = ArrayDeque<Pair<IntRange, Boolean>>(500)
        lock.addLast(1..1 to true)
        var start = 0
        var total = 1

        for ((idx, rng) in ranges.withIndex()) {
            total += rng.last - rng.first + 1
            if (idx.isEven()) {
                lock.addLast(rng to true)
            } else {
                start++
                lock.addFirst(rng to false)
            }
        }

        val totalTurns = (totalTurns + 1) % total
        var turns = 0

        for (i in start..start + lock.size) {
            val (rng, isForward) = lock.elementAt(i % lock.size)
            turns += rng.last - rng.first + 1
            if (turns >= totalTurns) {
                val diff = (turns - totalTurns).toInt()
                return if (isForward) {
                    rng.last - diff
                } else {
                    rng.first + diff
                }
            }
        }
        throw IllegalStateException("Unreachable!")
    }

    override fun part1(input: String): Int {
        val ranges = input.getInts().map { it..it }
        return solve(ranges, 2025)
    }
    override fun part2(input: String): Int {
        val ranges = input.getInts().chunked(2).map { (a, b) -> a..b }
        return solve(ranges, 20_252_025)
    }
    override fun part3(input: String): Int {
        val ranges = input.getInts().chunked(2).map { (a, b) -> a..b }
        return solve(ranges, 202_520_252_025)
    }
}

fun main() = Day.runDay(Y25D13::class)

//    Quest 1: 353 (1ms)
//    Quest 2: 7613 (4ms)
//    Quest 3: 217823 (3ms)
//    Total time: 10ms
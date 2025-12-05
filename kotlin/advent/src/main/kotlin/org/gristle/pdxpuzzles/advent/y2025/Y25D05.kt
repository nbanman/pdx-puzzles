package org.gristle.pdxpuzzles.advent.y2025

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit
import org.gristle.pdxpuzzles.utilities.parsing.getLongs
import kotlin.math.max

class Y25D05(input: String) : Day {
    private val ranges: List<LongRange>
    private val ids: List<Long>

    init {
        val (rangeStr, idStr) = input.blankSplit()
        ranges = rangeStr.getLongs()
            .chunked(2)
            .map { (a, b) -> a..b }
            .toMutableList()

        ranges.sortBy { it.first }
        do {
            var changed = false
            for (i in ranges.lastIndex  downTo 1) {
                val a = ranges[i - 1]
                val b = ranges[i]
                if(a.last >= b.first) {
                    ranges[i - 1] = a.first..max(a.last, b.last)
                    ranges.removeAt(i)
                    changed = true
                }
            }
        } while (changed)

        ids = idStr.getLongs().sorted().toList()
    }

    override fun part1() = ranges.sumOf { rng ->
        val below = ids.binarySearch(rng.first)
            .let { if (it < 0) -it - 1 else it }
        val within = ids.binarySearch(rng.last)
            .let { if (it < 0) -it - 1 else it }
        within - below
    }

    override fun part2(): Long {
        return ranges.sumOf { it.last - it.first + 1 }
    }
}

fun main() = Day.runDay(Y25D05::class)

//    Class creation: 7ms
//    Part 1: 652 (1ms)
//    Part 2: 341753674214273 (2ms)
//    Total time: 11ms
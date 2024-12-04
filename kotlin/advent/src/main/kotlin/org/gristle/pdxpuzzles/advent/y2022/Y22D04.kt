package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import org.gristle.pdxpuzzles.utilities.objects.containsAll
import org.gristle.pdxpuzzles.utilities.objects.overlaps

class Y22D04(input: String) : Day {

    private val ranges: Sequence<Pair<IntRange, IntRange>> = input
        .getInts()
        .chunked(4) { (low1, high1, low2, high2) -> low1..high1 to low2..high2 }

    override fun part1() = ranges.count { (left, right) ->
        left.containsAll(right) || right.containsAll(left)
    }

    override fun part2() = ranges.count { (left, right) ->
        left.overlaps(right)
    }
}

fun main() = Day.runDay(Y22D04::class)

//    Class creation: 3ms
//    Part 1: 605 (12ms)
//    Part 2: 914 (3ms)
//    Total time: 19ms
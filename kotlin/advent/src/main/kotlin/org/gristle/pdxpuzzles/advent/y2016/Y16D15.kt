package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getLongs

private typealias Disc = Pair<Long, Long>

class Y16D15(input: String) : Day {

    // Parse input into Discs
    private val discs: Sequence<Pair<Long, Long>> = input
        .getLongs()
        .chunked(4) { (startTime, positions, _, positionAtT0) -> positions to -startTime - positionAtT0 }

    // Uses a sieve version of CRT
    private fun solve(discs: Sequence<Disc>): Long {
        val (_, answer) = discs.fold(1L to 0L) { (interval, seconds), (positions, offset) ->

            // take the current number of seconds that works for the previous discs, and keep adding the current
            // interval until it works for the next disc.
            val nextSeconds = generateSequence(seconds) { it + interval }
                .first { (it - offset).mod(positions) == 0L }

            // values are coprime; next interval is current interval multiplied by the number of positions in next disc
            val nextInterval = interval * positions

            nextInterval to nextSeconds
        }
        return answer
    }

    override fun part1() = solve(discs)

    override fun part2() = solve(discs + (11L to -7L))
}

fun main() = Day.runDay(Y16D15::class)

//    Class creation: 18ms
//    Part 1: 122318 (3ms)
//    Part 2: 3208583 (1ms)
//    Total time: 23ms
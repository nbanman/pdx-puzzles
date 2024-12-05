package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y15D14(input: String) : Day {

    companion object {
        private const val SECONDS = 2503
    }

    data class Reindeer(val speed: Int, val duration: Int, val rest: Int) {
        private val interval = duration + rest
        fun distanceRaced(seconds: Int): Int {
            val wholeIntervals = seconds / interval
            val remainder = seconds % interval
            return wholeIntervals * (speed * duration) + minOf(remainder, duration) * speed
        }
    }

    private val racers = input
        .getInts()
        .chunked(3) { (speed, duration, rest) -> Reindeer(speed, duration, rest) }
        .toList()

    override fun part1() = racers.maxOf { it.distanceRaced(SECONDS) }

    override fun part2(): Int {
        val leaderboard = IntArray(racers.size)
        for (t in 1..SECONDS) {
            val distances = racers.map { it.distanceRaced(t) }
            val maxDistance = distances.max()
            distances.forEachIndexed { racer, distance -> if (distance == maxDistance) leaderboard[racer]++ }
        }
        return leaderboard.max()
    }
}

fun main() = Day.runDay(Y15D14::class)

//    Class creation: 14ms
//    Part 1: 2640 (0ms)
//    Part 2: 1102 (8ms)
//    Total time: 23ms
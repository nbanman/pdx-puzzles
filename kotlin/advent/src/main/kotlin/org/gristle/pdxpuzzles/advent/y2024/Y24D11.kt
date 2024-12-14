package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getLongs

class Y24D11(private val stones: String) : Day {
    data class State(val stone: Long, val onBlink: Int, val blinks: Int)
    private fun solve(blinks: Int): Long {
        val cache = mutableMapOf<State, Long>()
        return stones.getLongs().sumOf { stone ->
            val initialState = State(stone, 0, blinks)
            getStones(initialState, cache)
        }
    }

    private fun Long.blink(): List<Long> {
        val str = toString()
        val halfLength = str.length / 2
        return when {
            this == 0L -> listOf(1L)
            str.length and 1 == 0 -> listOf(
                str.take(halfLength).toLong(),
                str.takeLast(halfLength).toLong()
            )
            else -> listOf(this * 2024)
        }
    }

    private fun getStones(state: State, cache: MutableMap<State, Long>): Long = cache.getOrPut(state) {
        val (stone, onBlink, blinks) = state
        if (onBlink == blinks) 1 else {
            stone.blink().sumOf { subStone ->
                val subState = State(subStone, onBlink + 1, blinks)
                getStones(subState, cache)
            }
        }
    }

    override fun part1(): Long = solve(25)
    override fun part2(): Long = solve(75)
}
fun main() = Day.runDay(Y24D11::class)

//    Class creation: 1ms
//    Part 1: 231278 (9ms)
//    Part 2: 274229228071551 (82ms)
//    Total time: 94ms
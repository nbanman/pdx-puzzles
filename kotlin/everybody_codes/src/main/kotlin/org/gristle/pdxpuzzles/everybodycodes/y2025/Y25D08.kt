package org.gristle.pdxpuzzles.everybodycodes.y2025

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.runBlocking
import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.combinatorics.getPairs
import org.gristle.pdxpuzzles.utilities.iteration.minMax
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import kotlin.math.abs

object Y25D08 : Day {
    private fun parse(input: String): Sequence<Int> = input.getInts().map { it - 1 }
    private fun crosses(input: String): Sequence<Pair<Int, Int>> = parse(input)
        .windowed(2)
        .map { (a, b) -> minMax(a, b) }

    override fun part1(input: String): Int = parse(input)
        .windowed(2)
        .count { (a, b) -> abs(a - b) == 16 }

    override fun part2(input: String): Int {
        val completed = mutableListOf<Pair<Int, Int>>()
        var knots = 0

        for ((a, b) in crosses(input)) {
            for((ca, cb) in completed) {
                val semicircle = ca + 1 until cb
                val rng = ca..cb
                if (a in semicircle) {
                    if (b !in rng) {
                        knots++
                    }
                } else if (b in semicircle) {
                    if (a !in rng) {
                        knots++
                    }
                }
            }
            completed.add(a to b)
        }
        return knots
    }

    override fun part3(input: String): Int = runBlocking {
        val crosses = crosses(input).toList()
        val nails = 256
        val combinations = List(nails) { it }.getPairs()
        combinations
            .map { (a, b) ->
                async(context = Dispatchers.Default) {
                    var cuts = 0
                    for ((ca, cb) in crosses) {
                        if (a == ca && b == cb) {
                            cuts++
                            continue
                        }
                        val semicircle = ca + 1 until cb
                        val rng = ca..cb
                        if (a in semicircle) {
                            if (b !in rng) {
                                cuts++
                            }
                        } else if (b in semicircle) {
                            if (a !in rng) {
                                cuts++
                            }
                        }
                    }
                    cuts
                }
            }.awaitAll()
            .max()
    }
}

fun main() = Day.runDay(Y25D08::class)

//    Quest 1: 58 (1ms)
//    Quest 2: 2924358 (61ms)
//    Quest 3: 2792 (273ms)
//    Total time: 337ms
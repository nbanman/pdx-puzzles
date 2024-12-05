package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit
import kotlin.math.min

class Y20D22(input: String) : Day {

    private val p1: List<Int>
    private val p2: List<Int>

    init {
        val (p1, p2) = input
            .blankSplit()
            .map { it.lines().mapNotNull(String::toIntOrNull) }
        this.p1 = p1
        this.p2 = p2
    }

    private fun List<Int>.score() = foldIndexed(0) { index, acc, i -> acc + (size - index) * i }

    override fun part1(): Int {
        tailrec fun play(p1: List<Int>, p2: List<Int>): Int {
            return when {
                p1.isEmpty() -> p2.score()
                p2.isEmpty() -> p1.score()
                else -> {
                    val short = min(p1.size, p2.size)
                    val (p1Wins, p2Wins) = (p1 zip p2).partition { ab -> ab.first > ab.second }
                    play(
                        p1.drop(short) + p1Wins.flatMap { it.toList().sortedDescending() },
                        p2.drop(short) + p2Wins.flatMap { it.toList().sortedDescending() }
                    )
                }
            }
        }

        return play(p1, p2)
    }

    override fun part2(): Int {

        fun play(p1: List<Int>, p2: List<Int>): Pair<String, List<Int>> {
            val cache = mutableSetOf<Pair<List<Int>, List<Int>>>()
            tailrec fun play2(p1: List<Int>, p2: List<Int>): Pair<String, List<Int>> {

                if (cache.contains(p1 to p2)) {
                    return "p1" to p1
                } else {
                    cache.add(p1 to p2)
                }
                val p1New: List<Int>
                val p2New: List<Int>
                when {
                    p1.isEmpty() -> return "p2" to p2
                    p2.isEmpty() -> return "p1" to p1
                    else -> {
                        val p1Poll = p1.first()
                        val p2Poll = p2.first()
                        if (p1.size - 1 < p1Poll || p2.size - 1 < p2Poll) {
                            if (p1Poll > p2Poll) {
                                p1New = p1.drop(1) + listOf(p1Poll, p2Poll)
                                p2New = p2.drop(1)
                            } else {
                                p1New = p1.drop(1)
                                p2New = p2.drop(1) + listOf(p2Poll, p1Poll)
                            }
                        } else {
                            val p1Mini = p1.drop(1).take(p1Poll)
                            val p2Mini = p2.drop(1).take(p2Poll)
                            val subGame = play(p1Mini, p2Mini)
                            if (subGame.first == "p1") {
                                p1New = p1.drop(1) + listOf(p1Poll, p2Poll)
                                p2New = p2.drop(1)
                            } else {
                                p1New = p1.drop(1)
                                p2New = p2.drop(1) + listOf(p2Poll, p1Poll)
                            }
                        }
                    }
                }
                return play2(p1New, p2New)
            }
            return play2(p1, p2)
        }
        return play(p1, p2).second.score()
    }
}

fun main() = Day.runDay(Y20D22::class)

//    Class creation: 16ms
//    Part 1: 32824 (2ms)
//    Part 2: 36515 (731ms)
//    Total time: 750ms
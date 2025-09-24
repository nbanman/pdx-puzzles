package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import kotlin.math.abs

class Y19D16(private val input: String) : Day {
    private val nos = input.map(Char::digitToInt)
    private val phases = 100

    override fun part1(): Int {
        val startPattern = listOf(0, 1, 0, -1)
        return (1..phases).fold(nos) { acc, _ ->
            List(acc.size) { index ->
                acc.foldIndexed(0) { index2, acc2, ii ->
                    val ai = ((index2 + 1) / (index + 1)) % 4
                    acc2 + ii * startPattern[ai]
                }.let { abs(it % 10) }
            }
        }.take(8)
        .reduce { acc, i -> acc * 10 + i }
    }

    override fun part2(): Int {
        val offset = input.take(7).toInt()
        val arraySize = nos.size * 10_000 - offset
        val offsetMod = offset % nos.size
        val nosi = IntArray(arraySize) { i -> nos[(offsetMod + i) % nos.size] }
        repeat(phases) {
            for (i in nosi.lastIndex - 1 downTo 0) {
                nosi[i] = (nosi[i] + nosi[i + 1]) % 10
            }
        }
        return nosi
            .take(8)
            .reduce { acc, i -> acc * 10 + i }
    }
}

fun main() = Day.benchmarkDay(Y19D16::class)

//    Class creation: 11ms
//    Part 1: 52611030 (206ms)
//    Part 2: 52541026 (174ms)
//    Total time: 391ms
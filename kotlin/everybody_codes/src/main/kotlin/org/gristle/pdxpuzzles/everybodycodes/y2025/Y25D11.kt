package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getLongList

object Y25D11 : Day {
    private fun flock(input: String): LongArray {
        val numberList = input.getLongList()
        val iter = numberList.iterator()
        return LongArray(numberList.size) { iter.next() }
    }

    private fun solve(flock: LongArray, roundLimit: Int?): Int {
        var changed = true
        var round = 0
        // phase 1
        while (changed) {
            round++
            changed = false
            for (i in 0 until flock.size - 1) {
                if (flock[i] > flock[i + 1]) {
                    flock[i]--
                    flock[i + 1]++
                    changed = true
                }
            }
        }
        // phase 2
        round -= 1
        changed = true
        while (changed) {
            changed = false
            for (i in 0 until flock.size - 1) {
                if (flock[i] < flock[i + 1]) {
                    flock[i]++
                    flock[i + 1]--
                    changed = true
                }
            }
            round++
            if (round == roundLimit) break
        }
        return round - 1
    }

    override fun part1(input: String): Long {
        val flock = flock(input)
        solve(flock, 10)
        return flock.foldIndexed(0L) { index, acc, v ->
            acc + (index.toLong() + 1) * v
        }
    }
    override fun part2(input: String): Int = solve(flock(input), null)
    override fun part3(input: String): Long {
        val flock = flock(input)
        val mean = flock.sum() / flock.size.toLong()
        return flock
            .filter { n -> n < mean }
            .sumOf { n -> mean - n }
    }
}

fun main() = Day.runDay(Y25D11::class)

//    Quest 1: 271 (0ms)
//    Quest 2: 3984738 (205ms)
//    Quest 3: 130353341887463 (0ms)
//    Total time: 206ms
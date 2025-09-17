package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day
import kotlin.math.max

class Y20D09(input: String) : Day {

    private val numbers = input.lines().map(String::toLong).toList()

    override fun part1(): Long {
        val preamble = 25
        // Prep cache
        val cache = mutableMapOf<Long, Int>()
        for (l in 0 until (preamble - 1)) {
            for (u in (l + 1) until preamble) {
                cache[numbers[l] + numbers[u]] = l
            }
        }
        // Try each subsequent number
        for (i in (preamble)..numbers.lastIndex) {
            val current = numbers[i]
            val indexOfSum = cache[current] ?: -1
            if (indexOfSum < i - preamble) {
                return current
            }
            for (l in (i - preamble + 1) until i) {
                val next = numbers[l] + current
                val existing = cache[next] ?: -1
                if (l > existing) {
                    cache[next] = l
                }
            }
        }
        return -1L
    }

    override fun part2(): Long {
        val weakness = part1()
        var l = 0
        var u = 1
        var sum = numbers[l]
        while (true) {
            sum += numbers[u]
            if (sum == weakness)
                return numbers.slice(l..u).min() + numbers.slice(l..u).max()
            if (sum > weakness) {
                sum -= numbers[l]
                l++
                if (sum == weakness)
                    return numbers.slice(l..u).min() + numbers.slice(l..u).max()
                while (sum > weakness) {
                    sum -= numbers[u]
                    u--
                    if (sum == weakness)
                        return numbers.slice(l..u).min() + numbers.slice(l..u).max()
                }

            }
            u++
        }
    }
}

fun main() = Day.runDay(Y20D09::class)

//    Class creation: 2ms
//    Part 1: 552655238 (5ms)
//    Part 2: 70672245 (5ms)
//    Total time: 13ms
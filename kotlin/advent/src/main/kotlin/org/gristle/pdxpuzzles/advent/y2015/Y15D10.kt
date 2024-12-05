package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y15D10(private val input: String) : Day {

    private fun lookAndSay(s: String) = buildString {
        var digit = s[0]
        var count = 1
        for (i in 1 until s.length) {
            if (s[i] == digit) {
                count++
            } else {
                append(count)
                append(digit)
                digit = s[i]
                count = 1
            }
        }
        append(count)
        append(digit)
    }

    private fun solve(n: Int) = generateSequence(input, ::lookAndSay)
        .take(n + 1)
        .last()
        .length

    override fun part1() = solve(40)

    override fun part2() = solve(50)
}

fun main() = Day.runDay(Y15D10::class)

//    Class creation: 16ms
//    Part 1: 492982 (60ms)
//    Part 2: 6989950 (410ms)
//    Total time: 487ms

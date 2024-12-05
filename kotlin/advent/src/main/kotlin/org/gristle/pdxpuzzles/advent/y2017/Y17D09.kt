package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y17D09(private val input: String) : Day {

    val solution: Pair<Int, Int> by lazy {
        var inGarbage = false
        var garbage = 0
        var depth = 0
        var score = 0
        var last = ' '
        input.forEach { c ->
            if (inGarbage) {
                if (c == '>' && last != '!') inGarbage = false
                if (!(c in "!>" || last == '!')) garbage++
                last = if (last == '!') ' ' else c
            } else {
                when (c) {
                    '<' -> inGarbage = true

                    '{' -> {
                        depth++
                        score += depth
                    }

                    '}' -> {
                        depth--
                    }
                }
            }
        }
        score to garbage
    }

    override fun part1() = solution.first

    override fun part2() = solution.second
}


fun main() = Day.runDay(Y17D09::class)

//    Class creation: 2ms
//    Part 1: 9251 (2ms)
//    Part 2: 4322 (0ms)
//    Total time: 5ms
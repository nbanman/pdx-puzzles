package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.adventOfCode.y2016.shared.Assembunny

class Y16D23(private val input: String) : Day {

    override fun part1(): Int {
        val registers = IntArray(4) { idx -> if (idx == 0) 7 else 0 }
        return Assembunny(registers)
            .runInstructions(input)['a']
    }

    override fun part2(): Int = (1..12).reduce(Int::times) + 94 * 82
}

fun main() = Day.runDay(Y16D23::class)

//    Class creation: 2ms
//    Part 1: 12748 (21ms)
//    Part 2: 479009308 (0ms)
//    Total time: 23ms
package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.y2017.shared.denseHash
import org.gristle.pdxpuzzles.advent.y2017.shared.knotHash
import org.gristle.pdxpuzzles.utilities.objects.shift

class Y17D10(private val input: String) : Day {

    override fun part1(): Int {
        val lengths = input.split(',').map { it.toInt() }
        val ring = List(256) { it }
        return ring.knotHash(lengths)
            .shift(0 - (lengths.sum() + ((1 until lengths.size).reduce { acc, i -> acc + i })))
            .take(2)
            .run {
                first() * last()
            }
    }

    override fun part2() = denseHash(input.map { it.code } + listOf(17, 31, 73, 47, 23))
}

fun main() = Day.runDay(Y17D10::class)

//    Class creation: 11ms
//    Part 1: 23874 (1ms)
//    Part 2: e1a65bfb5a5ce396025fab5528c25a87 (29ms)
//    Total time: 42ms
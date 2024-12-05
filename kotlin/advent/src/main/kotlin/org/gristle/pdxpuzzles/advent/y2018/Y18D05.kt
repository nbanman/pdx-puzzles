package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import kotlin.math.abs

class Y18D05(input: String) : Day {

    // takes chars and creates a new list of chars only adding the char if the character is not removed
    // and the char is not preceded by the opposite polarity char. If it is preceded by the opposite polarity char,
    // that char is removed from the list.
    private fun Iterable<Char>.react(removed: Char? = null): List<Char> = buildList {
        this@react.forEach { unit ->
            when {
                removed != null && ((unit == removed) || (unit == removed.uppercaseChar())) -> {}
                isEmpty() || abs(last() - unit) != 32 -> add(unit)
                else -> removeLast()
            }
        }
    }

    // The initial reaction, the size of which is the pt 1 answer. However, the pt 2 answer can use this value to 
    // bootstrap its calculations.
    private val reactedPolymer: List<Char> by lazy { input.asIterable().react() }

    // The size of the fully reacted polymer.
    override fun part1() = reactedPolymer.size

    // Further react the polymer 26 times, each time selectively removing a particular unit. Return the smallest
    // size.
    override fun part2() = ('a'..'z').minOf { removed -> reactedPolymer.react(removed).size }
}

fun main() = Day.runDay(Y18D05::class)

//    Class creation: 2ms
//    Part 1: 10972 (15ms)
//    Part 2: 5278 (27ms)
//    Total time: 45ms
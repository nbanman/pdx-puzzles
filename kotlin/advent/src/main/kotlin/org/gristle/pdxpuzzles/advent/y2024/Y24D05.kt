package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit
import org.gristle.pdxpuzzles.utilities.parsing.getIntList

class Y24D05(input: String) : Day {
    private val updates: List<List<Int>>
    private val leftOf: Map<Int, Set<Int>>
    init {
        val (rules, updates) = input.blankSplit().map { stanza ->
            stanza.lines().map { it.getIntList() }
        }
        val leftOf = mutableMapOf<Int, MutableSet<Int>>()
        for ((left, right) in rules) {
            leftOf.getOrPut(right) { mutableSetOf() }.add(left)
            leftOf.getOrPut(left) { mutableSetOf() }
        }
        this.updates = updates
        this.leftOf = leftOf
    }

    override fun part1(): Int = updates
        .filter { update -> update.zipWithNext().all { (a, b) -> a in leftOf.getValue(b) } }
        .sumOf { update -> update[update.size / 2] }

    override fun part2(): Int = updates
        .filter { update -> update.zipWithNext().any { (a, b) -> a !in leftOf.getValue(b)} }
        .sumOf { update ->
            update.sortedWith { a, b -> if (a in leftOf.getValue(b)) -1 else 1 }[update.size / 2]
        }
    }

fun main() = Day.runDay(Y24D05::class)

//    Class creation: 18ms
//    Part 1: 5129 (4ms)
//    Part 2: 4077 (7ms)
//    Total time: 29ms
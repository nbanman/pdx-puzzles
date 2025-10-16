package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getInts

private typealias Bridge = List<Y17D24.MagComp>

class Y17D24(input: String) : Day {

    data class MagComp(val a: Int = 0, val b: Int = 0) {
        val strength = a + b
        fun canJoin(n: Int) = n == a || n == b
        fun otherEnd(n: Int) = if (n == a) b else a
    }

    private fun Bridge.strength() = sumOf(MagComp::strength)

    private fun buildBridge(
        comparator: Comparator<Bridge>,
        n: Int = 0,
        bridge: Bridge = listOf(),
        remaining: Bridge = components,
    ): Bridge {
        return remaining
            .filter { it.canJoin(n) }
            .map { buildBridge(comparator, it.otherEnd(n), bridge + it, remaining - it) }
            .maxWithOrNull(comparator) ?: bridge
    }

    private val components = input
        .getInts()
        .chunked(2) { (a, b) -> MagComp(a, b) }
        .toList()

    private val compareByStrength = compareBy { bridge: Bridge -> bridge.strength() }

    override fun part1() = buildBridge(compareByStrength).strength()
    override fun part2() = buildBridge(compareBy(Bridge::size) then compareByStrength).strength()
}

fun main() = Day.runDay(Y17D24::class)

//    Class creation: 2ms
//    Part 1: 1868 (378ms)
//    Part 2: 1841 (298ms)
//    Total time: 680ms
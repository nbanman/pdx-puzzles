package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y17D24(input: String) : Day {

    data class Stats(val length: Int, val strength: Int)
    data class Component(val id: Int, val a: Int, val b: Int) {
        val strength = a + b
        fun otherEnd(n: Int): Int = if (n == a) b else a
    }
    data class State(val n: Int, val strength: Int, val visited: Long) {
        val len = visited.countOneBits()
    }

    private val parts: Map<Int, List<Component>> = buildMap<Int, MutableList<Component>> {
        for (component in input
            .getInts()
            .chunked(2)
            .mapIndexed { id, (a, b) ->  Component(id, a, b) }
        ) {
            getOrPut(component.a) { mutableListOf() }.add(component)
            getOrPut(component.b) { mutableListOf() }.add(component)
        }
    }

    fun buildBridge(parts: Map<Int, List<Component>>, comparator: Comparator<Stats>): Int {
        val q = mutableListOf(State(0, 0, 0))
        var maxStats = Stats(0, 0)
        while (q.isNotEmpty()) {
            val cur = q.removeLast()
            var candidateCount = 0
            val candidates = parts.getValue(cur.n)
                .asSequence()
                .filter { (id) -> (cur.visited shr id) and 1L == 0L }
            for (component in candidates) {
                candidateCount++
                val strength = cur.strength + component.strength
                val n = component.otherEnd(cur.n)
                val visited = cur.visited + (1L shl component.id)
                q.add(State(n, strength, visited))
            }

            if (candidateCount == 0) {
                val stats = Stats(cur.len, cur.strength)
                if (comparator.compare(maxStats, stats) == -1) maxStats = stats
            }
        }
        return maxStats.strength
    }

    override fun part1() = buildBridge(parts, compareBy(Stats::strength))
    override fun part2() = buildBridge(parts, compareBy(Stats::length) then compareBy(Stats::strength))
}

fun main() = Day.benchmarkDay(Y17D24::class)

//    Class creation: 4ms
//    Part 1: 1868 (191ms)
//    Part 2: 1841 (96ms)
//    Total time: 292ms
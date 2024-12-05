package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getIntList

class Y17D12(input: String) : Day {

    private val programs: Map<Int, List<Int>> by lazy {
        input
            .lineSequence()
            .map { it.getIntList() }
            .associate { it.first() to it.drop(1) }
    }

    // simple BFS
    private fun allLinks(start: Int, programs: Map<Int, List<Int>>): Set<Int> {
        val visited = mutableSetOf<Int>()
        val queue = ArrayDeque<Int>().apply { add(start) }
        while (queue.isNotEmpty()) {
            val current = queue.removeFirst()
            visited.add(current)
            queue.addAll(programs.getValue(current).subtract(visited))
        }
        return visited
    }

    override fun part1() = allLinks(0, programs).size

    override fun part2(): Int {
        fun removeGroup(programSet: MutableSet<Int>): MutableSet<Int> =
            programSet.apply { removeAll(allLinks(programSet.first(), programs)) }

        return generateSequence(programs.keys.toMutableSet(), ::removeGroup)
            .indexOfFirst { it.isEmpty() }
    }
}

fun main() = Day.runDay(Y17D12::class)

//    Class creation: 2ms
//    Part 1: 115 (26ms)
//    Part 2: 221 (10ms)
//    Total time: 40ms
package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.graph.Graph
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y21D09(input: String) : Day {
    private val heightMap = input.toGrid(Char::digitToInt)

    private val lowIndices = heightMap
        .indices
        .filter { index -> heightMap.getNeighbors(index).all { it > heightMap[index] } }

    override fun part1() = lowIndices.sumOf { heightMap[it] + 1 }

    override fun part2(): Int {

        // Edge finder for below BFS
        val neighbors = { id: Int ->
            heightMap
                .getNeighborIndices(id)
                .filter { heightMap[it] != 9 && heightMap[it] > heightMap[id] }
        }

        return lowIndices
            .map { Graph.bfs(startId = it, defaultEdges = neighbors).size } // for each low point get size of each basin
            .sortedDescending() // sort by largest first
            .take(3) // take top 3
            .reduce(Int::times) // multiply them together
    }
}

fun main() = Day.runDay(Y21D09::class)

//    Class creation: 39ms
//    Part 1: 448 (0ms)
//    Part 2: 1417248 (27ms)
//    Total time: 67ms
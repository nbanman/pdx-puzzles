package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.graph.Graph
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y24D10(input: String) : Day {
    private val topoMap = input.toGrid(Char::digitToInt)
    private val trailheads = topoMap.withIndex().filter { (_, n) -> n == 0 }.map { (idx) -> topoMap.coordOf(idx) }
    private val hike: (Pair<Coord, Int>) -> List<Pair<Coord, Int>> = { (pos, height) ->
        topoMap.getNeighborsIndexedValue(pos)
            .filter { (_, neighborHeight) -> height + 1 == neighborHeight }
            .map { (neighborIndex, neighborHeight) -> topoMap.coordOf(neighborIndex) to neighborHeight }
    }

    private fun distinctPaths(trailhead: Coord): Int {
        val q = mutableListOf(trailhead to 0)
        var distinctPaths = 0
        while (q.isNotEmpty()) {
            val state = q.removeLast()
            if (state.second == 9) distinctPaths++
            q.addAll(hike(state))
        }
        return distinctPaths
    }

    override fun part1(): Int = trailheads.sumOf { trailhead ->
        Graph.bfsSequence(trailhead to 0, defaultEdges = hike).count { it.id.second == 9 }
    }

    override fun part2(): Int = trailheads.sumOf(::distinctPaths)
}

fun main() = Day.runDay(Y24D10::class)

//    Class creation: 5ms
//    Part 1: 461 (19ms)
//    Part 2: 875 (8ms)
//    Total time: 33ms

@Suppress("unused") // 81, 227
private val test = listOf("""0123
1234
8765
9876""",
    """89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732""",
    """012345
123456
234567
345678
4.6789
56789.""")
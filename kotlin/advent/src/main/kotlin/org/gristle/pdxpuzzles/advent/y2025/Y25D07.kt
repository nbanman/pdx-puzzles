package org.gristle.pdxpuzzles.advent.y2025

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.algorithms.Graph
import org.gristle.pdxpuzzles.utilities.iteration.pollUntil
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import java.util.PriorityQueue

class Y25D07(input: String) : Day {
    private val manifold = input.toGrid()
    private val start = manifold.indexOf('S')
    private val bottomRow = manifold.size - manifold.width

    override fun part1(): Int {
        var splits = 0

        val moveDown: (Int) -> List<Int> = { pos: Int ->
            if (pos >= bottomRow) {
                emptyList()
            } else {
                val down = pos + manifold.width
                if (manifold[down] == '^') {
                    splits++
                    listOf(down - 1, down + 1)
                } else {
                    listOf(down)
                }
            }
        }
        Graph.bfs(startId = start, defaultEdges = moveDown,)
        return splits
    }

    override fun part2(): Long {
        var totalTimelines = 0L
        var todo = PriorityQueue(
            compareBy<Pair<Int, Long>> { it.first }.thenByDescending { it.second }
        )
        todo.add(start to 1L)
        var next = PriorityQueue(
            compareBy<Pair<Int, Long>> { it.first }.thenByDescending { it.second }
        )
        val visited = mutableSetOf<Int>()
        val mergeTracker = mutableMapOf<Int, Long>()
        while (todo.isNotEmpty()) {
            while (true) {
                val (pos, timelines) = todo
                    .pollUntil { (pos) -> visited.add(pos) }
                    ?: break
                val down = pos + manifold.width
                if (down >= bottomRow) {
                    totalTimelines += timelines
                    continue
                }
                if (manifold[down] == '^') {
                    for (offset in -1..1 step(2)) {
                        val aPos = down + offset
                        val other = mergeTracker.getOrDefault(aPos, 0)
                        mergeTracker[aPos] = timelines + other
                        next.add(aPos to timelines + other)
                    }
                } else {
                    val other = mergeTracker.getOrDefault(down, 0)
                    mergeTracker[down] = timelines + other
                    next.add(down to timelines + other)
                }
            }
            mergeTracker.clear()
            visited.clear()
            todo = next.apply { next = todo }
        }
        return totalTimelines
    }
}

fun main() = Day.runDay(Y25D07::class)

@Suppress("unused")
private val test = listOf(""".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
""")
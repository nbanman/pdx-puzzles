package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import java.util.ArrayDeque

class Y24D10(private val topoMap: String) : Day {
    private fun solve(distinctPaths: Boolean): Int {
        // working directly with string, so need width of each line to navigate in 2D properly
        val width = topoMap.indexOf('\n') + 1

        // finds all trailheads, then passes that on to the paths function, then sums the result
        return topoMap.withIndex()
            .filter { (_, n) -> n == '0' }
            .sumOf { (trailhead) -> paths(trailhead, width, distinctPaths) }
    }

    private data class State(val pos: Int, val height: Char)

    // Standard BFS, with node tracking enabled or disabled depending on which part.
    private fun paths(trailhead: Int, width: Int, distinctPaths: Boolean): Int {
        val q = ArrayDeque<State>()
        q.add(State(trailhead, '0'))
        var paths = 0
        val neighbors = listOf(-width, 1, width, -1)
        val visited = if (distinctPaths) null else BooleanArray(topoMap.length)
        while (q.isNotEmpty()) {
            val state = q.removeFirst()
            if (state.height == '9') paths++
            for (adjacent in hikePath(state, neighbors)) {
                if (visited != null) {
                    if (visited[adjacent.pos]) continue else visited[adjacent.pos] = true
                }
                q.addLast(adjacent)
            }
        }
        return paths
    }

    // finds adjacent spots along path, and returns the ones that are one height higher than current state
    private fun hikePath(state: State, neighbors: List<Int>): List<State> {
        val (pos, height) = state
        return neighbors.mapNotNull { offset ->
            val neighborPos = pos + offset
            topoMap.getOrNull(neighborPos)?.let { neighborHeight ->
                if (neighborHeight == height + 1) {
                    State(neighborPos, neighborHeight)
                } else {
                    null
                }
            }
        }
    }

    override fun part1(): Int = solve(distinctPaths = false)
    override fun part2(): Int = solve(distinctPaths = true)
}

fun main() = Day.runDay(Y24D10::class)

//    Class creation: 1ms
//    Part 1: 461 (10ms)
//    Part 2: 875 (5ms)
//    Total time: 17ms

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
56789""")
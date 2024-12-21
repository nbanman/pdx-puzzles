package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.iteration.minMax
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.toCoord
import java.util.ArrayDeque

class Y24D20(private val racetrack: String) : Day {
    private val width = racetrack.indexOf('\n') + 1
    private val height = (racetrack.length + 1) / width
    private val movements = listOf(-width, 1, width, -1)
    private val start = racetrack.indexOf('S')
    private val end = racetrack.indexOf('E')
    private val visited = IntArray(racetrack.length) { 1_000_000 }
    private val fromStart = bfs(start, end, visited.clone().also { it[start] = 0 }, ::getNeighbors)
    private val fromEnd = bfs(end, start, visited.also { it[end] = 0 }, ::getNeighbors)
    private val honestDistance = fromStart[end]
    private val threshold = honestDistance - 100

    private inline fun<T> bfs(start: Int, end: Int?, visited: T, edges: (Int, T, Int) -> List<Pair<Int, Int>>): T {
        val q = ArrayDeque<Pair<Int, Int>>()
        q.addLast(0 to start)

        while (q.isNotEmpty()) {
            val (steps, current) = q.removeFirst()
            if (current == end) break
            val neighbors = edges(current, visited, steps)
            q.addAll(neighbors)
        }
        return visited
    }

    private fun getNeighbors(
        current: Int,
        visited: IntArray,
        steps: Int
    ): List<Pair<Int, Int>> = movements.mapNotNull { movement ->
        val neighbor = current + movement
        when {
            visited[neighbor] != 1_000_000 -> null
            racetrack[neighbor] == '#' -> {
                visited[neighbor] = steps + 1
                null
            }
            else -> {
                val neighborSteps = steps + 1
                visited[neighbor] = neighborSteps
                neighborSteps to neighbor
            }
        }
    }

    private fun countValid(
        pos: Int,
        endCoord: Coord,
        steps: Int,
    ): Pair<Int, List<Int>> {
        if (racetrack[pos] == '#') return 0 to emptyList()

        val posCoord = pos.toCoord(width)
        val difference = endCoord - posCoord
        val minSteps = steps + difference.manhattanDistance()

        if (minSteps > threshold) return 0 to emptyList()

        var count = 0
        val savings = mutableListOf<Int>()
        val allowance = (threshold - minSteps) / 2
        val (north, south) = minMax(posCoord.y, endCoord.y)
        val yRange = (north - allowance).coerceAtLeast(1)..
                (south + allowance).coerceAtMost(height - 2)
        for (y in yRange) {
            val xAllowance = when {
                y < north -> allowance - (north - y)
                y > south -> allowance - (y - south)
                else -> allowance
            }
            val (west, east) = minMax(posCoord.x, endCoord.x)
            val xRange = (west - allowance).coerceAtLeast(1)..
                    (east + allowance).coerceAtMost(width - 3)
            for (x in xRange) {
                val rePos = y * width + x
                if (racetrack[rePos] == '#') continue
                val cheatSteps = steps + posCoord.manhattanDistance(Coord(x, y)) + fromEnd[rePos]
                if (cheatSteps <= threshold) {
                    count++
                    savings.add(honestDistance - cheatSteps)
                }
            }
        }
        return count to savings
    }

    override fun part1(): Int = fromStart.withIndex().count { (pos, steps) ->
        racetrack[pos] == '#' && steps + fromEnd[pos] <= threshold
    }

    override fun part2() = let {
        val endCoord = end.toCoord(width)
        val test = fromStart.withIndex().map { (pos, steps) ->
            (pos to steps) to countValid(pos, endCoord, steps)
        }
        test.sumOf { it.second.first }
    }
}

fun main() = Day.runDay(Y24D20::class)
// 25536 too low
// 42903515 too high

@Suppress("unused")
private val test = listOf("""###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############""")
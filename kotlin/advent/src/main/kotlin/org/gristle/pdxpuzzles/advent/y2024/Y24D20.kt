package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.iteration.minMax
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.toCoord
import java.util.ArrayDeque
import kotlin.math.absoluteValue
import kotlin.math.max
import kotlin.math.min

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

    private inline fun <T> bfs(start: Int, end: Int?, visited: T, edges: (Int, T, Int) -> List<Pair<Int, Int>>): T {
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
    ): Int {
        if (racetrack[pos] == '#') return 0

        val posCoord = pos.toCoord(width)
        val difference = endCoord - posCoord
        val minSteps = steps + difference.manhattanDistance()

        if (minSteps > threshold) return 0

        var count = 0
        val allowance = (threshold - minSteps) / 2
        val (north, south) = minMax(posCoord.y, endCoord.y)
        val yMin = min(north - allowance, posCoord.y - 20).coerceAtLeast(1)
        val yMax = min(south + allowance, posCoord.y + 20).coerceAtMost(height - 2)
        val yRange = yMin..yMax
        for (y in yRange) {
            val (west, east) = minMax(posCoord.x, endCoord.x)
            val xAllowance = when {
                y < north -> allowance - (north - y)
                y > south -> allowance - (y - south)
                else -> allowance
            }
            val totalAllowed = 20 - (posCoord.y - y).absoluteValue
            val xMin = max(posCoord.x - totalAllowed, west - xAllowance).coerceAtLeast(1)
            val xMax = min(posCoord.x + totalAllowed, east + xAllowance).coerceAtMost(width - 3)
            val xRange = xMin..xMax
            for (x in xRange) {
                val rePos = y * width + x
                if (racetrack[rePos] == '#') continue
                val cheatSteps = steps + posCoord.manhattanDistance(Coord(x, y)) + fromEnd[rePos]
                if (cheatSteps <= threshold) count++
            }
        }
        return count
    }

    override fun part1(): Int = fromStart.withIndex().count { (pos, steps) ->
        racetrack[pos] == '#' && steps + fromEnd[pos] <= threshold
    }

    override fun part2() = fromStart.withIndex().sumOf { (pos, steps) ->
        countValid(pos, end.toCoord(width), steps)
    }
}

fun main() = Day.runDay(Y24D20::class)

//    Class creation: 9ms
//    Part 1: 1406 (4ms)
//    Part 2: 1006101 (90ms)
//    Total time: 104ms

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
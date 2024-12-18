package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.parsing.getIntList

class Y24D18(input: String) : Day {
    data class State(val pos: Coord, val t: Int)

    private val bytes = input.getIntList().chunked(2).map { (x, y) -> Coord(x, y) }
        .runningFold(setOf<Coord>()) { acc, pos -> acc + pos }

    private fun solve(simulate: Int): Int? {
        val bytes = bytes[simulate]
        val bounds = 0 until 71
        val start = Coord.ORIGIN to 0
        val end = Coord(71 - 1, 71 -1)
        val visited = mutableSetOf<Coord>()
        val q = ArrayDeque<Pair<Coord, Int>>()
        q.addLast(start)
        return generateSequence { q.removeFirstOrNull() }
            .firstOrNull { (pos, steps) ->
                if (pos == end) {
                    true
                } else {
                    visited.add(pos)
                    val neighbors = pos
                        .getNeighbors()
                        .filter { it.x in bounds && it.y in bounds && it !in bytes && visited.add(it) }
                    q.addAll(neighbors.map { it to steps + 1 })
                    false
                }
            }?.second
    }

    override fun part1(): Int? = solve(1024)

    override fun part2(): String {
        var l = 1025
        var r = bytes.lastIndex
        while (l != r) {
            val m = (l + r) / 2
            if (solve(m) == null) {
                r = m
            } else {
                l = m + 1
            }
        }
        val byte = bytes[r].last()
        return "${byte.x},${byte.y}"
    }
}

fun main() = Day.runDay(Y24D18::class)
// 3038 wrong

@Suppress("unused")
private val test = listOf("""5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0""")
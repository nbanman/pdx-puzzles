package org.gristle.pdxpuzzles.advent.y2025

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y25D07(manifold: String) : Day {
    private val p1Answer: Int
    private val p2Answer: Long

    init {
        val width = manifold.indexOf('\n') + 1
        val finished = manifold.length - width * 2 + 1

        var splits = 0

        var todo = LongArray(width - 1)
        todo[manifold.indexOf('S')] = 1
        var next = LongArray(width - 1)
        for (row in 0 until manifold.length step width) {
            for ((pos, timeline) in todo.withIndex()) {
                if (timeline == 0L) continue
                if (manifold[pos + row] == '^') {
                    splits++
                    for (offset in -1..1 step(2)) {
                        next[pos + offset] += timeline
                    }
                } else {
                    next[pos] += timeline
                }
            }
            if (row == finished) break
            todo = next
            next = LongArray(width - 1)
        }
        p1Answer = splits
        p2Answer = next.sum()
    }

    override fun part1() = p1Answer

    override fun part2() = p2Answer
}

fun main() = Day.benchmarkDay(Y25D07::class)

//    Class creation: 2ms
//    Part 1: 1533 (1ms)
//    Part 2: 10733529153890 (2ms)
//    Total time: 5ms

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
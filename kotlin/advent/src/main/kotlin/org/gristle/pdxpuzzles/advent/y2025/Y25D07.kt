package org.gristle.pdxpuzzles.advent.y2025

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y25D07(input: String) : Day {
    private val p1Answer: Int
    private val p2Answer: Long

    init {
        val manifold = input.toGrid()
        val finished = manifold.size - manifold.width * 2

        var splits = 0

        var todo = LongArray(manifold.width)
        todo[manifold.indexOf('S')] = 1
        var next = LongArray(manifold.width)
        for (row in 0 until manifold.size step manifold.width) {
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
            next = LongArray(manifold.width)
        }
        p1Answer = splits
        p2Answer = next.sum()
    }

    override fun part1() = p1Answer

    override fun part2() = p2Answer
}

fun main() = Day.runDay(Y25D07::class)

//    Class creation: 7ms
//    Part 1: 1533 (1ms)
//    Part 2: 10733529153890 (2ms)
//    Total time: 11ms

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
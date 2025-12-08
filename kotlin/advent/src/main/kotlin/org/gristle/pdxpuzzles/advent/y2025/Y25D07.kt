package org.gristle.pdxpuzzles.advent.y2025

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y25D07(manifold: String) : Day {
    private val p1Answer: Int
    private val p2Answer: Long

    init {
        val width = manifold.indexOf('\n') + 1
        val finished = (manifold.length - width * 2 + 1) / width

        var splits = 0

        var todo = LongArray(1) { 1 }
        var next = LongArray(todo.size + 2)

        val middle = manifold.indexOf('S')

        for (row in 2 .. finished step 2) {
            for ((pos, timeline) in todo.withIndex()) {
                if (timeline == 0L) continue
                if (manifold[(pos + middle - (row / 2 - 1)) + row * width] == '^') {
                    splits++
                    next[pos] += timeline
                    next[pos + 2] += timeline
                } else {
                    next[pos + 1] += timeline
                }
            }
            todo = next
            next = LongArray(todo.size + 2)
        }
        p1Answer = splits
        p2Answer = todo.sum()
    }

    override fun part1() = p1Answer

    override fun part2() = p2Answer
}

fun main() = Day.runDay(Y25D07::class)

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
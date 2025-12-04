package org.gristle.pdxpuzzles.advent.y2025

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.toMutableGrid

class Y25D04(private val input: String) : Day {
    private fun solve(loop: Boolean): Int {
        val grid = input.toMutableGrid { c -> c == '@' }
        var totalRemoved = 0
        do {
            val removable = grid.withIndex()
                .filter { (idx, b) ->
                    if (b) {
                        grid.getNeighbors(idx, true).count { it } < 4
                    } else {
                        false
                    }
                }
            for ((idx, _) in removable) {
                grid[idx] = false
            }
            totalRemoved += removable.size
        } while (loop && removable.isNotEmpty())
        return totalRemoved
    }

    override fun part1() = solve(false)
    override fun part2() = solve(true)
}

fun main() = Day.runDay(Y25D04::class)

//    Class creation: 1ms
//    Part 1: 1604 (22ms)
//    Part 2: 9397 (90ms)
//    Total time: 114ms
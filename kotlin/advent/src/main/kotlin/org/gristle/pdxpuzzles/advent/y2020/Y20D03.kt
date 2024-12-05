package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y20D03(input: String) : Day {

    val grid = input.toGrid { it == '#' }

    fun solve(right: Int = 3, down: Int = 1) =
        (down until grid.height step down).foldIndexed(0) { i, acc, y ->
            acc + if (grid[Coord(((i + 1) * right) % grid.width, y)]) 1 else 0
        }

    override fun part1() = solve()

    override fun part2() = listOf(
        solve(1),
        solve(3),
        solve(5),
        solve(7),
        solve(1, 2)
    ).fold(1L, Long::times)
}

fun main() = Day.runDay(Y20D03::class)

//    Class creation: 24ms
//    Part 1: 294 (1ms)
//    Part 2: 5774564250 (0ms)
//    Total time: 26ms
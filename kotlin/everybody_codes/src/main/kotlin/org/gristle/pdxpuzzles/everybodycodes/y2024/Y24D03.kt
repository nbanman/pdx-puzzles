package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.toGrid

object Y24D03 : Day {
    private fun solve(input: String, diagonals: Boolean = false): Int {
        val blocks = input
            .toGrid()
            .let { grid -> grid.coords().filter { pos -> grid[pos] == '#' }.toSet() }
        val dig = { blocks: Set<Coord> ->
            blocks
                .filter { block -> block.getNeighbors(diagonals).all { it in blocks } }
                .toSet()
        }
        return generateSequence(blocks, dig)
            .takeWhile { it.isNotEmpty() }
            .fold(0) { acc, blocks -> acc + blocks.size }
    }


    override fun part1(input: String): Int = solve(input)
    override fun part2(input: String): Int = solve(input)
    override fun part3(input: String): Int = solve(input, true)
}

fun main() = Day.runDay(Y24D03::class)

//    Quest 1: 134 (4ms)
//    Quest 2: 2810 (5ms)
//    Quest 3: 10443 (17ms)
//    Total time: 27ms
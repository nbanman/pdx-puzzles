package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y15D18(input: String) : Day {

    private val lights = input.toGrid { it == '#' }

    private fun Grid<Boolean>.iterate(cornersStuck: Boolean = false): Grid<Boolean> {
        val corners = if (cornersStuck) {
            listOf(0, width - 1, size - width, lastIndex)
        } else {
            emptyList()
        }

        return Grid(height, width) { i ->
            if (i in corners) {
                true
            } else {
                val neighborsOn = getNeighbors(i, true).count { it }
                neighborsOn == 3 || (neighborsOn == 2 && this[i])
            }
        }
    }

    private fun solve(cornersStuck: Boolean) = generateSequence(lights) { it.iterate(cornersStuck) }
        .take(101)
        .last()
        .count { it }

    override fun part1() = solve(false)

    override fun part2() = solve(true)
}

fun main() = Day.runDay(Y15D18::class)

//    Class creation: 24ms
//    Part 1: 1061 (338ms)
//    Part 2: 1006 (218ms)
//    Total time: 581ms
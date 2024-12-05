package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.isEven
import org.gristle.pdxpuzzles.utilities.objects.*
import kotlin.math.sqrt

class Y17D21(private val input: String) : Day {

    private val rules: Map<Grid<Boolean>, Grid<Boolean>> = buildMap {
        input
            .replace("/", "")
            .lines()
            .forEach { line ->
                val side: Int
                val prev = line
                    .takeWhile { it != ' ' }
                    .let {
                        side = sqrt(it.length.toDouble()).toInt()
                        Grid(side, side) { i -> it[i] == '#' }
                    }

                val next = line
                    .takeLastWhile { it != ' ' }
                    .let { Grid(side + 1, side + 1) { i -> it[i] == '#' } }

                listOf(prev, prev.flipX())
                    .asSequence()
                    .flatMap { generateSequence(it, Grid<Boolean>::rotate90).take(4) }
                    .forEach { put(it, next) }
            }
    }

    private fun expandGrid(grid: Grid<Boolean>): Grid<Boolean> {
        val length = if (grid.width.isEven()) 2 else 3
        val subGrids = buildList {
            for (y in 0 until grid.height step length) {
                for (x in 0 until grid.width step length) {
                    add(grid.subGrid(Coord(x, y), length, length))
                }
            }
        }.toGrid(grid.width / length)

        val transformedSubs = subGrids.mapToGrid { transGrid ->
            rules[transGrid] ?: throw Exception("no rule matches transGrid")
        }

        val expandedSize = transformedSubs.first().size * transformedSubs.size

        val expandedLength = sqrt(expandedSize.toDouble()).toInt()

        val expandedArray = MutableGrid(expandedLength, expandedLength) { false }

        for (subPos in transformedSubs.coords()) {
            val subGrid = transformedSubs[subPos]
            val offset = Coord(subPos.x * (length + 1), subPos.y * (length + 1))
            for (pos in subGrid.coords()) {
                expandedArray[pos + offset] = subGrid[pos]
            }
        }

        return expandedArray.toGrid(expandedLength)
    }

    private fun solve(iterations: Int): Int {
        val initial = ".#...####".toGrid(3).mapToGrid { it == '#' }
        return generateSequence(initial, ::expandGrid)
            .take(iterations + 1)
            .last()
            .count { it }
    }

    override fun part1() = solve(5)

    override fun part2() = solve(18)
}

fun main() = Day.runDay(Y17D21::class)

//    Class creation: 39ms
//    Part 1: 150 (6ms)
//    Part 2: 2606275 (1205ms)
//    Total time: 1252ms
package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.iteration.minMax
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y21D05(input: String) : Day {

    data class Line(val start: Coord, val end: Coord) {
        fun straightRange(includeDiagonals: Boolean): List<Coord> =
            if (start.x == end.x) {
                val (small, large) = minMax(start.y, end.y)
                (small..large).fold(emptyList()) { acc, i ->
                    acc + Coord(start.x, i)
                }
            } else if (start.y == end.y) {
                val (small, large) = minMax(start.x, end.x)
                (small..large).fold(emptyList()) { acc, i ->
                    acc + Coord(i, start.y)
                }
            } else if (includeDiagonals) {
                val xRange = if (start.x < end.x) {
                    start.x..end.x
                } else {
                    start.x downTo end.x
                }

                val yIncrement = if (start.y < end.y) 1 else -1

                xRange.foldIndexed(emptyList()) { index, acc, i ->
                    acc + Coord(i, start.y + index * yIncrement)
                }
            } else {
                emptyList()
            }
    }

    val lines = input
        .getInts()
        .chunked(4) { (x1, y1, x2, y2) -> Line(Coord(x1, y1), Coord(x2, y2)) }
        .toList()

    fun solve(includeDiagonals: Boolean): Int =
        buildMap {
            lines
                .flatMap { line -> line.straightRange(includeDiagonals) }
                .forEach { this[it] = (this[it] ?: 0) + 1 }
        }.count { it.value >= 2 }
    
    override fun part1() = solve(false)

    override fun part2() = solve(true)

}

fun main() = Day.runDay(Y21D05::class)

//    Class creation: 78ms
//    Part 1: 5774 (154ms)
//    Part 2: 18423 (148ms)
//    Total time: 381ms   
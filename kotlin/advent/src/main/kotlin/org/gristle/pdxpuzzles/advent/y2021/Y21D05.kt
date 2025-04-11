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
                (small..large).map { y -> Coord(start.x, y) }
            } else if (start.y == end.y) {
                val (small, large) = minMax(start.x, end.x)
                (small..large).map { x -> Coord(x, start.y) }
            } else if (includeDiagonals) {
                val xRange = if (start.x < end.x) {
                    start.x..end.x
                } else {
                    start.x downTo end.x
                }
                val yIncrement = if (start.y < end.y) 1 else -1
                xRange.mapIndexed { i, x -> Coord(x, start.y + i * yIncrement) }
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

//    Class creation: 5ms
//    Part 1: 5774 (31ms)
//    Part 2: 18423 (32ms)
//    Total time: 70ms
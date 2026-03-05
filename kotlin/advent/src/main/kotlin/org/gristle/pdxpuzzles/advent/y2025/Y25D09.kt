package org.gristle.pdxpuzzles.advent.y2025

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.combinatorics.getPairSequence
import org.gristle.pdxpuzzles.utilities.iteration.minMax
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.getBounds
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import kotlin.math.abs

class Y25D09(input: String) : Day {
    private val redTiles = input.getInts()
        .chunked(2) { (a, b) -> Coord(a, b) }
        .toList()

    fun rectArea(a: Coord, b: Coord): Long = (abs(a.x - b.x).toLong() + 1L) * (abs(a.y - b.y).toLong() + 1L)

    override fun part1() = redTiles.getPairSequence().maxOf { (a, b) -> rectArea(a, b) }

    override fun part2(): Long {
        val (xBounds, yBounds) = redTiles.getBounds()

        val greenX = mutableMapOf<Int, MutableList<Int>>()
        val greenY = mutableMapOf<Int, MutableList<Int>>()

        for ((prev, next) in (redTiles + redTiles[0]).windowed(2)) {
            val (xmin, xmax) = minMax(prev.x, next.x)
            for (x in xmin + 1 until xmax) {
                greenX.getOrPut(x) { mutableListOf() }.add(prev.y)
            }
            val (ymin, ymax) = minMax(prev.y, next.y)
            for (y in ymin + 1 until ymax) {
                greenY.getOrPut(y) { mutableListOf() }.add(prev.x)
            }
        }

        val possible = redTiles
            .filter { pos ->
                pos.x != xBounds.first && pos.x != xBounds.last && pos.y != yBounds.first && pos.y != yBounds.last
            }.getPairSequence()
            .filter { (a, b) ->
                val (xmin, xmax) = minMax(a.x, b.x)
                val (ymin, ymax) = minMax(a.y, b.y)
                greenX[xmin]
                    ?.let { it.all { y -> y !in (ymin + 1)..<ymax } }
                    ?: true
                        && greenX[xmax]
                            ?.let { it.all { y -> y !in (ymin + 1)..<ymax } }
                            ?: true
                        && greenY[ymin]
                            ?.let { it.all { x -> x !in (xmin + 1)..<xmax } }
                            ?:true
                        && greenY[ymax]
                            ?.let { it.all { x -> x !in (xmin + 1)..<xmax } }
                            ?:true
            }.toList()
        return possible.maxOf { (a, b) -> rectArea(a, b) }
    }
}

fun main() = Day.runDay(Y25D09::class)

@Suppress("unused")
private val test = listOf("""""")
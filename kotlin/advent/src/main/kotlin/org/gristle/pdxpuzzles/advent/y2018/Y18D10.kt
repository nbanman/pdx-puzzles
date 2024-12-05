package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.minMaxRanges
import org.gristle.pdxpuzzles.utilities.objects.toGraphicString
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import org.gristle.pdxpuzzles.utilities.parsing.ocr

class Y18D10(input: String) : Day {

    data class MovingPoint(val pos: Coord, val vel: Coord)

    private val points = input
        .getInts()
        .chunked(4) { (px, py, vx, vy) -> MovingPoint(Coord(px, py), Coord(vx, vy)) }
        .toList()

    private fun Iterable<MovingPoint>.move() = map { point -> point.copy(pos = point.pos + point.vel) }

    private val answer = generateSequence(points) { it.move() }
        .withIndex()
        .first { (_, points) ->
            val (_, yRange) = points.map(MovingPoint::pos).minMaxRanges()
            yRange.last - yRange.first == 9
        }

    override fun part1() = answer.value.map(MovingPoint::pos).toGraphicString('.').ocr()
    override fun part2() = answer.index
}

fun main() = Day.runDay(Y18D10::class)

//    Class creation: 187ms
//    Part 1: LRCXFXRP (10ms)
//    Part 2: 10630 (0ms)
//    Total time: 198ms
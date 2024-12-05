package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.iteration.minMax
import org.gristle.pdxpuzzles.utilities.parsing.getIntList
import kotlin.math.abs

class Y21D17(input: String) : Day {

    data class Vector(val pos: Coord, val velocity: Coord) {
        fun step(): Vector {
            val newXV = when {
                velocity.x > 0 -> velocity.x - 1
                velocity.x < 0 -> velocity.x + 1
                else -> 0
            }
            return Vector(pos + velocity, Coord(newXV, velocity.y - 1))
        }
    }

    data class Box(val tl: Coord, val br: Coord) {

        fun ySteps(v: Vector): List<Int> {
            val timeOffset = if (v.velocity.y >= 0) v.velocity.y * 2 + 1 else 0
            val velocity = if (v.velocity.y >= 0) -(v.velocity.y + 1) else v.velocity.y
            val cromulent = mutableListOf<Int>()
            var shot = Vector(Coord(0, 0), Coord(0, velocity))
            var time = 0
            while (shot.pos.y >= br.y) {
                time++
                shot = shot.step()
                if (shot.pos.y in yRange) cromulent.add(time + timeOffset)
            }
            return cromulent
        }

        fun xValues(steps: Int): List<Int> {
            return (stallSpeed..br.x).filter { xVel ->
                (1..steps)
                    .fold(Vector(Coord(0, 0), Coord(xVel, 0))) { acc, _ -> acc.step() }
                    .pos.x in xRange
            }
        }

        val minY = br.y
        val maxY = (1 until abs(br.y)).sum()
        private val yRange = br.y..tl.y
        private val xRange = tl.x..br.x
        private val stallSpeed = let {
            (5..br.x).first { (1..it).sum() in xRange }
        }
    }

    private val box = input
        .getIntList()
        .let { (x1, x2, y1, y2) ->
            val (xa, xb) = minMax(x1, x2)
            val (ya, yb) = minMax(y1, y2)
            Box(Coord(xa, yb), Coord(xb, ya))
        }

    override fun part1() = box.maxY

    override fun part2() = (box.minY..box.maxY)
        .map { y -> y to box.ySteps(Vector(Coord(0, 0), Coord(0, y))) }
        .filter { (_, ySteps) -> ySteps.isNotEmpty() }
        .flatMap { (y, ySteps) ->
            ySteps.flatMap { yStep ->
                box.xValues(yStep).map { x -> Coord(x, y) }
            }
        }.distinct()
        .size
}

fun main() = Day.runDay(Y21D17::class)

//    Class creation: 14ms
//    Part 1: 17766 (0ms)
//    Part 2: 1733 (70ms)
//    Total time: 85ms
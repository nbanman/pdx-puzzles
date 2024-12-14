package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.pow
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.map
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import kotlin.math.max
import kotlin.math.pow

class Y24D14(input: String) : Day {
    private val width = 101
    private val height = 103
    private val seconds = 100
    data class Robot(val p: Coord, val v: Coord)
    private val robots = input
        .getInts()
        .chunked(4) { (px, py, vx, vy) -> Robot(Coord(px, py), Coord(vx, vy)) }
        .toList()

    private fun move(robots: List<Robot>) = robots.map {
        val p = Coord((it.p.x + it.v.x).mod(width), (it.p.y + it.v.y).mod(height))
        it.copy(p = p)
    }
    private fun List<Robot>.score(): Int {
        val quadrants = IntArray(4)
        val splitX = width / 2
        val splitY = height / 2

        mapNotNull { (p) ->
            when (p.x) {
                in 0 until splitX -> 0
                splitX -> null
                else -> 2
            }?.let { x ->
                when(p.y) {
                    in 0 until splitY -> 0
                    splitY -> null
                    else -> 1
                }?.let { y -> quadrants[x + y]++ }
            }
        }
        return quadrants.reduce(Int::times)
    }

    override fun part1() = generateSequence(robots, ::move)
        .take(seconds + 1)
        .last()
        .score()

    override fun part2(): Int {
        val sample: List<Pair<Double, Double>> = generateSequence(robots, ::move)
            .take(max(width, height))
            .map { robotList ->
                val robots = robotList.map(Robot::p)
                // note that this isn't totally accurate for width because the larger height sample is used,
                // but it should still work because the stars will align only once per period and the
                // variance for that instance should be dramatically lower than for anything else.
                val (xMean, yMean) = robots
                    .reduce(Coord::plus)
                    .let { (x, y) -> x / robots.size.toDouble() to y / robots.size.toDouble() }
                robots
                    .map { (x, y) -> (x - xMean).pow(2) to (y - yMean).pow(2) }
                    .unzip()
                    .map { it.average() }
            }.toList()
        val xOffset = sample.withIndex().minBy { (_, variances) -> variances.first }.index
        val yOffset = sample.withIndex().minBy { (_, variances) -> variances.second }.index

        return generateSequence(xOffset) { it + width }.first { (it - yOffset).mod(height) == 0 }
    }
}

fun main() = Day.runDay(Y24D14::class)

//    Class creation: 9ms
//    Part 1: 210587128 (24ms)
//    Part 2: 7286 (39ms)
//    Total time: 73ms

@Suppress("unused")
private val test = listOf("""p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3""")
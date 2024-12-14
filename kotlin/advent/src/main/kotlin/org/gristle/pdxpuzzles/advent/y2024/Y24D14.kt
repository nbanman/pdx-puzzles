package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y24D14(input: String) : Day {
    private val width = 101
    private val height = 103
    private val seconds = 100
    data class Robot(val p: Coord, val v: Coord)
    private val robots = input
        .getInts()
        .chunked(4) { (px, py, vx, vy) -> Robot(Coord(px, py), Coord(vx, vy)) }
        .toList()

    private fun List<Robot>.move() = map {
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

    override fun part1() = generateSequence(robots) { robots -> robots.move() }
        .take(seconds + 1)
        .last()
        .score()

    override fun part2() = generateSequence(robots) { robots -> robots.move() }
        .map { robots -> robots.map(Robot::p) }
        .indexOfFirst { it.distinct().size == it.size }
}

fun main() = Day.runDay(Y24D14::class)

//    Class creation: 9ms
//    Part 1: 210587128 (23ms)
//    Part 2: 7286 (216ms)
//    Total time: 250ms

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
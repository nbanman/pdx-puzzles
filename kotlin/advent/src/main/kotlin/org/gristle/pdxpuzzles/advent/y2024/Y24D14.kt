package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y24D14(input: String) : Day {
    data class Robot(val p: Coord, val v: Coord)
    private val robots = input
        .getInts()
        .chunked(4) { (px, py, vx, vy) -> Robot(Coord(px, py), Coord(vx, vy)) }
        .toList()

    private fun List<Robot>.move(width: Int, height: Int) = map {
        val p = Coord((it.p.x + it.v.x).mod(width), (it.p.y + it.v.y).mod(height))
        it.copy(p = p)
    }
    private fun List<Robot>.score(width: Int, height: Int): Int {
        val quadrants = IntArray(4)
        for (robot in this) {

        }
    }

    private fun solve(seconds: Int, width: Int, height: Int): Int =
        generateSequence(robots) { robots -> robots.move(width, height) }
        .take(seconds + 1)
        .last()
        .score(width, height)

    override fun part1() = 3
    override fun part2() = 3
}

fun main() = Day.runDay(Y24D14::class)

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
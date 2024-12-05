package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.math.isEven
import kotlin.math.ceil
import kotlin.math.max
import kotlin.math.min
import kotlin.math.sqrt

class Y17D03(private val input: String) : Day {

    data class Turtle(var dir: Nsew, var pos: Coord, var vel: Int)

    override fun part1(): Int {
        val squareRoot = ceil(sqrt(input.toFloat())).toInt().let { if (it.isEven()) it + 1 else it }
        val furthest = (squareRoot / 2) * 2
        val br = squareRoot * squareRoot
        val diff = (br - input.toInt()) % furthest
        return furthest - min(diff, furthest / 2) + max(0, diff - furthest / 2)
    }

    override fun part2(): Int {
        val t = Turtle(Nsew.SOUTH, Coord.ORIGIN, 0)
        val space = mutableMapOf(t.pos to 1)
        while (true) {
            t.dir = t.dir.left()
            if (t.dir == Nsew.EAST || t.dir == Nsew.WEST) t.vel++
            for (i in 1..t.vel) {
                t.pos = t.pos.move(t.dir)
                val squareVal = t.pos.getNeighbors(true).sumOf { space[it] ?: 0 }
                if (squareVal > input.toInt()) return squareVal
                space[t.pos] = squareVal
            }
        }
    }
}

fun main() = Day.runDay(Y17D03::class)

//    Class creation: 9ms
//    Part 1: 552 (0ms)
//    Part 2: 330785 (3ms)
//    Total time: 13ms
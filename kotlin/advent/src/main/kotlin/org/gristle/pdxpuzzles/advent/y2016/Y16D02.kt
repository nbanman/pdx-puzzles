package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import kotlin.math.sign

class Y16D02(input: String) : Day {
    private val instructions = input.lineSequence()

    fun solve(
        start: Coord,
        inBounds: Coord.() -> Boolean,
        conversion: Coord.() -> String
    ): String {
        fun padTraverse(pos: Coord, c: Char) = pos
            .move(c)
            .let { newPos -> if (newPos.inBounds()) newPos else pos }

        fun processInstruction(pos: Coord, instruction: String) = instruction.fold(pos, ::padTraverse)

        return instructions
            .runningFold(start, ::processInstruction)
            .drop(1) // dump the starting coordinate as that is not part of the code
            .joinToString(separator = "", transform = conversion)
    }

    override fun part1(): String {
        val start = Coord(1, 1)
        fun Coord.inBounds(): Boolean = chebyshevDistance(start) < 2
        fun Coord.toNumpad(): String = (y * 3 + x + 1).toString()
        return solve(start, Coord::inBounds, Coord::toNumpad)
    }

    override fun part2(): String {
        val start = Coord(0, 2)
        fun Coord.inBounds(): Boolean = manhattanDistance(Coord(2, 2)) < 3
        fun Coord.toNumpad() = (5 + x + (y - 2) * 2 + 2 * (y - 2).sign).toString(16).uppercase()
        return solve(start, Coord::inBounds, Coord::toNumpad)
    }
}

fun main() = Day.runDay(Y16D02::class)

//    Class creation: 16ms
//    Part 1: 92435 (6ms)
//    Part 2: C1A88 (2ms)
//    Total time: 25ms
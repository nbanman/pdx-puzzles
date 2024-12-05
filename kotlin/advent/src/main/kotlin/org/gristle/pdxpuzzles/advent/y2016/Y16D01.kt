package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y16D01(input: String) : Day {

    private val moves: Sequence<Coord>

    init {
        var dir = Nsew.NORTH

        moves = input
            .splitToSequence(", ")
            .flatMap { instruction ->
                dir = if (instruction[0] == 'L') dir.left() else dir.right()
                List(instruction.getInts().first()) { dir }
            }.runningFold(Coord.ORIGIN, Coord::move)
    }

    override fun part1() = moves
        .last()
        .manhattanDistance()

    override fun part2(): Int {
        val visited = mutableSetOf<Coord>()
        return moves
            .first { !visited.add(it) }
            .manhattanDistance()
    }
}

fun main() = Day.runDay(Y16D01::class)

//    Class creation: 6ms
//    Part 1: 226 (3ms)
//    Part 2: 79 (4ms)
//    Total time: 14ms
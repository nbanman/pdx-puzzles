package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord

class Y19D03(input: String) : Day {

    private fun List<String>.wireUp(): List<Coord> = fold(mutableListOf()) { acc, instruction ->
        val last = if (acc.isNotEmpty()) acc.last() else Coord.ORIGIN
        val dir = when (instruction[0]) {
            'R' -> Nsew.EAST
            'L' -> Nsew.WEST
            'U' -> Nsew.NORTH
            'D' -> Nsew.SOUTH
            else -> throw IllegalArgumentException()
        }
        for (i in 1..instruction.drop(1).toInt()) acc.add(last.move(dir, i))
        acc
    }

    private val wiresInstructions = input.lines().map { it.split(',').wireUp() }

    private val intersections = wiresInstructions[0].intersect(wiresInstructions[1].toSet())

    override fun part1() = intersections.minOf(Coord::manhattanDistance)

    override fun part2(): Int {
        return intersections.minOf { coord ->
            wiresInstructions.sumOf { it.indexOf(coord) + 1 }
        }
    }
}

fun main() = Day.runDay(Y19D03::class)

//    Class creation: 106ms
//    Part 1: 266 (0ms)
//    Part 2: 19242 (31ms)
//    Total time: 137ms
package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord
import kotlin.math.sign

// Sequence-palooza!!
class Y22D09(input: String) : Day {

    // Sequence of positions that the head occupies, including any repeats
    private val headPositions: Sequence<Coord> = input
        .lineSequence()
        .flatMap { line -> // parse into a Sequence of directions, expanded so that "U 4" becomes 4 Nsew.NORTH entries
            val direction = Nsew.of(line[0])
            val times = line.takeLastWhile(Char::isDigit).toInt()
            List(times) { direction }
        }
        // take directions and turn them into a Sequence of positions that the head visits
        .runningFold(Coord.ORIGIN) { pos, direction -> pos.move(direction) }

    // from a Sequence of positions from the link ahead, provide a Sequence of positions that a following link takes
    private fun followPath(frontPositions: Sequence<Coord>): Sequence<Coord> = sequence {
        var pos = Coord.ORIGIN
        yield(pos)
        frontPositions.forEach { frontPos ->
            // only move if the link in front is at least 2 away on either the x- or the y-axis
            if (frontPos.chebyshevDistance(pos) > 1) {
                // move one toward the link in front, on both axes
                val diff = frontPos - pos
                pos = Coord(pos.x + diff.x.sign, pos.y + diff.y.sign)
                yield(pos)
            }
        }
    }

    // takes the initial sequence of positions, and adds subsequent links that follow the preceding sequence
    private fun solve(links: Int): Int = generateSequence(headPositions, ::followPath)
        .take(links)
        .last() // only interested in the last sequence representing the final link
        .toSet() // convert to set to remove duplicated positions
        .size

    override fun part1() = solve(2)
    override fun part2() = solve(10)
}

fun main() = Day.runDay(Y22D09::class)

//    Class creation: 23ms
//    Part 1: 6175 (36ms)
//    Part 2: 2578 (13ms)
//    Total time: 73ms
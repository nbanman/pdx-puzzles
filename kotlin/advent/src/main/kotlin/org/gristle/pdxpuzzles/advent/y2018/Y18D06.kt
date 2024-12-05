package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.MutableGrid
import org.gristle.pdxpuzzles.utilities.objects.getBounds
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import kotlin.math.max

class Y18D06(input: String) : Day {

    // coordinates from input, but offset so top left coordinate is normalized to (0,0) 
    private val coordinates: List<Coord>

    // width and height of starfield
    private val width: Int
    private val height: Int

    init {

        // "raw" coordinates before offset parsed from input
        val offsetCoordinates = input
            .getInts()
            .chunked(2) { (x, y) -> Coord(x, y) }
            .toList()

        val bounds = offsetCoordinates.getBounds()

        // calculate the offset
        val offset = Coord(-bounds.first.first, -bounds.second.first)

        // return offset coordinates
        coordinates = offsetCoordinates.map { it + offset }

        // return the width and height of the starfield
        width = bounds.first.last - bounds.first.first + 1
        height = bounds.second.last - bounds.second.first + 1
    }

    override fun part1(): Int {

        // array to track the closest coordinate to every space within the starfield. '-1' means unexplored.
        // '-2' means equally distant to at least two coordinates. Any other value is the index of the closest
        // coordinate.
        val space = MutableGrid(width, height) { -1 }

        // seed the space with the original coordinates
        coordinates.forEachIndexed { index, pos -> space[pos] = index }

        // helper for below ring function
        val sides = listOf(
            Coord(0, -1) to Coord(1, 1),
            Coord(1, 0) to Coord(-1, 1),
            Coord(0, 1) to Coord(-1, -1),
            Coord(-1, 0) to Coord(1, -1),
        )

        // function returns all coordinates that are distance t away from the coordinate
        fun Coord.ring(t: Int) = sides
            .flatMap { (initial, direction) ->
                (2..t).runningFold(this + Coord(initial.x * t, initial.y * t)) { pos, _ -> pos + direction }
            }

        // for each distance unit assign unassigned spaces to the closest coordinate 
        timeLoop@ for (distance in 1 until max(width, height)) {

            // track any assignments because if another coordinate also reaches the space at the same distance then
            // the assignment must be undone
            val distanceLog = mutableSetOf<Coord>()

            coordinates.forEachIndexed { index, pos -> // for each coordinate...
                pos
                    .ring(distance) // ...get the spaces reached at that distance
                    .filter { space.validCoord(it) } // remove spaces not in starfield (infinite space)
                    .forEach { ringPos ->
                        when (space[ringPos]) {
                            -1 -> { // when that space is unoccupied...
                                space[ringPos] = index // ...assign the index to it...
                                distanceLog.add(ringPos) // ...and add the assignment to the log
                            }

                            -2 -> {} // when a conflict has already been found, do nothing

                            // when an assignment has already been made at that space, undo it and put '-2' in its
                            // place to denote a conflict
                            else -> if (ringPos in distanceLog) space[ringPos] = -2
                        }
                    }
            }

            // break early if no new assignments were made
            if (distanceLog.isEmpty()) break@timeLoop
        }

        // 'infinite' stores those values that reach the edge of known space, and thus should not be counted
        val infinite = space.row(0).toSet() +
                space.row(space.height - 1) +
                space.column(0) +
                space.column(space.width - 1)

        return space // look at each space in the starfield
            .filter { it !in infinite } // remove those spaces assigned to 'infinite' coordinates
            .groupingBy { it }
            .eachCount() // count occurrences of the remainder
            .maxOf { (_, count) -> count } // return the largest 
    }

    // recreate indices of the starfield and count if the sum of all coordinate distances to that spot is less than
    // 10,000
    override fun part2() = (0 until width * height)
        .count { index -> coordinates.sumOf { it.manhattanDistance(Coord.fromIndex(index, width)) } < 10_000 }
}

fun main() = Day.runDay(Y18D06::class)

//    Class creation: 6ms
//    Part 1: 5365 (173ms)
//    Part 2: 42513 (50ms)
//    Total time: 230ms

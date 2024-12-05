package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.graph.Graph
import org.gristle.pdxpuzzles.utilities.math.md5
import org.gristle.pdxpuzzles.utilities.objects.contains

class Y16D17(private val salt: String) : Day {
    private val endPos: Coord = Coord(3, 3)

    // BFS sequence that explores the rooms. Used for both parts in different ways.
    private val explore: Sequence<Graph.Vertex<Pair<String, Coord>>> = let {

        // state for bfs is a pairing of the current passcode and current location
        val start = salt to Coord.ORIGIN

        // used to translate whether a door is open given a specific MD05 hash
        val openRange = 'b'..'f'

        // used for passIfInRooms fun below, so as not to require rebuilding each time
        val roomsDimensions = Coord.ORIGIN to endPos

        // takes a Coord, checks whether it's within the rooms, passes it through if so, otherwise returns null
        fun Coord.passIfInRooms(): Coord? = if (this in roomsDimensions) this else null

        // function that provides edges to the BFS sequence for each vertex
        fun getEdges(state: Pair<String, Coord>): List<Pair<String, Coord>> {
            val (passcode, pos) = state

            // If the node is at endPos, we do not want to return any neighbors. That path is completed. Relevant for
            // part 2 because we are not done when we first hit endPos.
            return if (pos == endPos) {
                emptyList()
            } else {

                // If the node is not at endPos, use MD05 on the passcode to determine which doors are open and return 
                // list of rooms that have open doors leading to them.
                passcode
                    .md5() // get hash
                    .take(4) // only look at first four characters
                    .mapIndexedNotNull { index, door -> // a "filter map" that generates edges for open doors only 
                        if (door in openRange) { // checks that the door is open
                            when (index) {

                                // for each door, calculate the position. If the position is out of bounds, return
                                // null; otherwise, generate new state with the position and corresponding passcode.
                                0 -> pos.north().passIfInRooms()?.let { passcode + 'U' to it }
                                1 -> pos.south().passIfInRooms()?.let { passcode + 'D' to it }
                                2 -> pos.west().passIfInRooms()?.let { passcode + 'L' to it }
                                3 -> pos.east().passIfInRooms()?.let { passcode + 'R' to it }
                                else -> throw IllegalStateException("Illegally evaluating hash beyond 4th character.")
                            }
                        } else {
                            null
                        }
                    }
            }
        }

        Graph.bfsSequence(startId = start, defaultEdges = ::getEdges)
    }

    override fun part1(): String = explore
        .first { it.id.second == endPos } // "second" refers to second element in state (pos). Run until == endpos 
        .id
        .first // "first" here refers to the first element in the state Pair. I.e, the passcode. 
        .drop(salt.length) // remove the salt from the passcode and return

    override fun part2(): Int = explore
        .last { it.id.second == endPos } // run until every combination has been tried
        .steps()
}

fun main() = Day.runDay(Y16D17::class)

//    Class creation: 5ms
//    Part 1: DDRUDLRRRD (13ms)
//    Part 2: 398 (117ms)
//    Total time: 136ms
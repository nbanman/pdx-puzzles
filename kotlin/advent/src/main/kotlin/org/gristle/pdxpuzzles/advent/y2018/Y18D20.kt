package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.graph.Graph
import org.gristle.pdxpuzzles.utilities.graph.Graph.steps

class Y18D20(private val input: String) : Day {

    // Build edge map for BFS function by parsing the input
    private val map: Map<Coord, List<Coord>> = buildMap<Coord, MutableList<Coord>> {

        // The input has multiple paths using a regex-style notation. Keep track of the paths in a stack.
        val returnCoords = mutableListOf<Coord>()

        // For a given coordinate, makes a move in accordance with the next char in the input. Processes the (|) 
        // by using the returnCoords stack. Builds out the edge map.
        fun move(pos: Coord, c: Char): Coord {
            val next = when (c) {
                'N' -> pos.north()
                'W' -> pos.west()
                'E' -> pos.east()
                'S' -> pos.south()
                '(' -> {
                    returnCoords.add(pos)
                    return pos
                }

                '|' -> return returnCoords.last()
                ')' -> {
                    returnCoords.removeLast()
                    return pos
                }

                else -> return pos
            }

            // Builds the edge map
            val to = getOrPut(pos) { mutableListOf() }
            if (next !in to) to.add(next)

            val from = getOrPut(next) { mutableListOf() }
            if (pos !in from) from.add(pos)
            return next
        }

        // Explores the map.
        input.asSequence().runningFold(Coord.ORIGIN, ::move).last()
    }

    // Standard BFS search of the space. Returns list of all vertices reached and their distance from origin.
    private val paths: List<Graph.Vertex<Coord>> by lazy { Graph.bfs(Coord.ORIGIN, edges = map) }

    override fun part1() = paths.steps()

    override fun part2() = paths.count { it.weight >= 1000 }
}

fun main() = Day.runDay(Y18D20::class)

//    Class creation: 19ms
//    Part 1: 3930 (30ms)
//    Part 2: 8240 (1ms)
//    Total time: 52ms
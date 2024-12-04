package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.graph.Graph
import org.gristle.pdxpuzzles.utilities.graph.Graph.steps
import org.gristle.pdxpuzzles.utilities.math.lcm
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.MutableGrid
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y22D24(input: String) : Day {

    private val valley = input.toGrid()

    // Rather than calculate specific coordinates, Blizzards are stored in a map that looks up by one coordinate
    // so the object only needs to supply the other coordinate. Locations for time are calculated on the fly as this
    // is very cheap to do.
    data class Blizzard(val initial: Int, val direction: Int, val valleySize: Int) {
        fun locationAt(n: Int) = (initial - 1 + (direction * n)).mod(valleySize) + 1
    }

    // The grid is larger than the valley, so this is used to calculate Blizzard behavior.
    private val valleySize = Coord(valley.width - 2, valley.height - 2)

    // Parse Blizzards into a map with one coordinate as the key and a list of Blizzards moving along that coordinate
    // on the other. The north/south blizzards use an offset the size of the valley's width so that one map can be
    // used for both coordinates.
    private val blizzards: Map<Int, List<Blizzard>> = buildMap<Int, MutableList<Blizzard>> {
        valley.forEachIndexed { index, c ->
            val pos = Coord.fromIndex(index, valley.width)
            val (movement, eastWest) = when (c) {
                '^' -> -1 to false
                'v' -> 1 to false
                '<' -> -1 to true
                '>' -> 1 to true
                else -> null to true
            }
            if (movement != null) {
                if (eastWest) {
                    this.getOrPut(pos.y) { mutableListOf() }
                        .add(Blizzard(pos.x, movement, valleySize.x))
                } else {
                    this.getOrPut(pos.x + valley.width) { mutableListOf() }
                        .add(Blizzard(pos.y, movement, valleySize.y))
                }
            }
        }
    }

    // State is custom rather than just the position because we have to do logic related to the minute as well to
    // calculate Blizzard locations. 
    data class State(val pos: Coord, val minute: Int)

    // Beginning and goal positions are the only wall openings.
    private val beginning = Coord.fromIndex(valley.indexOfFirst { it == '.' }, valley.width)
    private val goal = Coord.fromIndex(valley.indexOfLast { it == '.' }, valley.width)

    // Cached Blizzard values
    private val interval = lcm((valley.width - 2).toLong(), (valley.height - 2).toLong()).toInt()
    private val blizzardMap = buildMap {
        for (minute in 0 until interval) {
            val blizzardLocations = MutableGrid(valley.width, valley.height) { false }
            blizzards.entries.forEach { (firstPos, blizzardList) ->
                if (firstPos >= valley.width) {
                    val x = firstPos - valley.width
                    blizzardList.forEach { blizzard ->
                        val y = blizzard.locationAt(minute)
                        blizzardLocations[x, y] = true
                    }
                } else {
                    blizzardList.forEach { blizzard ->
                        val x = blizzard.locationAt(minute)
                        blizzardLocations[x, firstPos] = true
                    }
                }
            }
            put(minute, blizzardLocations)
        }
    }

    // A* algorithm
    private fun traverse(startPos: Coord, endPos: Coord, minute: Int): Int {
        val beginning = State(startPos, minute + 1)
        val defaultEdges = { (pos, minute): State ->

            Coord.CROSS.map { it + pos }
                .filter { candidate ->
                    // Checks that the candidate is 1) in the valley; 2) not a wall; and 3) no blizzards are moving 
                    // into the candidate space. 
                    valley.validCoord(candidate)
                            && valley[candidate] != '#'
                            && !blizzardMap.getValue(minute % interval)[candidate]
                }.map { Graph.Edge(State(it, minute + 1), 1.0) }
        }

        val distances = Graph.aStar(
            startId = beginning,
            heuristic = { state -> state.pos.manhattanDistance(endPos).toDouble() },
            defaultEdges = defaultEdges
        )
        return distances.steps()
    }

    override fun part1(): Int = traverse(beginning, goal, 0)

    override fun part2(): Int {
        val firstTrip = traverse(beginning, goal, 0)
        val secondTrip = traverse(goal, beginning, firstTrip)
        val thirdTrip = traverse(beginning, goal, firstTrip + secondTrip)
        return firstTrip + secondTrip + thirdTrip
    }
}

// Tested:
// 1. For each minute in a repeating interval (LCM of width and height that blizzards travel), calculate location
//    of all Blizzards and store in a map. (fastest)
// 2. Calculate blizzard spots on-the-fly, only looking at the row and column that the elves are on (medium)
// 3. #2, except with BFS instead of A* (slowest)
// Pt 1: 277 (1. 140ms) (2. 251ms) (3. 410ms)
// Pt 2: 877 (1. 235ms) (2. 469ms) (3. 929ms)
// Total: (1. 520ms) (2. 755ms) (3. 1375ms)
fun main() = Day.runDay(Y22D24::class)
package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.graph.Graph
import org.gristle.pdxpuzzles.utilities.graph.Graph.steps
import org.gristle.pdxpuzzles.utilities.parsing.getIntList

class Y18D22(input: String) : Day {

    private val start = Coord.ORIGIN

    private val depth: Int
    private val target: Coord

    init {
        val (d, x, y) = input.getIntList()
        depth = d
        target = Coord(x, y)
    }

    // the type of terrain found in each location of the cavern
    enum class Terrain { ROCKY, WET, NARROW }

    /**
     * Object used to provide the terrain of any position in the cavern. Internally it uses a mutable map to hold
     * geologic index values. These values are calculated lazily upon request. Prior implementation used a Grid
     * that calculated all values in advance. This was faster, but was a less general solution because the Grid
     * required padding on the right and bottom to allow the explorer to overshoot the target in search of more
     * efficient terrain to navigate. This version is theoretically limited only by the number of elements allowed
     * in a Map object.
     */
    class Cavern(start: Coord, target: Coord, private val depth: Int) {

        // helper functions used to convert geologic index to erosion level
        private fun Int.erosionLevel() = (this + depth) % 20183
        private fun Long.erosionLevel() = ((this + depth) % 20183).toInt()

        // helper function used to convert geologic index to terrain 
        private fun Int.terrain() = Terrain.entries[this % 3]

        // map to hold the erosion level of the cavern. From this, the terrain for any location can be
        // calculated. It remains mutable because the values are calculated lazily as needed using the getErosion
        // function. It starts with start and target values provided in the instructions.
        private val erosionMap = mutableMapOf<Coord, Int>()
            .apply {
                put(start, 0.erosionLevel())
                put(target, 0.erosionLevel())
            }

        // returns the erosion for a given position, using the erosionMap if a value is already found, otherwise
        // calculating it and putting it in the erosionMap
        private fun getErosion(pos: Coord): Int = erosionMap
            .getOrPut(pos) { // if value stored in map, return value. Otherwise...
                when { // ...calculate value and assign it to the map using provided rules
                    pos.y == 0 -> (pos.x * 16807L).erosionLevel()
                    pos.x == 0 -> (pos.y * 48271L).erosionLevel()
                    else -> (getErosion(pos.west()) * getErosion(pos.north())).erosionLevel() // recursive!
                }
            }

        // sole public getter provides the terrain for a given position
        operator fun get(pos: Coord): Terrain = getErosion(pos).terrain()
    }

    // initialize the cavern
    private val cavern = Cavern(start, target, depth)

    // for each position in the rectangle formed by the start and the target, add up the risk level
    override fun part1() = Coord.rectangleFrom(start, target).sumOf { cavern[it].ordinal }

    enum class Tool { GEAR, TORCH, NEITHER }

    data class State(val pos: Coord, val tool: Tool)

    /**
     * For a given proposed new location and current state, provides what tool will have to be used and the
     * time that it will take to change any tool, if necessary.
     */
    private fun changeTool(neighborTerrain: Terrain, state: State): Tool {
        val stateTerrain = cavern[state.pos]
        // if terrain is the same, no tool change needed 
        return if (stateTerrain == neighborTerrain) {
            state.tool
        } else {
            // if terrain is different, provide the tool that works for those two terrains by adding the ordinal values of the terrain
            // together to obtain a unique value specific to that terrain combination.
            Tool.entries[stateTerrain.ordinal + neighborTerrain.ordinal - 1]
        }
    }

    /**
     * Run A* algorithm to find the shortest path, using a State object that tracks both the position and the current
     * tool. The heuristic is the manhattan distance to the target. Edges are found by looking at neighboring
     * positions and calculating what tool change, if necessary, needs to be made.
     */
    override fun part2(): Int {
        val findEdges: (State) -> List<Graph.Edge<State>> = { state ->
            state.pos
                .getNeighbors() // get four neighbors of the location as grid indices
                .filter { it.x >= 0 && it.y >= 0 } // avoid negative positions
                .map { neighbor -> // for each neighbor index...
                    val newTool = changeTool(cavern[neighbor], state) // ...get the new tool if necessary 
                    val weight = if (state.tool != newTool) 8.0 else 1.0 // calculate weight incl tool change
                    // add 7 weight if the neighbor is the target and the target is not Torch, to account for
                    // ending up as torch
                    val endMod = if (neighbor == target && newTool != Tool.TORCH) 7 else 0
                    // wrap together in an Edge object
                    Graph.Edge(State(neighbor, newTool), weight + endMod)
                }
        }

        return Graph
            .aStar(
                startId = State(start, Tool.TORCH),
                heuristic = { state -> state.pos.manhattanDistance(target).toDouble() },
                defaultEdges = findEdges
            ).steps()
    }
}

fun main() = Day.runDay(Y18D22::class)

//    Class creation: 13ms
//    Part 1: 5637 (13ms)
//    Part 2: 969 (983ms)
//    Total time: 1010ms
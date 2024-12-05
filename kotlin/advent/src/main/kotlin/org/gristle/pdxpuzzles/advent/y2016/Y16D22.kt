package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.graph.Graph
import org.gristle.pdxpuzzles.utilities.graph.Graph.steps
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.contains
import org.gristle.pdxpuzzles.utilities.objects.getBounds
import org.gristle.pdxpuzzles.utilities.parsing.getInts

private typealias State = Pair<Coord, Coord>

class Y16D22(input: String) : Day {

    // Parsing input to Nodes
    class Node(val coord: Coord, val size: Int, val used: Int) {
        val available = size - used
    }

    private val nodes = input
        .getInts()
        .chunked(6) { (x, y, size, used, _) ->
            Node(Coord(x, y), size, used)
        }.toList()

    override fun part1() = nodes
        .filter { it.used != 0 }
        .sumOf { nodeA -> nodes.count { it != nodeA && it.available >= nodeA.used } }

    /**
     * Double-nested A* implementation. The problem requires moving certain "goal data" from its original location
     * to the top-left "origin." It can only move by swapping positions with an "empty spot." The empty spot can swap
     * positions within the bounds of the nodes, but there are some spots with too much data for swapping. These are
     * effectively "walls."
     *
     * The "outer" A* heuristic is the manhattan distance * 5 from the goal data to the origin, because the best-case
     * scenario^ takes four moves to get the empty spot in front of the goal data, and one move to move into the empty
     * spot. The state tracks both the goal data location and the empty spot.
     *
     * The "outer" edges are spots adjacent to the goal data within the bounds of the Node space, excluding "walls."
     *
     * The actual weight of each edge needs to be calculated. I used A* for this "inner" calculation. This time the
     * heuristic is simply manhattan distance from the empty spot to the edge location. The "inner" edges are spots
     * adjacent to the empty spot, excluding walls and the goal data itself.
     *
     * ^: Not truly a best-case scenario, if the empty spot is already adjacent to the goal data. This is an edge case I
     * leave as an exercise to the next coder.
     */
    override fun part2(): Int {

        // looks through the nodes and finds the boundaries, puts them into top-left and bottom-right Coords.
        val bounds = nodes.map(Node::coord).getBounds().let { (xR, yR) -> Coord.ORIGIN to Coord(xR.last, yR.last) }

        // finds positions of nodes that cannot be moved.
        val walls = nodes.filter { it.used > 400 }.map(Node::coord)

        // utility function for finding edges, used in both inner and outer A* implementations.
        fun Coord.adjacent(exclude: Coord? = null) = getNeighbors()
            .filter { it in bounds && it !in walls && it != exclude }

        // the data that needs to be moved to the origin, location defined by the problem
        val goalData = Coord(bounds.second.x, 0)

        // the node without any data in it
        val empty = nodes.find { it.used == 0 }?.coord ?: throw IllegalArgumentException("No empty node found!")

        // the "outer" heuristic
        val heuristic: (State) -> Double = { (pos, _) ->
            pos.manhattanDistance(Coord.ORIGIN) * 5.0
        }

        // finds edges for the "outer" A*. Calls an "inner" A* to find the weight for those edges.
        val move: (State) -> List<Graph.Edge<State>> = { (pos, empty) ->
            pos
                .adjacent()
                .map { neighbor ->
                    val weight = let {

                        // inner heuristic is manhattan distance. No 5x factor needed because it is simple movement.
                        val innerHeuristic = { innerPos: Coord -> innerPos.manhattanDistance(neighbor).toDouble() }

                        // edges for the inner A* uses same Coord.adjacent() function, but passes the goalData location
                        // as a coordinate to exclude from the list of edges.
                        val innerMove = { innerPos: Coord -> innerPos.adjacent(pos).map { Graph.Edge(it, 1.0) } }

                        // runs A* to find the shortest distance (ie, weight) from the current empty spot to the spot
                        // where the goal data will move.
                        Graph.aStar(
                            startId = empty,
                            heuristic = innerHeuristic,
                            defaultEdges = innerMove,
                        ).steps()
                    } + 1.0 // add 1 to move the goal data into the empty spot

                    // the empty spot is now where the goal data used to be
                    Graph.Edge(neighbor to pos, weight)
                }
        }

        // Runs the outer A*!
        return Graph.aStar(
            startId = goalData to empty,
            heuristic = heuristic,
            defaultEdges = move,
        ).steps()
    }
}

fun main() = Day.runDay(Y16D22::class)

//Class creation: 28ms
//Part 1: 924 (19ms)
//Part 2: 213 (16ms)
//Total time: 64ms
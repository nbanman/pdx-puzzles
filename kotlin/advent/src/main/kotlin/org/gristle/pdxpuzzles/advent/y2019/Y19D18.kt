package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.algorithms.Graph
import org.gristle.pdxpuzzles.utilities.algorithms.Graph.steps
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import org.gristle.pdxpuzzles.utilities.objects.toMutableGrid

class Y19D18(input: String) : Day {

    /**
     * Used for the edgemap, contains enhanced edge data. This includes the location of the edge, but also all
     * keys needed to be collected to make the move (precedingKeys) as well as all keys that are encountered while
     * traveling to the key (interveningKeys). precedingKeys is used to filter out moves that are not yet possible
     * due to not having the right key to pass through a door. interveningKeys is used to update the state so that
     * all keys are added in one swoop.
     */
    data class Key(val currentKey: Char, val precedingKeys: Set<Char>, val interveningKeys: Set<Char>)

    /**
     * Used for the main Dijkstra search. Contains the current position of each robot (location) and all keys that
     * have been collected up to that point (keys).
     */
    data class State(val location: List<Char>, val keys: Set<Char>)

    private val tunnels = input.toGrid()

    fun solve(tunnels: Grid<Char>, starts: String): Int {

        // Intermediate edge map used to later make the keyEdges edge map used for the final solution. Just gives
        // the distance to the nearest keys and doors. Once a key or door is reached, the BFS does not explore 
        // beyond that key or door. But it will still look for other keys or doors that do not require going through
        // that key or door.
        val basicEdges: Map<Char, List<Graph.Edge<Char>>> = buildMap {
            tunnels // take input...
                .withIndex() // associate each value with its index...
                .filter { (_, value) -> value.isLetter() || value in starts } // only run for important positions
                .forEach { (index, start) ->

                    // lambda for getting edges from the grid
                    val getEdges: (IndexedValue<Char>) -> List<IndexedValue<Char>> = { (index, current) ->

                        // If a key or door is reached, end this particular path.
                        if (current.isLetter() && current != start) {
                            emptyList()
                        } else { // ...otherwise keep exploring
                            tunnels
                                .getNeighborsIndexedValue(index) // get adjacent values
                                .filter { it.value != '#' } // remove walls
                        }
                    }

                    // populate edge map with all key and door locations that do not require passing through a 
                    // different key or door to get to
                    this[start] = Graph.bfs(IndexedValue(index, start), defaultEdges = getEdges)
                        .drop(1)
                        .filter { it.id.value.isLetter() } // only look at keys or doors
                        .map { Graph.Edge(it.id.value, it.weight) } // convert to an Edge
                }
        }

        // Edge map used for the main Dijkstra search. Itself uses a Dijkstra search relying on the basic edge map.
        val keyEdges: Map<Char, List<Graph.Edge<Key>>> = basicEdges
            .keys
            .filter { it.isLowerCase() || it in starts }
            .associateWith { key -> // make edge map
                Graph
                    .dijkstraSequence(key, edges = basicEdges)
                    .drop(1) // first edge returned is the start, so drop that 
                    .filter { it.id.isLowerCase() } // only look at key locations
                    .map { distance ->

                        // get the full path to key, dropping the start
                        val path = distance.path().drop(1)

                        // separate doors from keys
                        val (keyList, interveningKeyList) = path.partition { it.id.isUpperCase() }
                        val keys = keyList
                            .map { it.id.lowercaseChar() }
                            .toSet()
                        val interveningKeys = interveningKeyList
                            .map { it.id }
                            .toSet()

                        // create edge with metadata
                        Graph.Edge(Key(path.last().id, keys, interveningKeys), path.last().weight)
                    }.toList()
            }

        // initial state for the main Dijkstra algorithm
        val startState = State(starts.toList(), emptySet())

        // continue Dijkstra until all keys have been collected
        val endCondition = { state: State -> state.keys.size == 26 }

        // lambda finds edges by looking at each robot position, looking where each could move, and generating
        // new states for each possible move
        val findEdges = { state: State ->
            state.location.flatMapIndexed { robot, location ->
                keyEdges
                    .getValue(location) // gets all edges for location
                    .filter { keyEdge -> // keeps if key is new to state and all prerequisite keys collected 
                        keyEdge.vertexId.currentKey !in state.keys
                                && state.keys.containsAll(keyEdge.vertexId.precedingKeys)
                    }.map { edge -> // converts valid edge to state

                        // updates robot locations
                        val newLocations: List<Char> =
                            state.location.toMutableList().apply { this[robot] = edge.vertexId.currentKey }

                        // updates keys collected
                        val newKeys = state.keys union edge.vertexId.interveningKeys
                        val newState = State(newLocations, newKeys)
                        Graph.Edge(newState, edge.weight)
                    }
            }
        }

        return Graph.dijkstra(startState, endCondition, defaultEdges = findEdges).steps()
    }

    override fun part1() = solve(tunnels, "@")

    override fun part2(): Int {
        val robots = "@$%^"

        // Change the grid so that there are 4 quadrants
        val quadrants: Grid<Char> = tunnels.toMutableGrid().apply {
            val originalStart = indexOf('@')
            val wallIndices = getNeighborIndices(originalStart)
            val newStartIndices = getNeighborIndices(originalStart, true) - wallIndices.toSet()
            this[originalStart] = '#'
            wallIndices.forEach { this[it] = '#' }
            newStartIndices.forEachIndexed { index, i -> this[i] = robots[index] }
        }
        return solve(quadrants, robots)
    }
}

fun main() = Day.runDay(Y19D18::class)

//    Class creation: 4ms
//    Part 1: 3918 (123ms)
//    Part 2: 2004 (141ms)
//    Total time: 270ms
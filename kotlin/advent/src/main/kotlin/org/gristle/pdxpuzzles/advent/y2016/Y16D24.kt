package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.graph.Graph
import org.gristle.pdxpuzzles.utilities.objects.getEdgeMap
import org.gristle.pdxpuzzles.utilities.objects.toGrid

/**
 * Refactored; faster and cleaner!
 *
 * The original used BFS to find the distances between each number, found all the possible combinations for visiting
 * the numbers, calculated the total distance for each combination, and took the minimum. This is essentially the
 * same as a BFS for the second stage, which is far from optimal.
 *
 * I had an intermediary solution that just used BFS in one stage. It was very clean, but ran almost one second
 * slower than the OG version.
 *
 * I had another intermediary solution that calculated naive weighted edges between all numbers using DFS, then fed
 * that into a Dijkstra search. The "location" tracked not only the current position, but all numbers visited.
 * This was enough information to provide appropriate end conditions for both parts 1 and 2.
 *
 * My latest solution uses the Grid<Char>getEdgeMap function, which uses BFS to generate the
 * weighted edge map for all important locations.
 */

class Y16D24(input: String) : Day {
    // Read map
    private val layout = input.toGrid()

    // Find the numbers in the map and associate it with their location
    private val numbers = layout.withIndex().filter { it.value.isDigit() }

    // Naive edge map providing distance from any given number to all the other numbers. Naive in the sense that
    // the Dijkstra algo does not use it directly because we need to generate the edges on the fly in order to 
    // track the numbers already visited.    
    private val edgeMap = layout.getEdgeMap()

    // "State" tracks where the search is currently at and what numbers have been visited.
    data class State(val location: Char, val numbersVisited: Set<Char>)

    // Both parts have the same start: at '0', thus having already visited '0'
    private val start = State('0', setOf('0'))

    // Function to plug into Dijkstra that takes the edges from the edgemap and massages them to include all
    // the State data.
    private val getEdges = { state: State ->
        edgeMap[state.location]
            ?.map { Graph.Edge(State(it.vertexId, state.numbersVisited + it.vertexId), it.weight) }
            ?: throw IllegalStateException("Dijkstra search reached location that is not in the edgemap.")
    }

    private val explore = Graph.dijkstraSequence(start, defaultEdges = getEdges)

    // Runs dijkstra and provides weight of the shortest path. Takes in different end conditions to accommodate 
    // parts 1 & 2.
    fun solve(endCondition: (State) -> Boolean) = explore
        .first { endCondition(it.id) }
        .steps()

    // Part one ends when all numbers have been visited
    override fun part1() = solve { it.numbersVisited.size == numbers.size }

    // Part two ends when all numbers have been visited AND the robot has gone back to '0'
    override fun part2() = solve { it.location == '0' && it.numbersVisited.size == numbers.size }
}

fun main() = Day.runDay(Y16D24::class)

//Class creation: 95ms
//Part 1: 470 (9ms)
//Part 2: 720 (12ms)
//Total time: 116ms

// (218ms OG) (370ms BFS) (133ms 2-stage DFS-Dijk)
// (36ms OG) (638ms BFS) (10ms 2-stage DFS-Dijk)
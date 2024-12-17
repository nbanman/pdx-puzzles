package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.graph.Graph
import org.gristle.pdxpuzzles.utilities.graph.Graph.StdVertex
import org.gristle.pdxpuzzles.utilities.iteration.pollUntil
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import java.util.PriorityQueue

class Y24D16(input: String) : Day {
    private val maze = input.toGrid()
    private val start = State(maze.coordOfElement('S'), Nsew.EAST)
    private val end = maze.coordOfElement('E')
    private val cache = mutableMapOf<State, List<Graph.Edge<State>>>()
    private fun getEdges(state: State): List<Graph.Edge<State>> = cache.getOrPut(state) {
        val (pos, dir) = state
        listOf(dir, dir.left(), dir.right())
            .map { State(pos.move(it), it) }
            .filter { (p, _) -> maze[p] != '#' }
            .map { state ->
                val weight = if (state.dir == dir) 1.0 else 1001.0
                Graph.Edge(state, weight)
            }
    }
    data class State(val pos: Coord, val dir: Nsew)
    override fun part1(): Int {
        return Graph
            .dijkstraSequence(startId = start, defaultEdges = ::getEdges)
            .first { it.id.pos == end }
            .steps()
    }
    override fun part2(): Int {
        val start = StdVertex(start, 0.0)
        val q = PriorityQueue<Graph.Vertex<State>>()
        q.add(start)
        val weights = mutableMapOf(start.id to start.weight)
        val visited = mutableMapOf<State, Graph.Vertex<State>>()
        var bestPath = Double.MAX_VALUE
        val extraSeats = mutableMapOf<State, MutableSet<Coord>>()
        while (true) {
            val current = q.pollUntil { visited[it.id] == null } ?: break
            if (current.weight > bestPath) break
            visited[current.id] = current
            if (current.id.pos == end) bestPath = current.weight
            for (neighborEdge in getEdges(current.id)) {
                val alternateWeight = current.weight + neighborEdge.weight
                val weight = weights.getOrDefault(neighborEdge.vertexId, Double.MAX_VALUE)
                if (alternateWeight < weight && alternateWeight <= bestPath) {
                    weights[neighborEdge.vertexId] = alternateWeight
                    q.add(StdVertex(neighborEdge.vertexId, alternateWeight, current))
                }
                if (alternateWeight == weight) {
                    extraSeats.getOrPut(neighborEdge.vertexId) { mutableSetOf() }
                        .addAll(current.path().map { it.id.pos })
                }
            }
        }
        return visited
            .values
            .filter { (id, weight) -> id.pos == end && weight <= bestPath }
            .flatMap { vtx ->
                vtx.path().flatMap { extraSeats.getOrDefault(it.id, emptyList()) + it.id.pos }
            }.distinct()
            .size
    }
}

fun main() = Day.runDay(Y24D16::class)

//    Class creation: 12ms
//    Part 1: 105496 (48ms)
//    Part 2: 524 (59ms)
//    Total time: 120ms

@Suppress("unused")
private val test = listOf("""###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############""", """#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################""")
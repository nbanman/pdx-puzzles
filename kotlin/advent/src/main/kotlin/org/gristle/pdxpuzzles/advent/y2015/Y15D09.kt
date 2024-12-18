package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.algorithms.Graph

class Y15D09(input: String) : Day {

    // Parse input into adjacency map for Dijkstra search.
    private val edgeMap: Map<String, List<Graph.Edge<String>>> = buildMap<String, MutableList<Graph.Edge<String>>> {
        input.lineSequence().forEach { line ->
            val (cityA, _, cityB, _, distance) = line.split(' ')
            getOrPut(cityA) { mutableListOf() }.add(Graph.Edge(cityB, distance.toDouble()))
            getOrPut(cityB) { mutableListOf() }.add(Graph.Edge(cityA, distance.toDouble()))
        }
    }

    // The edgemap keys are a set of all the cities on the graph.
    private val cities: Set<String> = edgeMap.keys

    // The Dijkstra search does not use the adjacency map directly because the state object being tracked is the
    // complete list of cities, and repeat city visits are not allowed. This function translates the lookup and
    // applies the rule.
    private val defaultEdges: (List<String>) -> List<Graph.Edge<List<String>>> = { id: List<String> ->
        edgeMap[id.last()] // grabs edges connected to last city visited
            ?.filter { !id.contains(it.vertexId) } // filters out adjacent cities that have already been visited
            ?.map { Graph.Edge(id + it.vertexId, it.weight) } // provides an edge using the adjacency list data
            ?: throw IllegalStateException("City does not exist: ${id.last()}")
    }

    private fun tour(city: String): Sequence<Graph.Vertex<List<String>>> =
        Graph.dijkstraSequence(startId = listOf(city), defaultEdges = defaultEdges)

    // Part one finds the shortest path so the end condition is specified with first. It runs from each city
    // (hmm... Floyd-Warshall?) then takes the minimum of each city.
    override fun part1() = cities
        .minOf { city ->
            tour(city)
                .first { it.id.size == cities.size }
                .steps()
        }

    // Part two is the same as Part one except that it finds the longest path so the end condition is specified with
    // last. It runs until there are no more nodes to visit, then grabs the largest weight. This works because the
    // state object tracks the order that cities are visited, so no shortest path optimization is done. All paths are
    // checked.
    override fun part2() = cities
        .maxOf { city ->
            tour(city)
                .last { it.id.size == cities.size }
                .steps()
        }
}

fun main() = Day.runDay(Y15D09::class)

//    Class creation: 19ms
//    Part 1: 207 (23ms) (156ms old)
//    Part 2: 804 (176ms) (140ms old)
//    Total time: 219ms
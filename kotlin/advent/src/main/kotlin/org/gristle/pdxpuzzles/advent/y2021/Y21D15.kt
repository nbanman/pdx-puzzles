package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.algorithms.Graph
import org.gristle.pdxpuzzles.utilities.algorithms.Graph.steps
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y21D15(input: String) : Day {

    private val initialCavern = input.toGrid(Char::digitToInt)

    /**
     * Uses A* algorithm to find the shortest path from the start (index 0) to the end (lastIndex of Grid).
     * The locator id is the index in the Grid.
     * The heuristic is manhattan distance from the end.
     * The end position is implicitly determined by the heuristic function returning 0.
     * Rather than supply a graph, the defaultEdges function supplies edges on demand.
     */
    private fun shortestPath(cavern: Grid<Int>): Int {
        val heuristic = { i: Int -> cavern.coordOf(i).manhattanDistance(cavern.lastCoord()).toDouble() }
        val defaultEdges = { i: Int -> cavern.getNeighborIndices(i).map { Graph.Edge(it, cavern[it].toDouble()) } }
        return Graph.aStar(0, heuristic = heuristic, defaultEdges = defaultEdges).steps()
    }

    // Runs A* on the given cavern
    override fun part1() = shortestPath(initialCavern)

    // Expands cavern per part2 rules and runs A* on it.
    override fun part2(): Int {
        // Utility fun adds *n* to the existing risk, with 10 rolling back to 1.
        fun Int.addRisk(n: Int) = (this + n - 1) % 9 + 1

        // width and height of expanded cavern
        val expandedWidth = initialCavern.width * 5
        val expandedHeight = initialCavern.height * 5
        // create the expanded cavern
        val expandedCavern = Grid(expandedWidth, expandedHeight) { index ->
            // x and y coord of the cell being populated in the expanded cavern 
            val x = index % expandedWidth
            val y = index / expandedWidth

            // corresponding "base" x and y in the initial cavern
            val xBase = x % initialCavern.width
            val yBase = y % initialCavern.height

            // adds risk to the expanded parts of the cavern per part2 rules
            val addedRisk = x / initialCavern.width + y / initialCavern.height

            // populates expanded cavern by taking the risk at the base part of the cavern and adding risk
            // per part2 rules
            initialCavern[Coord(xBase, yBase)].addRisk(addedRisk)
        }
        // run A*
        return shortestPath(expandedCavern)
    }
}

// pt. 1: 602 (78ms Dij) (64 aStar)
// pt. 2: 2935 (461ms Dij) (327 aStar)
fun main() = Day.runDay(Y21D15::class)

//    Class creation: 20ms
//    Part 1: 602 (45ms)
//    Part 2: 2935 (371ms)
//    Total time: 437ms
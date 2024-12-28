package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.algorithms.Graph
import org.gristle.pdxpuzzles.utilities.algorithms.Graph.steps
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y24D21(input: String) : Day {
    private val codes = input.lines()
    private val doorKeys = "789456123-0E".toGrid(3)
    private val robotKeys = "-UALDR".toGrid(3)
    private val locations = buildMap {
        insertLocations(doorKeys)
        insertLocations(robotKeys)
    }

    private fun MutableMap<Char, Coord>.insertLocations(keys: Grid<Char>) {
        for ((idx, key) in keys.withIndex()) {
            this[key] = keys.coordOf(idx)
        }
    }

    data class State(
        val positions: List<Coord>,
        val level: Int,
        val goal: Int,
        val heuristic: Int
    ) : Comparable<State> {
        override fun compareTo(other: State): Int = this.heuristic - other.heuristic
    }

    private fun solve(robots: Int): Int =
        codes.sumOf { code -> code.dropLast(1).toInt() * code.presses(robots) }

    private fun String.presses(robots: Int): Int {
        val start = State(
            List(robots) { if (it == 0) locations.getValue('E') else locations.getValue('A') },
            0,
            0,
            0
        )
        val heuristic = { state: State ->
            0.0
        }
        val endCondition = { state: State -> state.goal == length }
        val edges = { state: State ->
            emptyList<Graph.Edge<State>>()
        }

        return Graph
            .aStar(
                start,
                heuristic,
                endCondition,
                defaultEdges = edges,
            ).steps()
    }


    override fun part1() = solve(3)

    override fun part2() = 3
}

fun main() = Day.runDay(Y24D21::class)

@Suppress("unused")
private val test = listOf("""029A
980A
179A
456A
379A
""")
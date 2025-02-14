package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.algorithms.Graph
import org.gristle.pdxpuzzles.utilities.algorithms.Graph.Edge
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import kotlin.math.min

object Y24D13 : Day {
    private data class State(val floor: Int, val pos: Int)

    private fun solve(input: String): Int {
        val chamber = input.toGrid()
        val start = chamber.indexOf('E')
        val getNeighbors = { state: State ->
            chamber
                .getNeighborsIndexedValue(state.pos)
                .filter { (_, c) -> c.isLetterOrDigit() }
                .map { (neighborPos, c) ->
                    val neighborFloor = c.digitToIntOrNull() ?: 0
                    val timeCost = 1 + min(
                        (neighborFloor - state.floor).mod(10),
                        (state.floor + 10 - neighborFloor).mod(10)
                    )
                    Edge(State(neighborFloor, neighborPos), timeCost.toDouble())
                }
        }
        return Graph.dijkstraSequence(startId = State(0, start), defaultEdges = getNeighbors)
            .first { vertex -> chamber[vertex.id.pos] == 'S' }
            .steps()
    }

    override fun part1(input: String): Int = solve(input)
    override fun part2(input: String): Int = solve(input)
    override fun part3(input: String): Int = solve(input)
}

fun main() = Day.runDay(Y24D13::class)

//    Quest 1: 135 (6ms)
//    Quest 2: 618 (13ms)
//    Quest 3: 521 (35ms)
//    Total time: 56ms
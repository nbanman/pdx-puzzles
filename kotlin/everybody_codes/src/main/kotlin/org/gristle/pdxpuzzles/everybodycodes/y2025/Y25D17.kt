package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.algorithms.Graph
import org.gristle.pdxpuzzles.utilities.algorithms.Graph.steps
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.StringGrid
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import kotlin.math.abs

object Y25D17 : Day {
    private fun Coord.inRange(other: Coord, radius: Int): Boolean {
        val dX = abs(x - other.x)
        val dY = abs(y - other.y)
        val p = dX * dX + dY * dY
        return p <= radius * radius
    }

    override fun part1(input: String): Int {
        val volcano = StringGrid(input)
        val centerIdx = volcano.string.indexOf('@')
        val center = Coord.fromIndex(centerIdx, volcano.width + 1)
        return volcano.string
            .filterIndexed { idx, ch ->
                ch != '@' && Coord.fromIndex(idx, volcano.width + 1).inRange(center, 10)
            }.sumOf { ch -> ch - '0' }
    }

    override fun part2(input: String): Int {
        val volcano = StringGrid(input)
        val centerIdx = volcano.string.indexOf('@')
        val center = Coord.fromIndex(centerIdx, volcano.width + 1)
        val r2 = (0 until center.x).map { it * it }
        val destruction = MutableList(r2.size) { 0 }
        for ((idx, ch) in volcano.string.withIndex()) {
            if (!ch.isDigit()) continue
            val pos = Coord.fromIndex(idx, volcano.width + 1)
            val dX = abs(center.x - pos.x)
            val dY = abs(center.y - pos.y)
            val p = dX * dX + dY * dY
            val d = ch - '0'
            val r = r2.binarySearch(element = p)
                .let {
                    if (it >= 0) {
                        it
                    } else {
                        abs(it + 1)
                    }
                }
            if (r < destruction.size) destruction[r] += d
        }
        return destruction.withIndex()
            .maxBy { (_, dest) -> dest }
            .let { (radius, dest) -> radius * dest }
    }

    override fun part3(input: String): Int {
        val sVolcano = StringGrid(input)
        val centerIdx = sVolcano.string.indexOf('@')
        val center = Coord.fromIndex(centerIdx, sVolcano.width + 1)
        val startIdx = sVolcano.string.indexOf('S')
        val start = Coord.fromIndex(startIdx, sVolcano.width + 1)
        val r2 = (0 until center.x).map { it * it }
        val volcano = input
            .mapIndexedNotNull { idx, ch ->
                if (ch == '\n') {
                    null
                } else {
                    val pos = Coord.fromIndex(idx, sVolcano.width + 1)
                    val dX = abs(center.x - pos.x)
                    val dY = abs(center.y - pos.y)
                    val p = dX * dX + dY * dY
                    val seconds = if (ch == 'S') 0 else ch - '0'
                    val r = r2.binarySearch(element = p)
                        .let {
                            if (it >= 0) {
                                it
                            } else {
                                abs(it + 1)
                            }
                        }
                    r to seconds
                }
            }.toGrid(sVolcano.width)

        var minSeconds = 0

        for (radius in 10 until (volcano.width - 1) / 2 - 1) {
            if (minSeconds <= 30 * (radius + 1) - 1) {
                val (success, seconds) = aStar(volcano, start, center, radius)
                if (success) {
                    return seconds * radius
                } else {
                    minSeconds = seconds
                }
            }
        }
        error("Unreachable")
    }

    private fun aStar(
        volcano: Grid<Pair<Int, Int>>,
        start: Coord,
        center: Coord,
        radius: Int
    ): Pair<Boolean, Int> {
        val heuristic: (Pair<Coord, Nsew>) -> Double = { (pos, phase) ->
            var pos = pos
            var h = 0
            if (phase == Nsew.EAST) {
                val target = center.move(phase, radius + 1)
                h += pos.manhattanDistance(target)
                pos = target
            }
            if (phase == Nsew.EAST || phase == Nsew.SOUTH) {
                val target = center.move(Nsew.SOUTH, radius + 1)
                h += pos.manhattanDistance(target)
                pos = target
            }
            if (phase != Nsew.NORTH) {
                val target = center.move(Nsew.WEST, radius + 1)
                h += pos.manhattanDistance(target)
                pos = target
            }
            (h + pos.manhattanDistance(start)).toDouble()
        }

        val endCondition: (Pair<Coord, Nsew>) -> Boolean = { (pos, phase) ->
            pos == start && phase == Nsew.NORTH
        }

        val getEdges: (Pair<Coord, Nsew>) -> List<Graph.Edge<Pair<Coord, Nsew>>> = { (pos, phase) ->
            volcano.getNeighborsIndexedValue(pos)
                .mapNotNull { (idx, value) ->
                    val adjPos = Coord.fromIndex(idx, volcano.width)
                    val (adjRad, adjSec) = value

                    // abort case 1
                    if (adjRad <= radius) return@mapNotNull null

                    // ac2
                    val backtracking = when (phase) {
                        Nsew.NORTH -> adjPos.y > center.y + 10
                        Nsew.SOUTH -> adjPos.y < center.y - 10
                        Nsew.EAST -> adjPos.x < center.x - 10
                        Nsew.WEST -> adjPos.x > center.x + 10
                    }
                    if (backtracking) return@mapNotNull null

                    val adjPhase = when (phase) {
                        Nsew.NORTH -> phase
                        Nsew.SOUTH -> if (adjPos.x == center.x) {
                            Nsew.WEST
                        } else {
                            phase
                        }
                        Nsew.EAST -> if (adjPos.y == center.y) {
                            Nsew.SOUTH
                        } else {
                            phase
                        }
                        Nsew.WEST -> if (adjPos.y == center.y) {
                            Nsew.NORTH
                        } else {
                            phase
                        }
                    }

                    Graph.Edge(adjPos to adjPhase, adjSec.toDouble())
                }
        }

        val seconds = Graph
            .aStar(
                start to Nsew.EAST,
                heuristic,
                endCondition,
                defaultEdges = getEdges,
            ).steps()
        val success = seconds <= 30 * (radius + 1) - 1
        return success to seconds
    }
}

fun main() = Day.runDay(Y25D17::class)

//    Quest 1: 1584 (1ms)
//    Quest 2: 66183 (7ms)
//    Quest 3: 42069 (171ms)
//    Total time: 181ms
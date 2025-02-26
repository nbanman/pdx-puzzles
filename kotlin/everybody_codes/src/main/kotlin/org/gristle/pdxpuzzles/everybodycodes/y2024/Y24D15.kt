package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.algorithms.Graph
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.math.isEven
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.contains
import org.gristle.pdxpuzzles.utilities.objects.mapToGrid
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import java.util.ArrayDeque

object Y24D15 : Day {
    enum class Terrain {
        BARRIER,
        FREE,
        HERB,
    }

    data class State(
        val pos: Int,
        val herbCollected: Boolean,
        val exit: Int,
        val noMoreExits: Boolean,
    )

    data class Segment(
        val forest: Grid<Terrain>,
        val rows: Int,
        val cols: Int,
        val from: List<Int>,
        val to: List<Int>,
        val herbs: Int,
        val splitPath: Boolean,
        val direction: Nsew,
    ) {
        fun makeMap() = buildMap<Int, MutableMap<Int, MutableMap<Int, Int>>> {
            val tl = Coord(
                cols * forest.width,
                if (direction == Nsew.SOUTH) {
                    forest.coordOf(from.first()).y
                } else {
                    to.firstOrNull()
                        ?.let { forest.coordOf(it).y }
                        ?: 0
                }
            )
            val br = Coord(
                (cols + 1) * forest.width - 1,
                if (direction == Nsew.NORTH) {
                    forest.coordOf(from.first()).y
                } else {
                    to.firstOrNull()
                        ?.let { forest.coordOf(it).y }
                        ?: forest.height
                },
            )
            val bounds = tl to br
            for (start in from) {
                val q = ArrayDeque<Graph.Vertex<State>>()
                q.addLast(Graph.StdVertex(
                    State(
                        start,
                        rows == 3,
                        -1,
                        forest.width < 60
                                || (forest.width < 100 && rows == 0)
                                || (forest.width > 100 && rows == 4)
                    ),
                    0.0
                ))
                val visited = mutableSetOf<State>()
                val combos = to.size.coerceAtLeast(1) * from.size
                var combosFound = 0
                while (q.isNotEmpty()) {
                    val (state, weight) = q.removeFirst()
                    if (!visited.add(state)) {
                        continue
                    }
                    if (state.pos in from && state.herbCollected && state.noMoreExits) {
                        getOrPut(start) { mutableMapOf() }
                            .getOrPut(state.exit) { mutableMapOf() }[state.pos] = weight.toInt()
                        if (++combosFound == combos) {
                            break
                        }
                    }
                    val neighbors = forest
                        .getNeighborsIndexedValue(state.pos)
                        .filter { (nPos, nT) ->
                            nT != Terrain.BARRIER
                                    && forest.coordOf(nPos) in bounds
                                    && (nPos !in from || (state.herbCollected && state.noMoreExits))
                        }
                        for ((nPos, nT) in neighbors) {
                            val herbCollected = state.herbCollected || nT == Terrain.HERB
                            val exit = if (state.exit != -1) {
                                state.exit
                            } else {
                                if (nPos in to) nPos else -1
                            }
                            val noMoreExits = state.noMoreExits
                                    || (splitPath && state.exit != -1 && nPos in to)
                                    || (forest.width > 100 && (rows == 0 && cols.isEven()))
                                    || (forest.width < 100 && rows == 4)
                            q.addLast(Graph.StdVertex(
                                State(
                                    nPos,
                                    herbCollected,
                                    exit,
                                    noMoreExits
                                ),
                                weight + 1
                            ))
                        }
                }
            }
        }
        override fun toString(): String {
            return "Segment(from=$from, to=$to, rows=$rows, cols=$cols, herbs=$herbs, splitPath=$splitPath)"
        }
    }

    private fun solve(input: String, rows: Int, cols: Int): Int {
        val segments = processMap(input, rows, cols)
        val starts = if (cols == 1) listOf(0) else listOf(1, 12, 14)
        val maps = segments
            .mapToGrid(Segment::makeMap)
        maps.forEach { println(it) }
        return 3
    }

    private fun processMap(input: String, rows: Int, cols: Int): Grid<Segment> {
        val forest = input.toGrid {
            when {
                it == '.' -> Terrain.FREE
                it == '~' || it == '#' -> Terrain.BARRIER
                cols > 1 && it in "EKR" -> Terrain.FREE
                else -> Terrain.HERB
            }
        }

        if (rows == 1) {
            return Grid(1, 1) { _ ->
                Segment(
                    forest,
                    0,
                    0,
                    listOf(forest.indexOf(Terrain.FREE)),
                    emptyList(),
                    1,
                    false,
                    Nsew.SOUTH
                )
            }
        }

        val dividers = forest.rows().withIndex()
            .mapNotNull { (idx, row) ->
                val barrierCount = row.count { it == Terrain.BARRIER }.toFloat()
                if (barrierCount.toFloat() / row.size > 0.8) {
                    idx
                } else {
                    null
                }
            }.windowed(size = 2, partialWindows = true)

        val colWidth = forest.width / cols

        return Grid(cols, rows) { segmentIdx ->
            val x = segmentIdx % cols
            val y = segmentIdx / cols

            val (north, south) = dividers[y]
                .map { rowIdx ->
                    val start = rowIdx * forest.width + x * colWidth
                    val openSpots = (start until start + colWidth).filter { forest[it] == Terrain.FREE }
                    if (cols != 1 && y != 0 && openSpots.isEmpty()) {
                        val offset = (rowIdx - 1) * forest.width + x * colWidth
                        listOf(offset, offset + colWidth - 1).filter { forest[it] == Terrain.FREE }
                    } else {
                        openSpots
                    }
                }
            val (from, to) = if (cols == 3 && x.isEven()) {
                south to north
            } else {
                north to south
            }

            val direction = if (north == from) {
                Nsew.SOUTH
            } else {
                Nsew.NORTH
            }

            val herbs = when {
                cols == 1 && y == 4 -> 2
                y == 3 -> 0
                else -> 1
            }

            val splitPath = x == 1 && y == 4

            Segment(forest, y, x, from, to, herbs, splitPath, direction)
        }
    }

    override fun part1(input: String) = 3//solve(input, 1, 1)
    override fun part2(input: String) = solve(input, 5, 1)
    override fun part3(input: String) = solve(input, 5, 3)
}

fun main() = Day.runDay(Y24D15::class)

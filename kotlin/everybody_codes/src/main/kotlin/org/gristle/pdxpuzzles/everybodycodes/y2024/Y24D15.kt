package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.algorithms.Graph
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.math.isEven
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.contains
import org.gristle.pdxpuzzles.utilities.objects.map
import org.gristle.pdxpuzzles.utilities.objects.mapToGrid
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import java.util.ArrayDeque

object Y24D15 : Day {
    enum class Terrain {
        BARRIER,
        FREE,
        HERB,
    }

    enum class PathStatus {
        NONE,
        HERBLESS,
        COMPLETE,
    }

    data class State(
        val pos: Int,
        val herbsCollected: List<Int>,
    )

    data class Segment(
        val forest: Grid<Terrain>,
        val x: Int,
        val y: Int,
        val from: List<Int>,
        val to: List<Int>,
        val herbs: Int,
        val direction: Nsew,
    ) {
        val segmentWidth = if (forest.width < 100) {
            forest.width
        } else {
            forest.width / 3
        }
        fun makeMap() = buildMap<Int, MutableMap<Int, MutableList<Double>>> {
            val tl = Coord(
                x * segmentWidth,
                if (direction == Nsew.SOUTH) {
                    forest.coordOf(from.first()).y
                } else {
                    if (to == from) {
                        0
                    } else {
                        to.firstOrNull()
                            ?.let { forest.coordOf(it).y }
                            ?: 0
                    }
                }
            )
            val br = Coord(
                (x + 1) * segmentWidth,
                if (direction == Nsew.NORTH) {
                    forest.coordOf(from.first()).y
                } else {
                    if (to == from) {
                        forest.height - 1
                    } else {
                        to.firstOrNull()
                            ?.let { forest.coordOf(it).y }
                            ?: (forest.height - 1)
                    }
                },
            )
            val bounds = tl to br
            for (start in from) {
                val exits = getOrPut(start) { mutableMapOf() }
                val paths = to
                    .associateWith {
                        if (exits.containsKey(it)) {
                            PathStatus.COMPLETE
                        } else {
                            PathStatus.NONE
                        }
                    }.toMutableMap()
                val q = ArrayDeque<Graph.Vertex<State>>()
                q.addLast(Graph.StdVertex(
                    State(start, emptyList()),
                    0.0
                ))
                val visited = mutableSetOf<State>()
                while (q.isNotEmpty()) {
                    val (state, weight) = q.removeFirst()
                    if (!visited.add(state)) {
                        continue
                    }
                    if (state.pos in to && paths.getValue(state.pos) != PathStatus.COMPLETE) {
                        when {
                            from == to -> {
                                if (state.herbsCollected.size == herbs) {
                                    paths[state.pos] = PathStatus.COMPLETE
                                    exits.getOrPut(state.pos) { mutableListOf() }
                                        .add(weight)
                                    if (paths.values.all { it == PathStatus.COMPLETE }) {
                                        break
                                    } else {
                                        continue
                                    }
                                }
                            }
                            paths[state.pos] == PathStatus.NONE -> {
                                paths[state.pos] = if (state.herbsCollected.size == herbs) {
                                    PathStatus.COMPLETE
                                } else {
                                    PathStatus.HERBLESS
                                }
                                exits.getOrPut(state.pos) { mutableListOf() }
                                    .add(weight)
                                getOrPut(state.pos) { mutableMapOf() }
                                    .getOrPut(start) { mutableListOf() }
                                    .add(weight)
                                if (paths.values.all { it == PathStatus.COMPLETE }) {
                                    break
                                } else {
                                    continue
                                }
                            }
                            else -> {
                                if (state.herbsCollected.size == herbs) {
                                    paths[state.pos] = PathStatus.COMPLETE
                                    exits.getOrPut(state.pos) { mutableListOf() }
                                        .add(weight)
                                    getOrPut(state.pos) { mutableMapOf() }
                                        .getOrPut(start) { mutableListOf() }
                                        .add(weight)
                                }
                                if (paths.values.all { it == PathStatus.COMPLETE }) {
                                    break
                                } else {
                                    continue
                                }
                            }
                        }
                    }
                    val neighbors = forest
                        .getNeighborsIndexedValue(state.pos)
                        .filter { (nPos, nT) ->
                            nT != Terrain.BARRIER && forest.coordOf(nPos) in bounds
                        }
                    for ((nPos, nT) in neighbors) {
                        val newHerb = state.herbsCollected.size < herbs && nT == Terrain.HERB && state.pos !in state.herbsCollected
                        val herbsCollected = if (newHerb) {
                            state.herbsCollected + state.pos
                        } else {
                            state.herbsCollected
                        }
                        q.addLast(Graph.StdVertex(
                            State(nPos, herbsCollected),
                            weight + 1
                        ))
                    }
                }
            }
        }

        override fun toString(): String {
            return "Segment(x=$x, y=$y, from=$from, to=$to, herbs=$herbs, direction=$direction)"
        }
    }

    private fun solve(input: String, rows: Int, cols: Int): Int {
        val segments = processMap(input, rows, cols)
        val starts = if (cols == 1) listOf(0) else listOf(1, 12, 14)
        val maps = segments.mapToGrid(Segment::makeMap)
        return starts.sumOf { start -> navigate(maps, start) }
    }

    private fun navigate(
        maps: Grid<Map<Int, MutableMap<Int, MutableList<Double>>>>,
        start: Int
    ): Int {
        return 3
    }

    private fun processMap(input: String, rows: Int, cols: Int): Grid<Segment> {
        val forest = input.toGrid {
            when {
                it == '.' -> Terrain.FREE
                it == '~' || it == '#' -> Terrain.BARRIER
                cols > 1 && it in "ER" -> Terrain.FREE
                else -> Terrain.HERB
            }
        }

        if (rows == 1) {
            val entrance = listOf(forest.indexOf(Terrain.FREE))
            return Grid(1, 1) { _ ->
                Segment(
                    forest,
                    0,
                    0,
                    entrance,
                    entrance,
                    1,
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
            }.zipWithNext()

        val colWidth = forest.width / cols

        return Grid(cols, rows) { segmentIdx ->
            val x = segmentIdx % cols
            val y = segmentIdx / cols

            val (north, south) = dividers[y]
                .map { rowIdx ->
                    val start = rowIdx * forest.width + x * colWidth
                    val openSpots = (start until start + colWidth)
                        .filter { forest[it] == Terrain.FREE }
                    if (cols != 1 && y != 0 && x != 1 && openSpots.isEmpty()) {
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
                x == 1 && y == 4 -> 3
                y == 3 -> 0
                else -> 1
            }

            val toFixed = if (to.isEmpty()) from else to

            Segment(forest, x, y, from, toFixed, herbs, direction)
        }
    }

    override fun part1(input: String) = solve(input, 1, 1)
    override fun part2(input: String) = solve(input, 5, 1)
    override fun part3(input: String) = 3//solve(input, 5, 3)
}

fun main() = Day.runDay(Y24D15::class)

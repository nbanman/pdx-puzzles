package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.graph.Graph
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.MutableGrid
import org.gristle.pdxpuzzles.utilities.objects.toMutableGrid
import java.util.*
import kotlin.math.max
import kotlin.math.min
import kotlin.properties.Delegates

class Y19D15(private val input: String) : Day {

    enum class Sector { UNEXPLORED, WALL, PATH, O2 }

    data class Droid(
        val grid: MutableGrid<Sector>,
        val initialState: List<Long>
    ) {
        private val toDroid: Deque<Long> = LinkedList()
        private val toComp: Deque<Long> = LinkedList()
        private val computer = IntCode("A", initialState, null, toComp, toDroid)

        fun explore(): Pair<Int, Int> {

            var minX = 500
            var maxX = 500
            var minY = 500
            var maxY = 500

            var coord: Coord by Delegates.observable(Coord(500, 500)) { _, _, newValue ->
                minX = min(minX, newValue.x)
                maxX = max(maxX, newValue.x)
                minY = min(minY, newValue.y)
                maxY = max(maxY, newValue.y)
            }
            val path = LinkedList<Int>()

            fun Nsew.toLong() = when (this) {
                Nsew.NORTH -> 1L
                Nsew.SOUTH -> 2L
                Nsew.WEST -> 3L
                Nsew.EAST -> 4L
            }

            fun move(direction: Nsew, goBack: Boolean = false): Sector {
                toComp.add(direction.toLong())
                computer.run()
                val response = when (toDroid.poll().toInt()) {
                    0 -> Sector.WALL
                    1 -> Sector.PATH
                    else -> Sector.O2
                }
                if (response != Sector.WALL) {
                    if (goBack) {
                        toComp.add(direction.right().right().toLong())
                        computer.run()
                        toDroid.poll()
                    } else {
                        coord = direction.forward(coord)
                    }
                }
                return response
            }

            var direction = Nsew.NORTH

            fun scan(): List<Nsew> {
                return Nsew.entries
                    .filter { grid[it.forward(coord)] == Sector.UNEXPLORED }
                    .filter {
                        val response = move(it, true)
                        grid[it.forward(coord)] = response
                        response != Sector.WALL
                    }
            }

            fun addPath(direction: Nsew) {
                val pathDir = when (direction) {
                    Nsew.NORTH -> 1
                    Nsew.SOUTH -> 2
                    Nsew.WEST -> 3
                    Nsew.EAST -> 4
                }
                path.add(pathDir)
            }

            fun newPath() {
                direction = direction.right().right()
                while (true) {
                    if (path.isEmpty()) return
                    val previousStep = path.removeLast()
                    if (previousStep == 0) {
                        if (grid[direction.right().forward(coord)] != Sector.WALL) {
                            direction = direction.right()
                        }
                        addPath(direction)
                        move(direction)
                        return
                    }
                    direction = when (previousStep) {
                        1 -> Nsew.SOUTH
                        2 -> Nsew.NORTH
                        3 -> Nsew.EAST
                        else -> Nsew.WEST
                    }
                    move(direction)
                }
            }

            fun proceed() {
                val validDirections = scan()
                if (validDirections.size > 1) path.add(0)
                direction = when {
                    direction.right() in validDirections -> direction.right()
                    direction in validDirections -> direction
                    direction.left() in validDirections -> direction.left()
                    else -> {
                        if (path.isNotEmpty()) {
                            newPath()
                            return
                        } else {
                            Nsew.SOUTH
                        }
                    }
                }
                addPath(direction)
                move(direction)
            }

            var steppes = 0
            var o2Coord = Coord(500, 500)
            do {
                proceed()
                if (grid[coord] == Sector.O2) {
                    o2Coord = coord
                    val hello = path.joinToString("") {
                        when (it) {
                            1 -> "N"
                            2 -> "S"
                            3 -> "W"
                            4 -> "X"
                            else -> "E"
                        }
                    }
                    var cc = 'X'
                    var count = 0
                    for (c in hello) {
                        if (c != cc) {
                            cc = c
                            count = 0
                        }
                        count++
                    }
                    steppes = path.count { it != 0 }
                }
            } while (path.isNotEmpty())
            val d2 = Graph.bfs(grid.indexOf(o2Coord)) { id ->
                grid
                    .getNeighborIndices(id)
                    .filter { grid[it] != Sector.WALL && grid[it] != Sector.UNEXPLORED }
            }
            return steppes to d2.maxOf { it.weight }.toInt()
        }
    }

    private fun solve(): Pair<Int, Int> {
        val initialState = input.split(',').map { it.toLong() }
        val grid = List(1_000_000) { Sector.UNEXPLORED }.toMutableGrid(1_000)
        val droid = Droid(grid, initialState)
        return droid.explore()
    }

    private val solution = solve()

    override fun part1() = solution.first

    override fun part2() = solution.second
}

fun main() = Day.runDay(Y19D15::class)

//    Class creation: 102ms
//    Part 1: 250 (0ms)
//    Part 2: 332 (0ms)
//    Total time: 103ms
package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord

object Y25D15 : Day {
    data class WallData(
        val hzWalls: Map<Int, List<Int>>,
        val vtWalls: Map<Int, List<Int>>,
        val hzDots: List<Int>,
        val vtDots: List<Int>,
        val start: Coord,
        val end: Coord,
    ) {
        companion object {
            fun from(input: String): WallData {
                val hzWalls = mutableMapOf<Int, MutableList<Int>>()
                val vtWalls = mutableMapOf<Int, MutableList<Int>>()
                val hzDotsSet = mutableSetOf<Int>()
                val vtDotsSet = mutableSetOf<Int>()
                var dir = Nsew.NORTH
                var turtle = Coord.ORIGIN
                var realEnd = Coord.ORIGIN

                val addDots: (Coord) -> Boolean = { pos: Coord ->
                    hzDotsSet.add(pos.x - 1)
                    hzDotsSet.add(pos.x + 1)
                    vtDotsSet.add(pos.y - 1)
                    vtDotsSet.add(pos.y + 1)
                }

                val commands = input.split(',')

                for ((idx, cmd) in commands.withIndex()) {
                    dir = when (cmd[0]) {
                        'L' -> dir.left()
                        'R' -> dir.right()
                        else -> error("cannot read command $cmd")
                    }

                    if (idx == 0) {
                        turtle = turtle.move(dir)
                        addDots(turtle)
                    }

                    var dist = cmd.drop(1).fold(0) { acc, c -> acc * 10 + (c - '0') }

                    if (idx == 0 || idx == commands.lastIndex) {
                        realEnd = turtle.move(dir, dist)
                        dist -= 1
                    }

                    val next = turtle.move(dir, dist)
                    addDots(next)

                    when (dir) {
                        Nsew.NORTH, Nsew.SOUTH -> {
                            val ranges = vtWalls.getOrPut(next.x) { mutableListOf() }
                            ranges.add(turtle.y)
                            ranges.add(next.y)
                        }
                        Nsew.EAST, Nsew.WEST -> {
                            val ranges = hzWalls.getOrPut(next.y) { mutableListOf() }
                            ranges.add(turtle.x)
                            ranges.add(next.x)
                        }
                    }
                    turtle = next
                }

                hzDotsSet.add(0)
                hzDotsSet.add(realEnd.x)
                vtDotsSet.add(0)
                vtDotsSet.add(realEnd.y)

                val hzDots = hzDotsSet.sorted()
                val vtDots = vtDotsSet.sorted()

                val start = Coord(
                    hzDots.binarySearch(element = 0),
                    vtDots.binarySearch(element = 0),
                )

                val end = Coord(
                    hzDots.binarySearch(element = realEnd.x),
                    vtDots.binarySearch(element = realEnd.y),
                )

                for (walls in hzWalls.values) {
                    walls.sort()
                }
                for (walls in vtWalls.values) {
                    walls.sort()
                }

                return WallData(
                    hzWalls,
                    vtWalls,
                    hzDots,
                    vtDots,
                    start,
                    end,
                )
            }
        }
    }

    data class State(val pos: Coord, val realPos: Coord, val weight: Int)

    fun shortestPath(input: String): Int {
        val wd = WallData.from(input)
        val start = State(wd.start, Coord.ORIGIN, 0)
        val visited = mutableSetOf<Coord>()
        visited.add(start.pos)
        var todo = mutableListOf(start)
        var next = mutableListOf<State>()

        while (todo.isNotEmpty()) {
            for ((pos, realPos, weight) in todo) {
                if (pos == wd.end) return weight
                for (adjPos in Nsew.entries.mapNotNull { dir -> movePos(pos, dir, wd) }) {
                    if (!visited.add(adjPos)) continue
                    val adjRealPos = getRealPos(adjPos, wd)
                    val adjWeight = weight + realPos.manhattanDistance(adjRealPos)
                    val adjState = State(adjPos, adjRealPos, adjWeight)
                    next.add(adjState)
                }
            }
            todo.clear()
            next = todo.also { todo = next }
        }
        error("Unreachable")
    }

    private fun getRealPos(pos: Coord, wd: WallData): Coord = Coord(wd.hzDots[pos.x], wd.vtDots[pos.y])

    private fun movePos(pos: Coord, dir: Nsew, wd: WallData): Coord? {
        val newX = when (dir) {
            Nsew.NORTH, Nsew.SOUTH -> pos.x
            Nsew.EAST -> {
                val newX = pos.x + 1
                if (newX == wd.hzDots.size) return null
                newX
            }
            Nsew.WEST -> {
                if (pos.x == 0) return null
                pos.x - 1
            }
        }

        val newY = when (dir) {
            Nsew.EAST, Nsew.WEST -> pos.y
            Nsew.SOUTH -> {
                val newY = pos.y + 1
                if (newY == wd.vtDots.size) return null
                newY
            }
            Nsew.NORTH -> {
                if (pos.y == 0) return null
                pos.y - 1
            }
        }

        val realPos = getRealPos(pos, wd)
        val oneOver = realPos.move(dir)
        val noHzConflict = checkConflict(oneOver.x, oneOver.y, wd.hzWalls)
        val noWallConflict = noHzConflict && checkConflict(oneOver.y, oneOver.x, wd.vtWalls)

        return if (noWallConflict) {
            Coord(newX, newY)
        } else {
            null
        }
    }

    private fun checkConflict(a: Int, b: Int, walls: Map<Int, List<Int>>): Boolean {
        return walls[b]
            ?.let {
                val search = it.binarySearch(element = a)
                if (search >= 0) {
                    false
                } else {
                    search and 1 == 1
                }
            }
            ?: true
    }

    override fun part1(input: String): Int = shortestPath(input)
    override fun part2(input: String): Int = shortestPath(input)
    override fun part3(input: String): Int = shortestPath(input)
}

fun main() = Day.runDay(Y25D15::class)

//    Quest 1: 110 (5ms)
//    Quest 2: 5053 (18ms)
//    Quest 3: 454681238 (28ms)
//    Total time: 52ms
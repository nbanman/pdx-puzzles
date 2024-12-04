package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.StringGrid

class Y23D16(input: String) : Day {

    private val grid = StringGrid(input)
    
    private fun next(c: Char, dir: Nsew): List<Nsew> {
        return when (c) {
            '.' -> listOf(dir)
            '|' -> when (dir) {
                Nsew.NORTH, Nsew.SOUTH -> listOf(dir)
                Nsew.EAST, Nsew.WEST -> listOf(Nsew.NORTH, Nsew.SOUTH)
            }
            '-' -> when (dir) {
                Nsew.NORTH, Nsew.SOUTH -> listOf(Nsew.EAST, Nsew.WEST)
                Nsew.EAST, Nsew.WEST -> listOf(dir)
            }
            '/' -> when (dir) {
                Nsew.NORTH -> listOf(Nsew.EAST)
                Nsew.EAST -> listOf(Nsew.NORTH)
                Nsew.SOUTH -> listOf(Nsew.WEST)
                Nsew.WEST -> listOf(Nsew.SOUTH)
            }
            '\\' -> when (dir) {
                Nsew.NORTH -> listOf(Nsew.WEST)
                Nsew.EAST -> listOf(Nsew.SOUTH)
                Nsew.SOUTH -> listOf(Nsew.EAST)
                Nsew.WEST -> listOf(Nsew.NORTH)
            }
            else -> throw IllegalArgumentException("$c not recognized as space or mirror")
        }
    }
    
    private fun next(state: Pair<Coord, Nsew>): List<Pair<Coord, Nsew>> {
        val (pos, dir) = state
        val nextPos = pos.move(dir)
        return if (nextPos.x in grid.xIndices && nextPos.y in grid.yIndices) {
            next(grid[nextPos], dir).map { nextDir -> nextPos to nextDir}
        } else {
            emptyList()
        }
    }
    
    private fun Pair<Coord, Nsew>.toIndex(): Int = ((first.y * grid.width + first.x) shl 2) + second.ordinal

    private fun lightBeam(state: Pair<Coord, Nsew>): Int {
        val visited = BooleanArray(grid.string.length * 4)
        val q = mutableListOf(state)
        while (q.isNotEmpty()) {
            val current = q.removeLast()
            next(current)
                .filter { nextState ->
                    val index = nextState.toIndex()
                    !visited[index].also { visited[index] = true }
                }.forEach { q.add(it) }
        }
        var count = 0 
        for (index in grid.string.indices) {
            for (dir in 0..3) {
                if (visited[(index shl 2) + dir]) {
                    count++
                    break
                }
            }
        }
        return count
    }

    override fun part1() = lightBeam(Coord(-1, 0) to Nsew.EAST)

    override fun part2(): Int {
        val states = (0 until grid.width).map { Coord(it, -1) to Nsew.SOUTH } +
                (0 until grid.width).map { Coord(it, grid.height) to Nsew.NORTH } +
                (0 until grid.height).map { Coord(-1, it) to Nsew.EAST } +
                (0 until grid.height).map { Coord(grid.width, it) to Nsew.WEST }
        return states.stream().parallel().map { lightBeam(it) }.max(Integer::compare).get()
    }
}

fun main() = Day.runDay(Y23D16::class)

//    Class creation: 13ms
//    Part 1: 7798 (24ms)
//    Part 2: 8026 (508ms)
//    Total time: 546ms

@Suppress("unused")
private val sampleInput = listOf(
    """.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
""",
)
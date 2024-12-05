package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.y2019.Intcode.IntCode
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.toMutableGrid
import java.util.*

class Y19D17(input: String) : Day {
    private val initialState = input.split(',').map { it.toLong() }

    fun solve(): Pair<Int, Long> {

        val toDroid: Deque<Long> = LinkedList()
        val toComp: Deque<Long> = LinkedList()
        val intCode = IntCode("A", initialState, null, toComp, toDroid)
        intCode.run()
        val width = toDroid.indexOfFirst { it == 10L }
        val grid = toDroid.mapNotNull { if (it != null && it != 10L) it else null }.toMutableGrid(width)
        val intersections = grid.mapIndexedNotNull { index, l ->
            if (l != 35L || grid.getNeighbors(index).any { it != 35L }) {
                null
            } else {
                grid.coordOf(index)
            }
        }.map { it.x * it.y}
        val p1 = intersections.sum()

        // Part 2
        var coord = grid.coordOfElement(94L)
        var dir = Nsew.NORTH
        var counter = 0
        val path = mutableListOf<String>()
        while (true) {
            if (cromulent(grid, coord, dir)) {
                counter++
                coord = dir.forward(coord)
            } else {
                if (cromulent(grid, coord, dir.left())) {
                    dir = dir.left()
                    if (counter != 0) path.add(counter.toString())
                    path.add("L")
                    counter = 0
                } else if (cromulent(grid, coord, dir.right())) {
                    dir = dir.right()
                    if (counter != 0) path.add(counter.toString())
                    path.add("R")
                    counter = 0
                } else {
                    path.add(counter.toString())
                    break
                }
            }
        }
        val formSeq = listOf('A', ',', 'B', ',', 'A', ',', 'C', ',', 'A', ',', 'A', ',', 'C', ',', 'B', ',', 'C', ',', 'B', '\n')
        val aForm = listOf('L', ',', '1', '2', ',', 'L', ',', '8', ',', 'R', ',', '1', '2', '\n')
        val bForm = listOf('L', ',', '1', '0', ',', 'L', ',', '8', ',', 'L', ',', '1', '2', ',', 'R', ',', '1', '2', '\n')
        val cForm = listOf('R', ',', '1', '2', ',', 'L', ',', '8', ',', 'L', ',', '1', '0', '\n', 'n', '\n')
        val commands = (formSeq + aForm + bForm + cForm).map { it.code.toLong() }
        toComp.addAll(commands)
        val intCodeB = IntCode("B", listOf(2L) + initialState.drop(1), null, toComp, toDroid)
        toDroid.clear()
        intCodeB.run()
        return p1 to toDroid.last()
    }

    private fun cromulent(grid: Grid<Long>, coord: Coord, dir: Nsew): Boolean {
        val prospect = dir.forward(coord)
        return (prospect.x in 0 until grid.width && prospect.y in 0 until grid.height) && grid[prospect] == 35L
    }

    private val solution = solve()

    override fun part1() = solution.first

    override fun part2() = solution.second
}

fun main() = Day.runDay(Y19D17::class)

//    Class creation: 115ms
//    Part 1: 10632 (0ms)
//    Part 2: 1356191 (0ms)
//    Total time: 116ms
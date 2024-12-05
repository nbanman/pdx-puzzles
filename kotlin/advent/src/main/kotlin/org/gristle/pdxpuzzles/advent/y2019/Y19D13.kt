package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.y2019.Intcode.IntCode
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.MutableGrid
import java.util.*

class Y19D13(input: String) : Day {

    private val initialState = input.split(',').map(String::toLong)

    data class Cabinet(
        val grid: MutableGrid<Int>,
        val input: Deque<Long>,
        val output: Deque<Long>
    ) {
        var score = 0
        private var ballPosition: Coord = Coord(0, 0)
        private var paddlePosition: Coord = Coord(0, 0)
        fun runSimple() {
            while (input.size > 2) {
                val coord = Coord(input.poll().toInt(), input.poll().toInt())
                grid[coord] = input.poll().toInt()
            }
        }

        fun run() {
            while (input.size > 2) {
                val coord = Coord(input.poll().toInt(), input.poll().toInt())
                val block = input.poll().toInt()
                if (coord == Coord(-1, 0)) {
                    score = block
                } else {
                    grid[coord] = block
                    if (block == 3) paddlePosition = coord
                    if (block == 4) ballPosition = coord
                }
            }
            output.add((ballPosition.x - paddlePosition.x).let { if (it < 0) -1L else if (it > 0) 1L else 0L })
        }
    }

    override fun part1(): Int {
        val inp: Deque<Long> = LinkedList()
        val output: Deque<Long> = LinkedList()
        val intCode = IntCode("A", initialState, 2L, null, output)
        intCode.run()
        val grid = MutableGrid(100, 500) { 0 }
        val cabinet = Cabinet(grid, output, inp)
        cabinet.runSimple()
        return grid.count { it == 2 }
    }

    override fun part2(): Int {
        val inp: Deque<Long> = LinkedList()
        val output: Deque<Long> = LinkedList()
        val intCode = IntCode("B", listOf(2L) + initialState.drop(1), null, inp, output)
        val grid = MutableGrid(100, 500) { 0 }
        val cabinet = Cabinet(grid, output, inp)
        while (!intCode.isDone) {
            intCode.run()
            cabinet.run()
        }
        return cabinet.score
    }
}

fun main() = Day.runDay(Y19D13::class)

//    Class creation: 25ms
//    Part 1: 348 (30ms)
//    Part 2: 16999 (140ms)
//    Total time: 195ms
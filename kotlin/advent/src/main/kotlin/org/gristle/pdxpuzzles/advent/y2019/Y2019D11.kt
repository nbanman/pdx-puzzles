package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import org.gristle.pdxpuzzles.utilities.parsing.ocr
import java.util.*

class Y19D11(input: String) : Day {

    private val initialData = input.split(',').map { it.toLong() }

    enum class Paint { BLACK, WHITE }

    data class Panel(val paint: Paint = Paint.BLACK, val numPainted: Int = 0)

    data class Robot(
        val grid: MutableMap<Coord, Panel>,
        var coord: Coord,
        var direction: Nsew,
        val input: Deque<Long>,
        val output: Deque<Long>
    ) {
        fun sendInstruction() {
            val instruction = if (grid[coord]?.paint == Paint.WHITE) 1L else 0L
            output.add(instruction)
        }

        fun run() {
            if (input.size > 1) {
                val panel = grid[coord] ?: Panel()
                val newPaint = if (input.poll() == 0L) Paint.BLACK else Paint.WHITE
                grid[coord] = panel.copy(paint = newPaint, numPainted = panel.numPainted + 1)
                direction = if (input.poll() == 0L) direction.left() else direction.right()
                coord = direction.forward(coord)
                sendInstruction()
            }
        }
    }

    override fun part1(): Int {
        val grid = mutableMapOf<Coord, Panel>()
        val fromRobot = LinkedList<Long>()
        val fromComputer = LinkedList<Long>()
        val intCode = IntCode("A", initialData, null, fromRobot, fromComputer)
        val robot = Robot(grid, Coord(0, 0), Nsew.NORTH, fromComputer, fromRobot)
        robot.sendInstruction()
        while (!intCode.isDone) {
            intCode.run()
            robot.run()
        }
        return grid.values.count { it.numPainted >= 1 }
    }

    override fun part2(): String {
        val grid = mutableMapOf<Coord, Panel>()
        grid[Coord(0, 0)] = Panel(Paint.WHITE)
        val fromRobot = LinkedList<Long>()
        val fromComputer = LinkedList<Long>()
        val intCode = IntCode("B", initialData, null, fromRobot, fromComputer)
        val robot = Robot(grid, Coord(0, 0), Nsew.NORTH, fromComputer, fromRobot)
        robot.sendInstruction()
        while (!intCode.isDone) {
            intCode.run()
            robot.run()
        }
        val minX = grid.keys.minOf { it.x }
        val maxX = grid.keys.maxOf { it.x }
        val minY = grid.keys.minOf { it.y }
        val maxY = grid.keys.maxOf { it.y }

        val width = maxX - minX + 1
        val height = maxY - minY + 1

        val printGrid = List(width * height) { i ->
            if (grid[Coord(i % width + minX, i / width + minY)]?.paint == Paint.WHITE) '#' else '.'
        }.toGrid(width)
        return printGrid.ocr()
    }
}

fun main() = Day.runDay(Y19D11::class)
//    var time = System.nanoTime()
//    val c = Y19D11(readRawInput("y2019/d11"))
//    println("Class creation: ${elapsedTime(time)}ms")
//    time = System.nanoTime()
//    println("Part 1: ${c.part1()} (${elapsedTime(time)}ms)") // 2720
//    time = System.nanoTime()
//    println("Part 2: ${c.part2()} (${elapsedTime(time)}ms)") // JZPJRAGJ
//}
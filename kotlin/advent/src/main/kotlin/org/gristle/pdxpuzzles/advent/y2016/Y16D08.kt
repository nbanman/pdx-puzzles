package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.MutableGrid
import org.gristle.pdxpuzzles.utilities.objects.shift
import org.gristle.pdxpuzzles.utilities.parsing.getIntList
import org.gristle.pdxpuzzles.utilities.parsing.ocr

class Y16D08(input: String) : Day {

    private fun MutableGrid<Boolean>.executeInstruction(instruction: String) {
        val (n1: Int, n2: Int) = instruction.getIntList()
        when (instruction.replaceFirst("rotate ", "").takeWhile { it != ' ' }) {
            "rect" -> Coord
                .rectangleFrom(Coord.ORIGIN, Coord(n1 - 1, n2 - 1))
                .forEach { this[it] = true }

            "column" -> this
                .column(n1)
                .shift(-n2)
                .forEachIndexed { y, b -> this[n1, y] = b }

            "row" -> this
                .row(n1)
                .shift(-n2)
                .forEachIndexed { x, b -> this[x, n1] = b }
        }
    }

    private val screen: MutableGrid<Boolean> by lazy {
        MutableGrid(50, 6) { false }
            .apply { input.lineSequence().forEach { instruction -> executeInstruction(instruction) } }
    }

    override fun part1() = screen.count { it }

    override fun part2() = screen.ocr()
}

fun main() = Day.runDay(Y16D08::class)

//    Class creation: 24ms
//    Part 1: 123 (0ms)
//    Part 2: AFBUPZBJPS (5ms)
//    Total time: 30ms
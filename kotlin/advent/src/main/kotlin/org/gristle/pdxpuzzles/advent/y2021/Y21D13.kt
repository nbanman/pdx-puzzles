package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.MutableGrid
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import org.gristle.pdxpuzzles.utilities.parsing.groupValues
import org.gristle.pdxpuzzles.utilities.parsing.ocr

class Y21D13(input: String) : Day {

    data class FoldInstruction(val axis: Char, val amt: Int) {
        fun execute(paper: Grid<Boolean>): Grid<Boolean> {
            return when (axis) {
                'x' -> {
                    val left = paper.subGrid(Coord(0, 0), amt, paper.height)
                    val right = paper
                        .subGrid(Coord(amt + 1, 0), paper.width - 1 - amt, paper.height)
                        .flipY()
                    val (larger, smaller) = largerSmaller(left, right)
                    val offset = larger.width - smaller.width
                    performFold(Coord(offset, 0), larger, smaller)
                }

                'y' -> {
                    val up = paper.subGrid(Coord(0, 0), paper.width, amt)
                    val down = paper
                        .subGrid(Coord(0, amt + 1), paper.width, paper.height - 1 - amt)
                        .flipX()
                    val (larger, smaller) = largerSmaller(up, down)
                    val offset = smaller.height - larger.height
                    performFold(Coord(0, offset), larger, smaller)
                }

                else -> throw IllegalArgumentException("Regex returned illegal value")
            }
        }

        private fun largerSmaller(a: Grid<Boolean>, b: Grid<Boolean>) = if (a.size > b.size) a to b else b to a

        private fun performFold(
            adjustment: Coord,
            larger: Grid<Boolean>,
            smaller: Grid<Boolean>,
        ) = List(larger.size) { i ->
            val lCoord = larger.coordOf(i)
            val sCoord = lCoord + adjustment
            larger[lCoord] || if (smaller.validCoord(sCoord)) smaller[sCoord] else false
        }.toGrid(larger.width)
    }

    private val foldInstructions: List<FoldInstruction>

    private val dots: List<Coord>

    init {
        val (dotInput, foldInput) = input.blankSplit()

        dots = dotInput
            .getInts()
            .chunked(2) { (x, y) -> Coord(x, y) }
            .toList()

        foldInstructions = foldInput
            .groupValues("""fold along ([xy])=(\d+)""")
            .map { FoldInstruction(it[0][0], it[1].toInt()) }
    }

    private val paperWidth = dots.maxOf { it.x } + 1
    private val paperHeight = dots.maxOf { it.y } + 1

    private val paper = MutableGrid(paperWidth, paperHeight) { false }
        .apply {
            dots.forEach { dot -> this[dot] = true }
        } as Grid<Boolean>

    override fun part1() = foldInstructions.first().execute(paper).count { it }

    override fun part2(): String = foldInstructions
        .fold(paper) { acc, foldInstruction -> foldInstruction.execute(acc) }
        .ocr()
}

fun main() = Day.runDay(Y21D13::class)

//    Class creation: 40ms
//    Part 1: 735 (150ms)
//    Part 2: UFRZKAUZ (98ms)
//    Total time: 288ms
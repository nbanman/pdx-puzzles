package org.gristle.pdxpuzzles.advent.y2024


import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y24D04(input: String) : Day {
    private val jumble = input.toGrid()
    override fun part1(): Int {
        val starts = jumble.withIndex().filter { (_, c) -> c == 'X' }.map { (idx) -> jumble.coordOf(idx) }
        return starts
            .flatMap { start ->
                Coord.ALL_ADJACENT
                    .map { dir ->
                        (1..3)
                            .runningFold(start) { acc, _ -> acc + dir }
                            .mapNotNull { pos -> jumble.getOrNull(pos) }
                            .joinToString("")
                    }
            }.count { it == "XMAS" }
    }
    override fun part2(): Int {
        val starts = jumble.withIndex().filter { (_, c) -> c == 'A' }.map { (idx) -> jumble.coordOf(idx) }
        val lr = listOf(Coord(-1, -1), Coord(1, 1))
        val rl = listOf(Coord(1, -1), Coord(-1, 1))
        val ms = "MS".toSet()
        return starts.count { start ->
            val lMas = lr.mapNotNull { pos -> jumble.getOrNull(start + pos) }.toSet()
            val rMas = rl.mapNotNull { pos -> jumble.getOrNull(start + pos) }.toSet()
            lMas == ms && rMas == ms
        }
    }
}

fun main() = Day.runDay(Y24D04::class)

@Suppress("unused")
private val test = listOf(
    """MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX""",
)

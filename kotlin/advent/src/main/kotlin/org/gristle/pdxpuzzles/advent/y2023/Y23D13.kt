package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit

class Y23D13(input: String) : Day {
    data class Pattern(val rows: List<List<Boolean>>, val cols: List<List<Boolean>>) {
        fun seamSummary(smudged: Boolean): Int =
            findSeam(true, smudged)
                ?: findSeam(false, smudged)?.times(100)
                ?: throw IllegalStateException("No seam found!")

        private fun findSeam(isHz: Boolean, smudged: Boolean): Int? {
            val smudge = if (smudged) 1 else 0
            val lines = if (isHz) cols else rows

            for (i in 0 until lines.lastIndex) {
                var diff = 0
                for (j in 0..i) {
                    if (i + j + 1 == lines.size) break
                    diff += (lines[i - j] zip lines[i + j + 1]).count { (aa, bb) -> aa != bb }
                    if (diff > smudge) break
                }
                if (smudge == diff) return i + 1
            }
            return null
        }

        companion object {
            fun new(s: String): Pattern {
                val mirrors = s.toGrid { it == '#' }
                return Pattern(mirrors.rows(), mirrors.columns())
            }
        }
    }

    private val patterns: List<Pattern> = input.blankSplit().map(Pattern::new)

    override fun part1() = patterns.sumOf { it.seamSummary(false) }
    override fun part2() = patterns.sumOf { it.seamSummary(true) }
}

fun main() = Day.runDay(Y23D13::class)

//    Class creation: 8ms
//    Part 1: 27505 (4ms)
//    Part 2: 22906 (2ms)
//    Total time: 15ms

@Suppress("unused")
private val sampleInput = listOf(
    """#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
""",
)
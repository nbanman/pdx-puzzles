package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit

class Y23D13(input: String) : Day {

    // The value for part 1 is used in part 2, so make it a class value. We want representative times for part 1,
    // so make the value lazy. This parses by splitting on blank lines, turning each split into a Grid, then
    // running the findSeam function using Grid components. The Grid and the answer are returned in a Pair.
    private val patterns: List<Pair<Grid<Boolean>, Int>> by lazy {
        input.blankSplit()
            .map { s ->
                val pattern = s.toGrid { it == '#' }
                with(pattern) {
                    val value = findSeam(width, columns(), 1)
                        ?: findSeam(height, rows(), 100)
                        ?: throw IllegalStateException("No seam found in columns or rows.")
                    pattern to value
                }
            }
    } 
    
    // Finds the initial seam. It uses a stack that starts at the left/top and keeps adding lines until
    // a duplicate is found. That is a potential seam. It verifies the seam by popping lines off the stack and
    // comparing them to the next lines. If the stack value and the next line value don't add up, it's not a valid
    // seam, and the values are reset and the outer loop is resumed. If it reaches the end of the stack or the 
    // end of the lines, the seam is valid and that value is returned.  
    private fun findSeam(length: Int, lines: List<List<Boolean>>, factor: Int): Int? {
        val stack: MutableList<List<Boolean>> = ArrayList(length - 1)
        stack.add(lines.first())
        var potentialSeam = -1
        var location = 1
        loop@while (location in lines.indices) {
            val next = lines[location]
            if (next != stack.last()) {
                stack.add(next)
                location++
            } else { // potential seam found
                potentialSeam = location
                inner@while (true) {
                    stack.removeLast()
                    if (stack.isEmpty()) break@loop
                    val innerNext = lines.getOrNull(++location) ?: break@loop
                    if (stack.last() != innerNext) { // potential seam not real
                        location = potentialSeam - (location - potentialSeam)
                        while (location != potentialSeam + 1) {
                            stack.add(lines[location])
                            location++
                        }
                        potentialSeam = -1
                        break@inner
                    }
                }
                
            }
        }
        return if (potentialSeam == -1) null else potentialSeam * factor
    }

    // Finds the seam with a smudge. The logic is the same as findSeam, except for two things. First, it will
    // ignore the seam if the index is passed in by the 'ignore' parameter. This prevents finding the old seam.
    // Second, when making comparisons, it will allow a difference of 1, but only once for any potential seam.
    // So if 'smudge' is used to identify the potential seam, then the popping/comparison phase must have 
    // identical comparisons. If not, then the popping/comparison phase can have one comparison with a difference
    // of one. This is tracked with a 'smudged' Boolean variable.
    private fun findSmudgeSeam(length: Int, lines: List<List<Boolean>>, factor: Int, ignore: Int?): Int? {
        val stack: MutableList<List<Boolean>> = ArrayList(length - 1)
        stack.add(lines.first())
        var potentialSeam = -1
        var location = 1
        var smudged = false
        loop@while (location in lines.indices) {
            val next = lines[location]
            val difference = next.difference(stack.last())
            if (difference > 1 || (difference == 0 && ignore == location)) {
                stack.add(next)
                location++
            } else { // potential seam found
                if (difference == 1) smudged = true
                potentialSeam = location
                inner@while (true) {
                    stack.removeLast()
                    if (stack.isEmpty()) break@loop
                    val innerNext = lines.getOrNull(++location) ?: break@loop
                    val innerDifference = innerNext.difference(stack.last())
                    if (innerDifference > 1 || (innerDifference == 1 && smudged)) { // potential seam not real
                        smudged = false
                        location = potentialSeam - (location - potentialSeam)
                        while (location != potentialSeam + 1) {
                            stack.add(lines[location])
                            location++
                        }
                        potentialSeam = -1
                        break@inner
                    }
                    if (innerDifference == 1) smudged = true
                }

            }
        }
        return if (potentialSeam == -1) null else potentialSeam * factor
    }
    
    // Utility function to return the number of different elements between two lists.
    private fun List<Boolean>.difference(other: List<Boolean>): Int {
        return (this zip other).count { (a, b) -> a != b }
    }

    override fun part1() = patterns.sumOf { (_, value) -> value }

    override fun part2() = patterns
        .sumOf { (pattern, value) ->
            // The indices of previous seams to ignore. 
            val ignoreHorizontal = if (value < 100) value else null
            val ignoreVertical = if (value >= 100) value / 100 else null
            
            with (pattern) {
                val answer = findSmudgeSeam(width, columns(), 1, ignoreHorizontal)
                    ?: findSmudgeSeam(height, rows(), 100, ignoreVertical)
                    ?: throw IllegalStateException("No seam found in columns or rows.")
                answer
            }
        }
}

fun main() = Day.runDay(Y23D13::class)

//    Class creation: 2ms
//    Part 1: 27505 (17ms)
//    Part 2: 22906 (10ms)
//    Total time: 30ms

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
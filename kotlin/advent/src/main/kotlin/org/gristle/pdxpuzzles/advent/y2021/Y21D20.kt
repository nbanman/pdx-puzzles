package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.isEven
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.toGrid

/*
This one broke my brain, needed help. You will notice a resemblance to /u/jkpr's solution. 
https://github.com/jkpr/advent-of-code-2021-kotlin/blob/master/src/day20/Day20.kt
Mine is essentially the same as his but uses my Grid/Coord objects instead of List<List<Boolean>> 
and nested for loops.
*/

class Y21D20(val data: String) : Day {
    private val inputSplit = data.split("\n\n")
    private val algorithm = inputSplit[0].map { it == '#' }
    private val image = inputSplit[1].toGrid { it == '#' }

    // Pairs up the two IntRanges for a quick-and-dirty 3x3 matrix.
    private operator fun IntRange.times(other: IntRange) =
        other.flatMap { y -> map { x -> x to y } }

    private fun Coord.nine() = ((-1..1) * (-1..1)).map { (x, y) -> Coord(x, y) + this }

    private fun enhance(steps: Int): Int {
        val result = (1..steps).fold(image) { acc, step ->
            // flashes the lights outside the finite grid on and off depending on the turn
            val outside = step.isEven()

            // Each step, the finite grid becomes one larger on every side. The offset is used
            // to refer to the equivalent point on the previous, smaller grid.
            val offset = Coord(1, 1)
            Grid(acc.width + 2, acc.height + 2) { i ->
                // Finds the index to look up the algorithm string.
                val algorithmIndex = Coord.fromIndex(i, acc.width + 2) // start with the coordinates
                    .nine() // include the coordinates of the surrounding spots
                    .map { // transform to Boolean indicating whether that spot is lit
                        if (it.x in 1..acc.width && it.y in 1..acc.height) { // if part of finite grid... 
                            acc[it - offset]  // record what was in previous grid
                        } else { // else part of infinite gird
                            outside // record whether flashing or not
                        }
                    }.reversed() // reversed().withIndex().sumOf() uses the index as an exponent to concatenate the
                    .withIndex() // boolean values in the way specified by the algorithm
                    .sumOf { (index, bit) -> if (bit) 1 shl index else 0 }

                algorithm[algorithmIndex] // Populates the new grid
            }
        }
        return result.count { it }
    }

    override fun part1() = enhance(2)

    override fun part2() = enhance(50)
}

fun main() = Day.runDay(Y21D20::class)

//    Class creation: 24ms
//    Part 1: 5786 (79ms)
//    Part 2: 16757 (924ms)
//    Total time: 1027ms
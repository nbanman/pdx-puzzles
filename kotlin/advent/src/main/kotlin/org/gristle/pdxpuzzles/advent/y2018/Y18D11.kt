package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.toMutableGrid

class Y18D11(input: String) : Day {

    // used to calculate power level
    private val serialNumber = input.toInt()

    // size of each side of grid
    private val length = 300

    // initial power level calculations for the grid
    private val cells: Grid<Int> = Grid(length, length) { index ->
        val x = (index % length) + 1
        val y = (index / length) + 1
        val rackId = x + 10
        ((((rackId * y + serialNumber) * rackId) % 1000) / 100) - 5
    }

    // Used to store relevant information for answer and keeping track of max power 
    data class Square(val x: Int = 0, val y: Int = 0, val size: Int = 0, val power: Int = 0)

    // smallest and largest set the minimum and maximum size for the square. So 3,3 for pt1 and 1,300 for pt2
    private fun solve(smallest: Int, largest: Int): Square {
        // create a "working" grid that starts as a copy of the "cells" grid. But after each pass of a row, each 
        // cell in that row is updated to include cells from lower rows in accordance with the size of the squares
        // being evaluated. This way we can avoid re-summing when evaluating larger squares.
        val grid = cells.toMutableGrid()

        // Track the square that has the largest power. Default is zero.
        var max = Square()

        // Outer loop gradually increases the size of the square to be evaluated up to "largest." Starts at 1
        // rather than "smallest" because even if sums are not calculated, the grid needs to be updated for later 
        // passes.
        for (size in 1..largest) {

            // First nested loop runs through all the rows to be evaluated. The grid update process grabs cell 
            // values from lower rows so this maxes out at the length minus the size.
            for (y in 0..length - size) {

                // The Grid class stores data in a 1-D array so this grabs all the values in that row for easy access 
                val row = grid.row(y)

                // Only do summing activity if the square size is at least as large as "smallest."
                if (size >= smallest) {

                    // start with the left-most cell. Because of grid updating grabbing values from lower rows, 
                    // this is always the sum of all cells below it that are part of the square
                    var power = row.take(size).sum()

                    // move right with a "windowed" movement, adding the next to the right and subtracting the last
                    // from the left
                    for (x in size until row.size) {

                        // update max if needed 
                        if (power > max.power) max = Square(x - size + 1, y + 1, size, power)

                        // calculate power for next square in row
                        power += row[x] - row[x - size]
                    }
                }

                // update grid by adding the row with row + size
                if (y < length - size) {
                    for (x in row.indices) {
                        grid[x, y] += cells[x, y + size]
                    }
                }
            }
        }
        return max
    }

    override fun part1() = solve(3, 3).let { (x, y) -> "$x,$y" }

    override fun part2() = solve(1, 300).let { (x, y, size) -> "$x,$y,$size" }
}

fun main() = Day.runDay(Y18D11::class)

//    Class creation: 11ms
//    Part 1: 235,48 (40ms)
//    Part 2: 285,113,11 (316ms)
//    Total time: 368ms
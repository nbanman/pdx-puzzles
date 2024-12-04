package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import kotlin.math.abs

class Y23D11(private val image: String) : Day {
    private val width = image.indexOf('\n') + 1
    private val height = (image.length + 1) / width
    
    // for each axis, get indexed iterables of pairs. The first being the index where galaxies reside, the 
    // second being the number of galaxies. We separate the axes to avoid repeat calculation.
    private val xGalaxies: Iterable<IndexedValue<Pair<Int, Int>>> = (0 until width - 1)
        .map { x -> x to (0 until height).count { y -> image[y * width + x] == '#' } }
        .filter { (_, count) -> count > 0 }
        .withIndex()
    private val yGalaxies: Iterable<IndexedValue<Pair<Int, Int>>> = (0 until height)
        .map { y -> y to (0 until width - 1).count { x -> image[y * width + x] == '#' } }
        .filter { (_, count) -> count > 0 }
        .withIndex()

    // for each axis, track the indices representing expansion fields
    private val xExpansions: List<Int> = (0 until width - 1)
        .filter { x -> (0 until height).none { y -> image[y * width + x] == '#' } }
    private val yExpansions: List<Int> = (0 until height)
        .filter { y -> (0 until width - 1).none { x -> image[y * width + x] == '#' } }

    // run the distance function twice (once for each axis), return the sum 
    private fun solve(expansionFactor: Int): Long = distance(expansionFactor, xGalaxies, xExpansions) +
            distance(expansionFactor, yGalaxies, yExpansions)

    // Get the distance between two indices where galaxies reside.
    // This involves calculating the unexpanded difference multiplied by the number of expansions passed times
    // the expansion factor
    // Lastly, the distance is multiplied by #galaxies in the first index multiplied by #galaxies in the second index.
    private fun distance(
        expansionFactor: Int,
        galaxies: Iterable<IndexedValue<Pair<Int, Int>>>,
        expansions: List<Int>
    ): Long {
        return galaxies.sumOf { (i, a) ->
            val (aPosition, aCount) = a

            // calculate which expansions are to the left of the source galaxies
            // this returns a negative number due to how binarySearch returns values but this will be rectified
            // later.
            val alreadyPassed = expansions.binarySearch(aPosition)
            galaxies.drop(i + 1).sumOf { (_, b) ->
                val (bPosition, bCount) = b

                // calculates which expansions are to the left of the destination galaxies, and subtracts the
                // ones to the left of the source galaxies. The abs function handles the negative value.
                val expansionsPassed = abs(expansions.binarySearch(bPosition) - alreadyPassed)
                ((bPosition - aPosition) + expansionsPassed.toLong() * (expansionFactor - 1)) * aCount * bCount
            }
        }
    }

    override fun part1() = solve(2)

    override fun part2() = solve(1_000_000)
}

fun main() = Day.runDay(Y23D11::class)

//    Class creation: 7ms
//    Part 1: 9545480 (10ms)
//    Part 2: 406725732046 (6ms)
//    Total time: 24ms

@Suppress("unused")
private val sampleInput = listOf(
    """...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
""",
)
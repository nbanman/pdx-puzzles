package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y23D03(private val schematic: String) : Day {
    // we use the input schematic directly, using indexes. To move up and down the y axis, we need the width.
    private val width = schematic.indexOf('\n') + 1

    // intermediate step used for both parts. Given a predicate to know which symbols to look for, looks at every
    // symbol and for each returns a set of IntRanges representing numbers that are adjacent to it. Intranges are
    // used rather than the underlying Ints because IntRanges are unique and Ints are not.
    private inline fun numbersAdjacentToSymbol(isValidSymbol: (Char) -> Boolean): List<Set<IntRange>> = schematic
        .withIndex()
        .filter { (_, c) -> isValidSymbol(c) } // tests whether a 'symbol' in pt1, and a 'gear' in pt2
        .map { (index, _) -> // turn each valid symbol into a list of the adjacent IntRanges
            buildSet {
                for (y in -1..1) {
                    for (x in -1..1) {
                        getNumberOrNull(index + y * width + x)?.let { add(it) }
                    }
                }
            }
        }

    // for a given index in the schematic, if there is a digit, expand left and right until the digit ends. Returns
    // null if the index is out of bounds or does not contain a digit.
    private fun getNumberOrNull(index: Int): IntRange? {
        // If the index is out of bounds or does not contain a digit, return null.
        if (index !in schematic.indices || !schematic[index].isDigit()) return null

        // keep subtracting from leftIndex while there are digits to the left. The getOrNull version of get ensures
        // that the leftIndex does not go below 0.
        var leftIndex = index
        while (schematic.getOrNull(leftIndex - 1)?.isDigit() == true) leftIndex--

        // keep adding to rightIndex while there are digits to the right. 
        var rightIndex = index
        while (schematic.getOrNull(rightIndex + 1)?.isDigit() == true) rightIndex++

        // return as range
        return leftIndex..rightIndex
    }

    // sums the numbers provided by validNumbers, using a "catchall" predicate that says a symbol is valid if it is 
    // not a digit, '.', or line break.
    override fun part1() = numbersAdjacentToSymbol { it !in ".\n" && !it.isDigit() }
        .flatten() // internal set of numbers not needed for p1 so flatten it away 
        .distinct() // removes theoretical double-count of an IntRange adjacent to two symbols
        .sumOf { schematic.substring(it).toInt() } // map IntRange to number in schematic and sum them

    // 'gears' are '*' symbols that have exactly two numbers adjacent. So we use filter to remove symbols with fewer
    // or greater numbers. We sum the product of the two numbers.
    override fun part2() = numbersAdjacentToSymbol { it == '*' }
        .filter { it.size == 2 } // definition of gear requires exactly two adjacent numbers
        .map { it.map { intRange -> schematic.substring(intRange).toInt() } } // convert IntRanges to numbers
        .sumOf { it.reduce(Int::times) } // multiply gear ratios and sum them
}

fun main() = Day.runDay(Y23D03::class)

//    Class creation: 3ms
//    Part 1: 525911 (21ms)
//    Part 2: 75805607 (7ms)
//    Total time: 32ms
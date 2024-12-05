package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y20D05(input: String) : Day {

    // The seat id is Row * 8 + Column. Since there are 8 columns this means that the seatIDs are sequential from
    // 0 to 1016, and the FBLR code is just a binary number with 'F' and 'L' meaning '0' and 'B' and 'R' meaning 1.
    private val seatIds: List<Int> by lazy {
        input
            .lines()
            .map { it.foldIndexed(0) { index, acc, c -> acc + if (c in "BR") 1.shl(9 - index) else 0 } }
            .sorted() // Pt1 needs the largest, and Pt2 needs a full sort, so just go ahead and sort the seatIds
    }

    override fun part1() = seatIds.last() // Since sorted, this is the highest seat ID on a boarding pass.

    // The seatIds should all be contiguous. Yours is missing, so look for the first non-contiguous seatId in the
    // sorted list of seatIds. Yours would be the seatId immediately below that.
    override fun part2() = seatIds
        .zipWithNext() // pair up previous seatId and the next seatId
        .first { (prev, next) -> prev + 1 != next } // find the first instance where the next seatId is not contiguous
        .let { (prev, _) -> prev + 1 } // return that, plus 1 (since your ticket is the missing seatId)
}

fun main() = Day.runDay(Y20D05::class)

//    Class creation: 14ms
//    Part 1: 922 (11ms)
//    Part 2: 747 (0ms)
//    Total time: 25ms
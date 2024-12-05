package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.mapToGridIndexed
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y19D24(input: String) : Day {

    private val erisOriginal = input.toGrid { it == '#' }

    private fun Int.bitOn(pos: Int) = shr(pos).and(1)

    private val up = (0..4).sumOf { 1.shl(it) }
    private val down = (20..24).sumOf { 1.shl(it) }
    private val left = (0..4).sumOf { 1.shl(it * 5) }
    private val right = (0..4).sumOf { 1.shl(it * 5 + 4) }

    private fun Int.bugsOnSide(side: Int) = Integer.bitCount(and(side))

    private fun Int.bugInMiddle(side: Int) = shr(side).and(1)

    @Suppress("unused")
    private fun Int.print() {
        buildString {
            for (i in 0..24) {
                if (i == 12) {
                    append('?')
                    continue
                }
                append(if (bitOn(i) == 1) '#' else '.')
                if (i % 5 == 4) append('\n')
            }
        }.let { println(it) }
    }

    private fun Grid<Boolean>.toInt(): Int {
        return foldIndexed(0) { index, acc, b ->
            if (b) {
                acc + 1.shl(index)
            } else {
                acc
            }
        }
    }

    private fun Grid<Boolean>.advance() = mapToGridIndexed { index, b ->
        val adjacentBugs = getNeighbors(index).count { it }
        adjacentBugs == 1 || (!b && adjacentBugs == 2)
    }

    override fun part1(): Int {
        val ratings = mutableSetOf<Int>() // tracks ratings so that sequence can stop when rating appears twice 
        return generateSequence(erisOriginal) { it.advance() } // base GoL sequence 
            .map { it.toInt() } // converted to biodiversity rating
            .first { !ratings.add(it) } // add rating to ratings and return the first rating that is a repeat
    }

    override fun part2(): Int {
        val iterations = 200
        var eris = List(301) { i -> if (i == 150) erisOriginal.toInt() else 0 }
        var lowerBound = 149
        var upperBound = 151
        for (i in 1..iterations) {
            eris = List(eris.size) { d ->
                if (d !in lowerBound..upperBound) {
                    0
                } else {
                    var newDimension = 0
                    for (bit in 0..24) {
                        if (bit == 12) continue
                        val isBug = eris[d].bitOn(bit) == 1
                        var bugs = 0
                        // Look up
                        bugs += when {
                            bit == 17 -> {
                                eris[d - 1].bugsOnSide(down)
                            }
                            bit < 5 -> {
                                eris[d + 1].bugInMiddle(7)
                            }
                            else -> {
                                eris[d].bitOn(bit - 5)
                            }
                        }
                        // Look down
                        bugs += when {
                            bit == 7 -> {
                                eris[d - 1].bugsOnSide(up)
                            }
                            bit >= 20 -> {
                                eris[d + 1].bugInMiddle(17)
                            }
                            else -> {
                                eris[d].bitOn(bit + 5)
                            }
                        }
                        // Look left
                        bugs += when {
                            bit == 13 -> {
                                eris[d - 1].bugsOnSide(right)
                            }
                            bit % 5 == 0 -> {
                                eris[d + 1].bugInMiddle(11)
                            }
                            else -> {
                                eris[d].bitOn(bit - 1)
                            }
                        }
                        // Look right
                        bugs += when {
                            bit == 11 -> {
                                eris[d - 1].bugsOnSide(left)
                            }
                            bit % 5 == 4 -> {
                                eris[d + 1].bugInMiddle(13)
                            }
                            else -> {
                                eris[d].bitOn(bit + 1)
                            }
                        }
                        val newBug = bugs == 1 || (!isBug && bugs == 2)
                        newDimension += if (newBug) 1.shl(bit) else 0
                    }
                    newDimension
                }
            }
            if (141440.and(eris[lowerBound]) != 0) {
                lowerBound--
            }
            if (33080895.and(eris[upperBound]) != 0) {
                upperBound++
            }
        }
        return eris.sumOf { Integer.bitCount(it) }
    }
}

fun main() = Day.runDay(Y19D24::class)

//    Class creation: 20ms
//    Part 1: 18852849 (8ms)
//    Part 2: 1948 (58ms)
//    Total time: 87ms
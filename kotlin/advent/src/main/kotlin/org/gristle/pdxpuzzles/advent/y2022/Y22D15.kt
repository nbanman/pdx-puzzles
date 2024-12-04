package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.iteration.minMaxBy
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import kotlin.math.abs

class Y22D15(input: String) : Day {

    class Sensor(val pos: Coord, private val beaconPos: Coord) {
        fun toRangeOrNull(y: Int): IntRange? {
            val xDistance = pos.manhattanDistance(beaconPos) - abs(pos.y - y)
            return if (xDistance >= 0) {
                pos.x - xDistance..pos.x + xDistance
            } else {
                null
            }
        }
    }

    private fun IntRange.contiguousOrNull(other: IntRange): IntRange? {
        val (lesser, greater) = minMaxBy(this, other) { it.first }
        return if (lesser.last >= greater.first) {
            lesser.first..kotlin.math.max(lesser.last, greater.last)
        } else {
            null
        }
    }

    private val sensors = input
        .getInts()
        .chunked(4) { (x1, y1, x2, y2) -> Sensor(Coord(x1, y1), Coord(x2, y2)) }
        .toList()

    private fun List<IntRange>.concatenate(): List<IntRange> {
        val mutableRanges = this.toMutableList()
        var i: Int
        var size = 0
        while (size != mutableRanges.size) {
            size = mutableRanges.size
            i = 0
            while (i < mutableRanges.lastIndex) {
                var j = i + 1
                while (j < mutableRanges.size) {
                    val union = mutableRanges[i].contiguousOrNull(mutableRanges[j])
                    if (union == null) j++ else {
                        mutableRanges[i] = union
                        mutableRanges.removeAt(j)
                    }
                }
                i++
            }
        }
        return mutableRanges
    }

    private fun rowRanges(y: Int): List<IntRange> = sensors
        .mapNotNull { it.toRangeOrNull(y) }
        .concatenate()

    override fun part1() = rowRanges(2000000)
        .sumOf { it.last - it.first }

    override fun part2() = generateSequence(0) { it + 1 }
        .map { y -> y to rowRanges(y) }
        .first { (_, ranges) -> ranges.size > 1 }
        .let { (y, ranges) ->
            val x = ranges.minBy(IntRange::first).last + 1
            4_000_000L * x + y
        }
}

fun main() = Day.runDay(Y22D15::class)

//    Class creation: 7ms
//    Part 1: 5073496 (0ms)
//    Part 2: 13081194638237 (1893ms)
//    Total time: 1901ms
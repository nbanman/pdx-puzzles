package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.iteration.takeUntil
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.parsing.getIntList

class Y18D17(input: String) : Day {
    enum class Ground { CLAY, STILL_WATER, MOVING_WATER }

    private val cavern: MutableMap<Coord, Ground> = input
        .lines()
        .flatMap { line ->
            val (fixed, low, high) = line.getIntList()
            if (line[0] == 'x') {
                (low..high).map { Coord(fixed, it) }
            } else {
                (low..high).map { Coord(it, fixed) }
            }
        }.associateWith { Ground.CLAY }
        .toMutableMap()

    // The deepest part of the cavern recorded.
    private val firstClayDepth = cavern.keys.minOf(Coord::y)
    private val depth = cavern.keys.maxOf(Coord::y)

    private val start = Coord(500, 1)

    private fun Coord.seep(): List<Coord> {

        val below = south()

        return when (cavern[below]) {
            null, Ground.MOVING_WATER -> {
                cavern[this] = Ground.MOVING_WATER
                if (below.y > depth) emptyList() else listOf(below)
            }

            else -> {
                val left = generateSequence(this.west()) { it.west() }
                    .takeUntil {
                        cavern[it] == Ground.CLAY
                                || cavern[it.south()] == null
                                || cavern[it.south()] == Ground.MOVING_WATER
                    }.toMutableList()
                val leftWall = cavern[left.last()] == Ground.CLAY
                if (leftWall) {
                    left.removeLast()
                } else {
                    if (cavern[left.last().south()] == Ground.MOVING_WATER) {
                        left.clear()
                    }
                }

                val right = generateSequence(this) { it.east() }
                    .takeUntil {
                        cavern[it] == Ground.CLAY
                                || cavern[it.south()] == null
                                || cavern[it.south()] == Ground.MOVING_WATER
                    }.toMutableList()
                val rightWall = cavern[right.last()] == Ground.CLAY
                if (rightWall) {
                    right.removeLast()
                } else {
                    if (cavern[right.last().south()] == Ground.MOVING_WATER) {
                        right.clear()
                    }
                }

                val waterline = left + right

                if (leftWall && rightWall) {
                    waterline.forEach { cavern[it] = Ground.STILL_WATER }
                    listOf(this.north())
                } else {
                    waterline.forEach { cavern[it] = Ground.MOVING_WATER }
                    val stillRunning = mutableListOf<Coord>()
                    if (!leftWall && left.isNotEmpty()) stillRunning.add(left.last().south())
                    if (!rightWall && right.isNotEmpty()) stillRunning.add(right.last().south())
                    stillRunning
                }
            }
        }
    }

    init {
        generateSequence(listOf(start)) { stillRunning -> stillRunning.flatMap { it.seep() }.distinct() }
            .first { stillRunning -> stillRunning.isEmpty() }
    }

    override fun part1() = cavern.count { (pos, ground) ->
        pos.y >= firstClayDepth && (ground == Ground.MOVING_WATER || ground == Ground.STILL_WATER)
    }

    override fun part2() = cavern.values.count { it == Ground.STILL_WATER }
}

fun main() = Day.runDay(Y18D17::class)

//    Class creation: 136ms
//    Part 1: 40879 (14ms)
//    Part 2: 34693 (7ms)
//    Total time: 158ms
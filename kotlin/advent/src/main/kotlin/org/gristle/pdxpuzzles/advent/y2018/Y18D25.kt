package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.MCoord
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y18D25(private val input: String) : Day {
    private fun MCoord.inRange(other: MCoord) = manhattanDistance(other) <= 3

    override fun part1(): Int {
        val points = input
            .getInts()
            .chunked(4, ::MCoord)

        val constellations = mutableSetOf<List<MCoord>>()

        for (point in points) {
            val inRange = constellations
                .filter { constellation -> constellation.any { it.inRange(point) } }
            if (inRange.isEmpty()) {
                constellations.add(listOf(point))
            } else {
                val stars = inRange.sumOf { it.size } + 1
                val newConstellation = ArrayList<MCoord>(stars).apply {
                    add(point)
                    inRange.forEach { addAll(it) }
                }
                constellations.removeAll(inRange.toSet())
                constellations.add(newConstellation)
            }
        }
        return constellations.size
    }

    override fun part2() = "Merry XMas!!!"
}

fun main() = Day.runDay(Y18D25::class)

//    Class creation: 1ms
//    Part 1: 394 (87ms)
//    Total time: 89ms
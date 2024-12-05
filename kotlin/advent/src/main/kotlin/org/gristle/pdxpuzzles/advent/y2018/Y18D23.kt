package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.iteration.pollUntil
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import org.gristle.pdxpuzzles.utilities.objects.Xyz
import java.util.*
import kotlin.math.ceil
import kotlin.math.ln
import kotlin.math.pow

class Y18D23(input: String) : Day {

    data class Nanobot(val pos: Xyz, val radius: Int) {
        fun inRangeOf(other: Nanobot) = radius >= pos.manhattanDistance(other.pos)
        private fun inRangeOf(other: Xyz) = radius >= pos.manhattanDistance(other)
        fun inRangeOf(cube: Cube): Boolean {
            return Xyz(
                when {
                    pos.x in cube.pos.x until cube.pos.x + cube.length -> {
                        pos.x
                    }
                    pos.x < cube.pos.x -> {
                        cube.pos.x
                    }
                    else -> {
                        cube.pos.x + cube.length - 1
                    }
                },
                when {
                    pos.y in cube.pos.y until cube.pos.y + cube.length -> {
                        pos.y
                    }
                    pos.y < cube.pos.y -> {
                        cube.pos.y
                    }
                    else -> {
                        cube.pos.y + cube.length - 1
                    }
                },
                when {
                    pos.z in cube.pos.z until cube.pos.z + cube.length -> {
                        pos.z
                    }
                    pos.z < cube.pos.z -> {
                        cube.pos.z
                    }
                    else -> {
                        cube.pos.z + cube.length - 1
                    }
                }
            ).let { inRangeOf(it) }
        }
    }

    class Cube(val pos: Xyz, val length: Int, parentBots: List<Nanobot>) {
        val nanobots = parentBots.filter { it.inRangeOf(this) }

        fun cubify(): List<Cube> {
            val newLength = length / 2
            return listOf(
                Cube(Xyz(pos.x, pos.y, pos.z), newLength, nanobots),
                Cube(Xyz(pos.x + newLength, pos.y, pos.z), newLength, nanobots),
                Cube(Xyz(pos.x, pos.y + newLength, pos.z), newLength, nanobots),
                Cube(Xyz(pos.x + newLength, pos.y + newLength, pos.z), newLength, nanobots),
                Cube(Xyz(pos.x, pos.y, pos.z + newLength), newLength, nanobots),
                Cube(Xyz(pos.x + newLength, pos.y, pos.z + newLength), newLength, nanobots),
                Cube(Xyz(pos.x, pos.y + newLength, pos.z + newLength), newLength, nanobots),
                Cube(Xyz(pos.x + newLength, pos.y + newLength, pos.z + newLength), newLength, nanobots)
            )
        }

        override fun toString(): String {
            return "Cube: $pos, Length: $length, Bots: ${nanobots.size}"
        }
    }

    private val nanobots = input
        .getInts()
        .chunked(4) { (x, y, z, radius) -> Nanobot(Xyz(x, y, z), radius) }
        .toList()

    override fun part1(): Int {
        val strongest = nanobots.maxByOrNull(Nanobot::radius) ?: return -1
        return nanobots.count { strongest.inRangeOf(it) }
    }

    override fun part2(): Int {
        val xMin = nanobots.minOf { it.pos.x }
        val xMax = nanobots.maxOf { it.pos.x }
        val yMin = nanobots.minOf { it.pos.y }
        val yMax = nanobots.maxOf { it.pos.y }
        val zMin = nanobots.minOf { it.pos.z }
        val zMax = nanobots.maxOf { it.pos.z }

        val (min, max) = listOf(xMin to xMax, yMin to yMax, zMin to zMax)
            .maxByOrNull { (min, max) -> max - min } ?: throw Exception("list of minmax ranges empty")

        val length = 2.0f.pow(ceil(ln((max - min).toFloat()) / ln(2.0f))).toInt()
        val initialCube = Cube(Xyz(min, min, min), length, nanobots)
        val cubes = PriorityQueue(compareBy<Cube> { it.length }.thenByDescending { it.nanobots.size })
        cubes.add(initialCube)
        var current = Cube(Xyz(0, 0, 0), 1, nanobots)
        while (true) {
            val next = cubes
                .pollUntil { it.nanobots.size > current.nanobots.size }
                ?.cubify() ?: break
            for (c in next) {
                current = if (c.length == 1 && c.nanobots.size > current.nanobots.size) c else current
                cubes.add(c)
            }
        }
        return current.pos.manhattanDistance(Xyz(0, 0, 0))
    }
}

fun main() = Day.runDay(Y18D23::class)

//    Class creation: 37ms
//    Part 1: 481 (2ms)
//    Part 2: 47141479 (874ms)
//    Total time: 914ms
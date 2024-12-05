package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.iteration.stabilized
import org.gristle.pdxpuzzles.utilities.objects.Coord

class Y21D25(input: String) : Day {
    private val lines = input.lines()

    data class Cucumbers(val east: Set<Coord>, val south: Set<Coord>, val size: Coord) {
        fun step(): Cucumbers {
            val newEast = east.map { p ->
                p.east(1, size, true).let { n -> if (n in east || n in south) p else n }
            }.toSet()
            val newSouth = south.map { p ->
                p.south(1, size, true).let { n -> if (n in newEast || n in south) p else n }
            }.toSet()
            return Cucumbers(newEast, newSouth, size)
        }
    }

    private fun makeCucumbers(): Cucumbers {
        val seaFloor = lines.flatMapIndexed { y, line -> line.mapIndexed{ x, ch -> Coord(x, y) to ch }}
        val east = seaFloor.mapNotNull { (coord, char) -> if (char == '>') coord else null }.toSet()
        val south = seaFloor.mapNotNull { (coord, char) -> if (char == 'v') coord else null }.toSet()
        val size = Coord(lines[0].length, lines.size)

        return Cucumbers(east, south, size)
    }

    tailrec fun solve(steps: Int = 1, prev: Cucumbers = makeCucumbers()): Int {
        val next = prev.step()
        return if (prev == next) steps else solve(steps + 1, next)
    }

    override fun part1() = solve()

    override fun part2() = "Merry Xmas, you filthy animal!"

    fun part1b() = generateSequence(makeCucumbers(), Cucumbers::step).withIndex().stabilized().index + 1
}


fun main() = Day.runDay(Y21D25::class)

//    Class creation: 30ms
//    Part 1: 528 (563ms)
//    Total time: 594ms
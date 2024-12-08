package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.combinatorics.getPairs
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y24D08(val input: String) : Day {
    private val cityLimits = input.toGrid()

    private val antennae: Map<Char, List<Coord>> = buildMap<Char, MutableList<Coord>> {
        cityLimits.withIndex()
            .filter { (_, roof) -> roof != '.' }
            .forEach { (index, antenna) ->
                val pos = Coord.fromIndex(index, cityLimits.width)
                getOrPut(antenna) { mutableListOf() }.add(pos)
            }
    }

    private inline fun solve(getAntinodes: (Pair<Coord, Coord>) -> List<Coord>) = antennae.values
        .flatMap { positions -> positions.getPairs().flatMap(getAntinodes) }
        .distinct()
        .size

    override fun part1() = solve { (a, b) ->
        val diff = a - b
        listOf(a + diff, b - diff).filter { antinode -> antinode isWithin cityLimits }
    }

    override fun part2() = solve { (a, b) ->
        val diff = a - b
        ray(a, diff) + ray(b, -diff)
    }

    private fun ray(start: Coord, diff: Coord) = generateSequence(start) { it + diff }
        .takeWhile { antinode -> antinode isWithin cityLimits }
        .toList()
}

fun main() = Day.runDay(Y24D08::class)

@Suppress("unused")
private val test = listOf(
    """............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
""",
)

//    Class creation: 7ms
//    Part 1: 228 (4ms)
//    Part 2: 766 (8ms)
//    Total time: 20ms
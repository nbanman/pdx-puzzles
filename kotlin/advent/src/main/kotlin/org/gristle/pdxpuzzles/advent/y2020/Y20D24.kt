package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Hexagon

class Y20D24(input: String) : Day {

    val flippedTiles: Set<Hexagon> = buildSet {
        val pattern = "(nw|ne|sw|se|w|e)".toRegex()
        val rules = input
            .lines()
            .map { line ->
                pattern.findAll(line)
                    .toList()
                    .map {
                        when (it.value) {
                            "w" -> "n"
                            "nw" -> "ne"
                            "ne" -> "se"
                            "e" -> "s"
                            "se" -> "sw"
                            "sw" -> "nw"
                            else -> throw IllegalStateException()
                        }
                    }
            }

        for (rule in rules) {
            val tile = rule.fold(Hexagon.ORIGIN, Hexagon::hexAt)

            if (contains(tile)) {
                remove(tile)
            } else {
                add(tile)
            }
        }
    }



    override fun part1() = flippedTiles.size

    override fun part2(): Int {
        var flippedTiles = flippedTiles

        val candidates = mutableMapOf<Hexagon, Int>()

        repeat(100) {
            for (tile in flippedTiles) {
                for (adj in tile.neighbors()) {
                    candidates[adj] = candidates.getOrDefault(adj, 0) + 1
                }
            }
            flippedTiles = candidates.asSequence()
                .filter { (candidate, occurrences) ->
                    occurrences == 2 || (occurrences == 1 && flippedTiles.contains(candidate))
                }.map { (candidate, _) -> candidate }
                .toSet()
            candidates.clear()
        }

        return flippedTiles.size
    }
}

fun main() = Day.runDay(Y20D24::class)

//    Class creation: 11ms
//    Part 1: 244 (1ms)
//    Part 2: 3665 (80ms)
//    Total time: 93ms
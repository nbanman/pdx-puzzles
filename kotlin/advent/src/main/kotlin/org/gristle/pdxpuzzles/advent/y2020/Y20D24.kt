package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Hexagon

class Y20D24(input: String) : Day {

    val pattern = "(nw|ne|sw|se|w|e)".toRegex()
    private val rules = input
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
                        else -> it.value
                    }
                }
        }
    private val home = Hexagon.ORIGIN
    private val flipped: Map<Hexagon, Boolean> = buildMap {
        rules.forEach { rule ->
            val tile = rule.fold(home, Hexagon::hexAt)
            this[tile] = !getOrPut(tile) { false }
        }
    }

    override fun part1() = flipped.count { it.value }

    override fun part2(): Int {
        fun hexRing(n: Int): List<Hexagon> = buildList {
            for (r in 0..n) add(Hexagon(-n, r))
            for (q in -(n - 1)..0) {
                add(Hexagon(q, n))
                add(Hexagon(q, -(n + q)))
            }
            addAll(dropLast(2).map { hex -> Hexagon(-hex.q, -hex.r) })
        }

        var flipMap = flipped
        var radius = flipMap.filter { it.value }.maxOf { it.key.distance(home) }
        for (day in 1..100) {
            val newMap = flipMap.toMutableMap()
            radius++
            val hexen: List<Hexagon> = buildList {
                add(home)
                (1..radius).forEach { addAll(hexRing(it)) }
            }
            hexen.forEach { hexagon ->
                val blackNeighbors = hexagon.neighbors().count { flipMap[it] ?: false }
                val isBlack = flipMap[hexagon] ?: false
                newMap[hexagon] = if (isBlack) { // if black
                    blackNeighbors in 1..2
                } else {
                    blackNeighbors == 2
                }
            }
            flipMap = newMap
        }
        return flipMap.count { it.value }
    }
}

fun main() = Day.runDay(Y20D24::class)

//    Class creation: 39ms
//    Part 1: 244 (2ms)
//    Part 2: 3665 (1101ms)
//    Total time: 1142ms
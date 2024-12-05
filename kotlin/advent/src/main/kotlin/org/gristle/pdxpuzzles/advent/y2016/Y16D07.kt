package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.utilities.iteration.collate

class Y16D07(input: String) : Day {
    private val ips: List<List<List<String>>> = input.lines().map { it.split('[', ']').collate(2) }
    private fun String.abba(): Boolean = windowed(4).any { it[0] == it[3] && it[0] != it[1] && it[1] == it[2] }
    private fun String.aba(): List<String> = windowed(3)
        .filter { it[0] == it[2] && it[0] != it[1] }
        .map { "${it[1]}${it[0]}${it[1]}" }

    override fun part1() = ips
        .count { (supernets, hypernets) -> supernets.any { it.abba() } && hypernets.none { it.abba() } }

    override fun part2() = ips
        .count { (supernets, hypernets) ->
            supernets
                .flatMap { supernet -> supernet.aba() }
                .any { aba -> hypernets.any { aba in it } }
        }
}

fun main() = Day.runDay(Y16D07::class)

//    Class creation: 51ms
//    Part 1: 118 (11ms)
//    Part 2: 260 (17ms)
//    Total time: 80ms
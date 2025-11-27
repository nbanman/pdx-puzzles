package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getIntList
import kotlin.math.ceil

object Y25D16 : Day {
    private fun sumBricks(spell: List<Int>, len: Long): Long = spell.sumOf { len / it }
    private fun spell(notes: String): List<Int> {
        val wall = notes.getIntList().toMutableList()
        val spell = mutableListOf<Int>()
        for (n in 1..wall.size) {
            if (wall[n - 1] > 0) {
                spell.add(n)
                for (k in (n - 1 until wall.size).step(n)) {
                    wall[k] -= 1
                }
            }
        }
        return spell
    }
    override fun part1(input: String): Long = sumBricks(input.getIntList(), 90L)
    override fun part2(input: String): Long = spell(input).fold(1L) { acc, i -> acc * i }
    override fun part3(input: String): Long {
        val spell = spell(input)
        val blocks = 202_520_252_025_000L
        val highest = spell.last().toDouble()
        val portionSum = spell.sumOf { highest / it }
        val low = ceil(blocks / (portionSum / highest)).toLong()
        val gallop = 10
        var high = low + gallop
        var highSum = sumBricks(spell, high)
        while (highSum <= blocks) {
            if (highSum == blocks) return high
            high += gallop
            highSum = sumBricks(spell, high)
        }



        return 3
    }
}

fun main() = Day.runDay(Y25D16::class)

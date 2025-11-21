package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.pow

object Y25D14 : Day {
    @JvmInline
    value class Floor(val value: List<ULong>) {
        companion object {
            fun from(s: String): Floor {
                val value = s.lines().map { line ->
                    line.fold(0uL) { acc, c ->
                        when (c) {
                            '#' -> acc shl 1 or 1uL
                            '.' -> acc shl 1
                            '\n' -> acc
                            else -> error("$c not a valid character")
                        }
                    }
                }
                return Floor(value)
            }
        }

        fun next(mask: ULong): Floor {
            val shaken = value.asSequence().map { row ->
                ((row shl 1) and mask) xor (row shr 1)
            }
            val inner = (sequenceOf(0uL) + shaken + sequenceOf(0uL))
                .windowed(3)
                .zip(value.asSequence())
                .map { (abut, row) -> (row xor (abut[0] xor abut[2])).inv() and mask }
                .toList()
            return Floor(inner)
        }

        fun active(): Int = value.sumOf { row -> row.countOneBits() }
    }

    fun sumAllActive(input: String, rounds: Int): Int {
        val floor = Floor.from(input)
        val mask = (2.pow(input.indexOf('\n')) - 1).toULong()
        return generateSequence(floor) { it.next(mask) }
            .drop(1)
            .take(rounds)
            .sumOf(Floor::active)
    }

    override fun part1(input: String): Int = sumAllActive(input, 10)
    override fun part2(input: String): Int = sumAllActive(input, 2025)
    override fun part3(input: String): Int {
        val floor = Floor(List(34) { 0uL })
        val center = input.fold(0uL) { acc, c ->
            when (c) {
                '#' -> acc shl 1 or 1uL
                '.' -> acc shl 1
                '\n' -> acc
                else -> error("'$c' should not be in input")
            }
        }

        val mask = 17179869183uL

        // skip the first round b/c it has nothing in it and does not cycle
        val totalRounds = 999_999_999
        val cycleLength = 4095
        val cycles = totalRounds / cycleLength
        val remainder = totalRounds % cycleLength
        var cycleSum = 0
        var remainderSum = 0

        for ((index, floor) in generateSequence(floor) { it.next(mask) }
            .withIndex()
            .drop(1)
            .take(cycleLength)
        ) {
            val floorCenter = floor.value.subList(13, 21).fold(0uL) { acc, row ->
                val trimmed = row shr 13 and 0xFFuL
                acc shl 8 or trimmed
            }
            if (floorCenter == center) {
                cycleSum += floor.active()
            }
            if (index == remainder) {
                remainderSum = cycleSum
            }
        }
        return cycleSum * cycles + remainderSum
    }
}

fun main() = Day.runDay(Y25D14::class)

//    Quest 1: 474 (5ms)
//    Quest 2: 1170584 (26ms)
//    Quest 3: 1012942728 (28ms)
//    Total time: 60ms
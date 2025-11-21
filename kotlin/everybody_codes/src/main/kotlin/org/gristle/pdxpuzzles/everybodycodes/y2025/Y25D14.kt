package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.pow

object Y25D14 : Day {
    @OptIn(ExperimentalUnsignedTypes::class)
    class Floor(val value: ULongArray, val shaken: ULongArray) {
        companion object {
            fun from(s: String): Floor {
                val value = s.lines()
                    .map { line ->
                        line.fold(0uL) { acc, c ->
                            when (c) {
                                '#' -> acc shl 1 or 1uL
                                '.' -> acc shl 1
                                '\n' -> acc
                                else -> error("$c not a valid character")
                            }
                        }
                    }.toULongArray()
                val shaken = ULongArray(value.size)
                return Floor(value, shaken)
            }
        }

        fun next(mask: ULong): Floor {
            for (i in 0 until shaken.size) {
                val row = value[i]
                shaken[i] = ((row shl 1) and mask) xor (row shr 1)
            }
            for ((i, row) in value.withIndex()) {
                val up = if (i == 0) 0uL else shaken[i - 1]
                val down = if (i == shaken.lastIndex) 0uL else shaken[i + 1]
                value[i] = mask and ((row xor (up xor down)).inv())
            }
            return this
        }

        fun active(): Int = value.sumOf { row -> row.countOneBits() }
    }

    @JvmInline
    value class SymmetricFloor(val value: IntArray = IntArray(17)) {
        fun next() {
            val mask = 131_071
            val shaken = IntArray(17) { i ->
                val row = value[i]
                val shiftLeft = (row shl 1) or (row and 1) and mask
                val shiftRight = row shr 1
                shiftLeft xor shiftRight
            }
            for ((i, row) in value.withIndex()) {
                val up = if (i == 0) 0 else shaken[i - 1]
                val down = if (i == 16) shaken[i] else shaken[i + 1]
                value[i] = mask and ((row xor (up xor down)).inv())
            }
        }

        fun active(): Int = value.sumOf { row -> row.countOneBits() } * 4
    }

    fun sumAllActive(input: String, rounds: Int): Int {
        val floor = Floor.from(input)
        val mask = (2.pow(input.indexOf('\n')) - 1).toULong()
        (0 until rounds)
        return generateSequence(floor) { it.next(mask) }
            .drop(1)
            .take(rounds)
            .sumOf(Floor::active)
    }

    override fun part1(input: String): Int = sumAllActive(input, 10)
    override fun part2(input: String): Int = sumAllActive(input, 2025)
    override fun part3(input: String): Int {
        val floor = SymmetricFloor()
        val center = input
            .take(input.length / 2)
            .foldIndexed(0) { idx, acc, c ->
                if (idx % 9 > 3) {
                    acc
                } else {
                    when (c) {
                        '#' -> acc shl 1 or 1
                        '.' -> acc shl 1
                        '\n' -> acc
                        else -> error("'$c' should not be in input")
                    }
                }
            }

        // skip the first round b/c it has nothing in it and does not cycle
        val totalRounds = 999_999_999
        val cycleLength = 4095
        val cycles = totalRounds / cycleLength
        val remainder = totalRounds % cycleLength
        var cycleSum = 0
        var remainderSum = 0

        for (index in 0 until cycleLength) {
            floor.next()
            val floorCenter = (13 until 17)
                .fold(0) { acc, rowIdx -> acc shl 4 or (floor.value[rowIdx] and 0xF) }
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

//    Quest 1: 474 (3ms)
//    Quest 2: 1170584 (6ms)
//    Quest 3: 1012942728 (2ms)
//    Total time: 12ms
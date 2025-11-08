package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.iteration.minMax
import org.gristle.pdxpuzzles.utilities.parsing.getLongList
import kotlin.math.sign

object Y25D05 : Day {
    fun concat(a: Long, b: Long): Long {
        var pow = 1
        while (pow <= b) {
            pow *= 10
        }
        return a * pow + b
    }

    data class Segment(var spine: Long, var left: Long? = null, var right: Long? = null) {
        fun place(n: Long): Boolean =
            if (n < spine && left == null) {
                left = n
                true
            } else if (n > spine && right == null) {
                right = n
                true
            } else {
                false
            }

        fun number(): Long = concat(
            concat(left ?: 0L, spine),
            right ?: 0
        )
    }

    data class Sword(val id: Long, val segments: List<Segment>): Comparable<Sword> {
        companion object {
            fun from(s: String): Sword {
                val ints = s.getLongList()
                val id = ints[0]
                val segments = mutableListOf(Segment(ints[1]))

                outer@for (int in ints.drop(2)) {
                    for (idx in segments.indices) {
                        val segment = segments[idx]
                        if (segment.place(int)) continue@outer
                    }
                    segments.add(Segment(int))
                }

                return Sword(id, segments)
            }
        }

        fun quality(): Long = segments.fold(0L) { acc, segment ->
            concat(acc, segment.spine)
        }

        override fun compareTo(other: Sword): Int {
            val qualityControl = quality() - other.quality()
            return if (qualityControl != 0L) {
                qualityControl.sign
            } else {
                val segmentControl = segments
                    .zip(other.segments)
                    .map { (a, b) -> a.number() - b.number() }
                    .firstOrNull { it != 0L }
                segmentControl?.sign ?: (id - other.id).sign
            }
        }
    }

    override fun part1(input: String) = Sword.from(input).quality()

    override fun part2(input: String) = input
        .lines()
        .map { Sword.from(it).quality() }
        .minMax()
        .let { (min, max) -> max - min }

    override fun part3(input: String) = let {
        val swords = input
            .lines()
            .map(Sword::from)
        val sortedSwords = swords
            .sortedDescending()
        sortedSwords
            .mapIndexed { index, sword -> (index + 1) * sword.id }
            .sum()
    }
}

fun main() = Day.runDay(Y25D05::class)

//    Quest 1: 2782784532 (0ms)
//    Quest 2: 8637361015798 (7ms)
//    Quest 3: 31574813 (9ms)
//    Total time: 18ms
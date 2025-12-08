package org.gristle.pdxpuzzles.advent.y2025

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.pow
import org.gristle.pdxpuzzles.utilities.objects.MCoord
import org.gristle.pdxpuzzles.utilities.objects.UnionFind
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import java.util.PriorityQueue
import kotlin.math.sqrt

class Y25D08(input: String) : Day {
    private val boxes = input.getInts().chunked(3).map(::MCoord).toList()

    private fun MCoord.dist(other: MCoord): Float {
        val inner = this.coordinates
            .zip(other.coordinates) { a, b -> (a - b).pow(2) }
            .sum()
        return sqrt(inner.toFloat())
    }

    private fun connections(boxes: List<MCoord>) = sequence<Pair<Int, Int>> {
        val lengths = PriorityQueue<Triple<Float, Int, Int>>(compareBy { it.first })
        for (i in 0 until boxes.lastIndex) {
            for (j in i + 1 until boxes.size) {
                lengths.add(Triple(boxes[i].dist(boxes[j]), i, j))
            }
        }
        while (lengths.isNotEmpty()) yield(lengths.poll().let { it.second to it.third })
    }

    override fun part1(): Int {
        val lights = UnionFind.new(boxes.size)
        for ((a, b) in connections(boxes).take(1_000)) {
            lights.union(a, b)
        }

        return lights.size.sortedDescending().take(3).reduce(Int::times)
    }

    override fun part2(): Long {
        val lights = UnionFind.new(boxes.size)
        for ((a, b) in connections(boxes)) {
            lights.union(a, b)
            val rootSize = lights.size.max()
            if (rootSize == boxes.size) {
                return boxes[a][0].toLong() * boxes[b][0]
            }
        }
        error("Unreachable!")
    }
}

fun main() = Day.runDay(Y25D08::class)

//    Class creation: 5ms
//    Part 1: 181584 (95ms)
//    Part 2: 8465902405 (79ms)
//    Total time: 180ms
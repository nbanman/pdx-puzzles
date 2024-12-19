package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit
import kotlin.math.absoluteValue
import kotlin.math.ceil

class Y24D19(input: String) : Day {
    private val towels: List<Pair<Int, Int>>
    private val designs: List<String>
    private val towelRef: List<Pair<String, Int>>

    init {
        val stanzas = input.blankSplit().map { it.split(", ", "\n") }
        towels = stanzas[0].map { it.length to it.toBitSet() }.sortedBy { it.second }
        towelRef = stanzas[0].map { it to it.toBitSet() }.sortedBy { it.second }
        designs = stanzas[1]
    }

    private val initialChunkSize = towels.last().second.stripes()
    private val cache = mutableMapOf<String, Long>()

    private fun Int.stripes() = ceil((32 - this.countLeadingZeroBits()) / 3.0).toInt()

    private fun Char.toBits() = when (this) {
        'w' -> 1
        'u' -> 2
        'b' -> 3
        'r' -> 4
        'g' -> 5
        else -> throw IllegalStateException("unrecognized stripe")
    }
    private fun String.toBitSet() = fold(0) { acc, c -> (acc shl 3) + c.toBits() }

    private fun String.checkDesign(): Boolean {
        if (isEmpty()) return true
        var chunk = take(initialChunkSize).toBitSet()
        var currentSize = chunk.stripes()
        val startIndex = towels
            .binarySearch { (_, bitset) -> bitset - chunk }
            .let { if (it >= 0) it else it.absoluteValue.coerceIn(towels.indices) }

        for (i in startIndex downTo 0) {
            val (size, towel) = towels[i]
            val delta = currentSize - size
            if (delta > 0) {
                currentSize -= delta
                chunk = chunk shr (delta * 3)
            }
            if (towel == chunk) {
                if (drop(size).checkDesign()) return true
            }
        }
        return false
    }

    private fun String.countTheWays(): Long = cache.getOrPut(this) {
        if (isEmpty()) return@getOrPut 1
        var count = 0L
        var chunkString = take(initialChunkSize)
        var chunk = take(initialChunkSize).toBitSet()
        var currentSize = chunk.stripes()
        val startIndex = towels
            .binarySearch { (_, bitset) -> bitset - chunk }
            .let { if (it >= 0) it else it.absoluteValue.coerceIn(towels.indices) }

        for (i in startIndex downTo 0) {
            val (size, towel) = towels[i]
            val delta = currentSize - size
            if (delta > 0) {
                currentSize -= delta
                chunkString = chunkString.dropLast(delta)
                chunk = chunk shr (delta * 3)
            }
            if (towel == chunk) {
                count += drop(size).countTheWays()
            }
        }
        count
    }

    override fun part1(): Int = designs.count { it.checkDesign() }
    override fun part2(): Long = designs.sumOf { design -> design.countTheWays() }
}

fun main() = Day.runDay(Y24D19::class)

//    Class creation: 7ms
//    Part 1: 238 (21ms)
//    Part 2: 635018909726691 (59ms)
//    Total time: 88ms

@Suppress("unused")
private val test = listOf("""r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb""")
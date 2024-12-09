package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.isEven

class Y24D09(val input: String) : Day {

    private fun List<Int>.checksum() = foldIndexed(0L) { index, acc, i -> acc + index * i }

    override fun part1(): Long {
        val fragmented = buildList {

            for ((idx, n) in input.map { it.digitToInt() }.withIndex()) {
                if (idx.isEven()) {
                    repeat(n) { add(idx / 2) }
                } else {
                    repeat(n) { add(null) }
                }
            }
        }
        var left = 0
        var right = fragmented.lastIndex
        val blocks = fragmented.count { it != null }
        val defragged = buildList {
            while (left < blocks) {
                while (fragmented[left] != null && left < right) add(fragmented[left++]!!)
                while (fragmented[right] == null) right--
                left++
                add(fragmented[right--]!!)
            }
        }
        return defragged.checksum()
    }

    private sealed interface Block {
        val order: Int
        val index: Int
        val size: Int

        fun checksum(): Long
    }

    private class Data(
        override val order: Int,
        override val index: Int,
        override val size: Int,
        val value: Int,
        var defragged: Boolean
    ) : Block {
        override fun checksum(): Long = checksum(index)
        fun checksum(index: Int): Long = (index.toLong() until (index + size).toLong()).sumOf { it * value }
    }

    private class Space(
        override val order: Int,
        override val index: Int,
        override var size: Int,
        val data: MutableList<Data>
    ) : Block {
        override fun checksum(): Long = data
            .dropLast(1)
            .runningFold(index) { acc, data -> acc + data.size  }
            .zip(data)
            .sumOf { (index, datum) -> datum.checksum(index) }
    }

    override fun part2(): Long {
        var index = 0
        val spaces = mutableListOf<Space>()
        val data = buildList {
            for ((order, n) in input.map { it.digitToInt() }.withIndex()) {
                if (order.isEven()) {
                    add(Data(order, index, n, order / 2, false))
                } else {
                    spaces.add(Space(order, index, n, mutableListOf()))
                }
                index += n
            }
        }
        val spacesPosterity = spaces.toList()
        for (datum in data.reversed()) {
            // checks that a) there is room; and b) that the space is not to the right of the data
            spaces.find { it.size >= datum.size && it.index < datum.index }?.let { space ->
                datum.defragged = true
                space.data.add(datum)
                space.size -= datum.size
                if (space.size == 0) spaces.remove(space)
            }
        }
        return (data.filter { !it.defragged } + spacesPosterity).sumOf { it.checksum() }
    }
}

fun main() = Day.runDay(Y24D09::class)
@Suppress("unused")
private val test = listOf("12345", "2333133121414131402")

//    Class creation: 2ms
//    Part 1: 6390180901651 (26ms)
//    Part 2: 6412390114238 (200ms)
//    Total time: 230ms
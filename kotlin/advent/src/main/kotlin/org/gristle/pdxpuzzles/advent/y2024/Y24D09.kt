package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.isEven
import java.util.PriorityQueue

class Y24D09(val input: String) : Day {

    override fun part1(): Long {
        val fragmented = buildList {
            for ((idx, n) in input.map { it.digitToInt() }.withIndex()) {
                val value = if (idx.isEven()) idx / 2 else null
                repeat(n) { add(value) }
            }
        }
        var left = 0
        var right = fragmented.lastIndex
        val blocks = fragmented.count { it != null }
        var checksum = 0L

        while (left < blocks) {
            while (fragmented[left] != null && left < right) checksum += left * fragmented[left++]!!
            while (fragmented[right] == null) right--
            checksum += left * fragmented[right--]!!
            left++
        }
        return checksum
    }

    private class Block(val initialIndex: Int, val size: Int, val value: Int) {
        fun checksum(): Long = checksum(initialIndex)
        fun checksum(index: Int): Long = (index.toLong() until (index + size).toLong())
            .sumOf { it * value }
    }

    override fun part2(): Long {
        val spaces = List(10) { PriorityQueue<Int>() }
        var index = 0
        val blocks = buildList {
            for ((order, size) in input.map(Char::digitToInt).withIndex()) {
                if (size > 0) {
                    if (order.isEven()) {
                        add(Block(index, size, order / 2))
                    } else {
                        spaces[size].add(index)
                    }
                    index += size
                }
            }
        }
        var checksum = 0L
        for (block in blocks.reversed()) {
            val indexes = spaces.subList(block.size, spaces.size)
                .withIndex()
                .mapNotNull { (heapIndex, space) ->
                    space.peek()?.let { spaceIndex ->
                        if (spaceIndex < block.initialIndex) {
                            spaceIndex to heapIndex
                        } else {
                            null
                        }
                    }
                }.minByOrNull { it.first }

            if (indexes == null) {
                checksum += block.checksum()
                continue
            }
            val (spaceIndex, heapOffset) = indexes
            val heapIndex = heapOffset + block.size
            spaces[heapIndex].poll()
            if (block.size < heapIndex) {
                spaces[heapIndex - block.size].add(spaceIndex + block.size)
            }
            checksum += block.checksum(spaceIndex)
        }
        return checksum
    }
}

fun main() = Day.runDay(Y24D09::class)
@Suppress("unused")
private val test = listOf("12345", "2333133121414131402")

//    Class creation: 1ms
//    Part 1: 6390180901651 (18ms)
//    Part 2: 6412390114238 (34ms)
//    Total time: 55ms
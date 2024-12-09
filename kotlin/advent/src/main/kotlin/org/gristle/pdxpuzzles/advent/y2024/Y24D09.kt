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
        val defragged = buildList {
            while (left < blocks) {
                while (fragmented[left] != null && left < right) add(fragmented[left++]!!)
                while (fragmented[right] == null) right--
                left++
                add(fragmented[right--]!!)
            }
        }
        return defragged.foldIndexed(0L) { index, acc, i -> acc + index * i }
    }

    private class Block(var index: Int, val size: Int, val value: Int) {
        fun checksum(): Long = (index.toLong() until (index + size).toLong())
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
        for (block in blocks.reversed()) {
            val (spaceIndex, heapOffset) = spaces.subList(block.size, spaces.size)
                .withIndex()
                .mapNotNull { (heapIndex, space) ->
                    space.peek()?.let { spaceIndex ->
                        if (spaceIndex < block.index) {
                            spaceIndex to heapIndex
                        } else {
                            null
                        }
                    }
                }.minByOrNull { it.first }
                ?: continue
            val heapIndex = heapOffset + block.size
            spaces[heapIndex].poll()
            block.index = spaceIndex
            if (block.size < heapIndex) {
                spaces[heapIndex - block.size].add(spaceIndex + block.size)
            }
        }
        return blocks.sumOf { it.checksum() }
    }
}

fun main() = Day.runDay(Y24D09::class)
@Suppress("unused")
private val test = listOf("12345", "2333133121414131402")

//    Class creation: 2ms
//    Part 1: 6390180901651 (26ms)
//    Part 2: 6412390114238 (172ms)
//    Total time: 201ms
package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.y2017.shared.denseHash
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y17D14(input: String) : Day {
    private fun stringRep(input: String): String {
        val preparation = input
            .map { it.code } + listOf(17, 31, 73, 47, 23)

        val denseHash = denseHash(preparation)

        val binary = denseHash.map { c ->
            Integer
                .parseInt(c.toString(), 16)
                .let { Integer.toBinaryString(it) }
        }

        val ret = buildString {
            binary.forEach {
                val leadingZeros = "0".repeat(4 - it.length)
                append(leadingZeros + it)
            }
        }
        return ret
    }

    private val rows = List(128) { i ->
        stringRep("$input-$i")
    }

    override fun part1() = rows.sumOf { row ->
        row.count { it == '1' }
    }

    override fun part2(): Int {
        val grid = rows.joinToString("\n").toGrid { it == '1' }
        var regions = 0
        val visited = BooleanArray(grid.size)
        val queue: MutableList<Int> = ArrayList(300)

        for ((pos, _) in grid.withIndex().filter { it.value }) {
            if (visited[pos]) continue
            regions++
            queue.add(pos)
            while (queue.isNotEmpty()) {
                val current = queue.removeLast()
                for ((nPos, used) in grid.getNeighborsIndexedValue(current)) {
                    if (used && !visited[nPos]) {
                        visited[nPos] = true
                        queue.add(nPos)
                    }
                }
            }
        }
        return regions
    }
}

fun main() = Day.runDay(Y17D14::class)

//    Class creation: 298ms
//    Part 1: 8222 (3ms)
//    Part 2: 1086 (21ms)
//    Total time: 322ms
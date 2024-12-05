package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.y2017.shared.denseHash
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y17D14(input: String) : Day {

    private fun allLinks(links: Map<Int, List<Int>>, seed: Int): List<Int> {

        tailrec fun aL(found: MutableSet<Int>, evaluate: Set<Int>): Set<Int> {
            return if (evaluate.isEmpty()) {
                found
            } else {
                found.addAll(evaluate)
                val newRegisters = evaluate.fold(mutableListOf<Int>()) { acc, i ->
                    acc.apply { addAll(links.getValue(i).filter { it !in found }) }
                }.toSet()
                aL(found, newRegisters)
            }
        }
        return aL(mutableSetOf(), setOf(seed)).toList()
    }

    private fun stringRep(input: String): String {
        val preparation = input
            .map { it.code } + listOf(17, 31, 73, 47, 23)

        val denseHash = denseHash(preparation)

        val binary = denseHash.map { c ->
            Integer
                .parseInt(c.toString(), 16)
                .let { Integer.toBinaryString(it) }
        }

        return buildString {
            binary.forEach {
                val leadingZeros = "0".repeat(4 - it.length)
                append(leadingZeros + it)
            }
        }
    }

    private val rows = List(128) { i ->
        stringRep("$input-$i")
    }

    override fun part1() = rows.sumOf { row -> row.count { it == '1' } }

    override fun part2(): Int {
        val width = rows.first().length
        val grid = List(width * rows.size) { index ->
            val x = index % width
            val y = index / width
            rows[y][x]
        }.toGrid(width)

        val registers = mutableMapOf<Int, List<Int>>()

        grid.forEachIndexed { index, c ->
            if (c == '1') {
                registers[index] = grid.getNeighborIndices(index).filter { grid[it] == '1' }
            }
        }

        val linkSet = registers.keys.toMutableSet()

        return generateSequence(linkSet) { it.apply { removeAll(allLinks(registers, linkSet.first()).toSet()) } }
            .indexOfFirst { it.isEmpty() }
    }
}

fun main() = Day.runDay(Y17D14::class)

//    Class creation: 442ms
//    Part 1: 8222 (1ms)
//    Part 2: 1086 (105ms)
//    Total time: 549ms
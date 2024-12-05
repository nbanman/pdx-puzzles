package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y17D06(input: String) : Day {

    private val initialState: List<Int> = input.split('\t').map { it.toInt() }

    private fun List<Int>.reallocate(): List<Int> {
        val (index, alloc) = withIndex().maxBy { it.value }
        val newList = toMutableList()
        newList[index] = 0
        for (i in 1..alloc) {
            newList[(index + i) % size]++
        }
        return newList
    }

    private val allocations: IndexedValue<Pair<Set<List<Int>>, List<Int>>> by lazy {
        val allocationSequence =
            generateSequence(mutableSetOf<List<Int>>() to initialState) { (set, last) ->
                set.apply { add(last) } to last.reallocate()
            }
        allocationSequence
            .withIndex()
            .first { (_, value) -> value.second in value.first }
    }

    override fun part1(): Int = allocations.index

    override fun part2(): Int = allocations
        .let { (index, value) ->
            val (set, last) = value
            index - set.indexOf(last)
        }
}

fun main() = Day.runDay(Y17D06::class)

//    Class creation: 5ms
//    Part 1: 12841 (20ms)
//    Part 2: 8038 (13ms)
//    Total time: 39ms

package org.gristle.pdxpuzzles.everybodycodes.y2025

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.runBlocking
import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.StringGrid

object Y25D12 : Day {
    private fun parse(input: String): Pair<StringGrid, BooleanArray> {
        val clearing = StringGrid(input)
        val visited = BooleanArray(clearing.string.length)
        return clearing to visited
    }

    private fun solve(clearing: StringGrid, visited: BooleanArray, todo: MutableList<Int>): Int {
        var todo = todo
        var next = mutableListOf<Int>()
        for (pos in todo) {
            visited[pos] = true
        }

        while (todo.isNotEmpty()) {
            for (pos in todo) {
                val barrel = clearing.string[pos]
                for (adjPos in clearing.getNeighborIndices(pos)) {
                    if (!visited[adjPos] && barrel >= clearing.string[adjPos]) {
                        visited[adjPos] = true
                        next.add(adjPos)
                    }
                }
            }
            todo.clear()
            todo = next.also { next = todo }
        }

        return visited.count { it }
    }

    override fun part1(input: String): Int {
        val (clearing, visited) = parse(input)
        val start = mutableListOf(0)
        return solve(clearing, visited, start)
    }

    override fun part2(input: String): Int {
        val (clearing, visited) = parse(input)
        val start = mutableListOf(0, input.length - 1)
        return solve(clearing, visited, start)
    }
    override fun part3(input: String): Int = runBlocking(Dispatchers.Default) {
        val (clearing, visited) = parse(input)
        var history = visited.clone()
        val winners = mutableListOf<Int>()

        val peaks = clearing.string.withIndex()
            .filter { (idx, c) ->
                clearing.getNeighborIndices(idx).all { adj -> c > clearing[adj] }
            }.map { it.index }

        repeat(3) {
            val (_, exploded, winner) = coroutineScope {
                peaks
                    .filter { it !in winners }
                    .map { pos ->
                        async {
                            val exploded = history.clone()
                            val barrels = solve(clearing, exploded, mutableListOf(pos))
                            Triple(barrels, exploded, pos)
                        }
                    }.awaitAll()
                    .maxBy { (barrels) -> barrels }
            }
            winners.add(winner)
            history = exploded
        }
        solve(clearing, visited, winners)
    }
}

fun main() = Day.runDay(Y25D12::class)

//    Quest 1: 240 (2ms)
//    Quest 2: 5731 (7ms)
//    Quest 3: 4135 (210ms)
//    Total time: 220ms
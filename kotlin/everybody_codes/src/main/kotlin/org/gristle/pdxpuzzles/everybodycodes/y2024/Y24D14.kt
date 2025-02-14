package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Xyz
import java.util.ArrayDeque

object Y24D14 : Day {
    override fun part1(input: String): Int = growBranch(input).maxOf { it.y }

    override fun part2(input: String): Int = input
        .lineSequence()
        .flatMap { line -> growBranch(line) }
        .toSet()
        .size

    override fun part3(input: String): Int {
        val branches: List<List<Xyz>> = input
            .lines()
            .map { line -> growBranch(line).toList() }
        val leaves = branches
            .map { branch -> branch.last() }
        val tree = branches.flatten().toSet()
        val treeHeight = tree.maxOf { it.y }
        val tapPoints = List(treeHeight) { IntArray(leaves.size) }
        val offsets = listOf(
            Xyz(1, 0, 0),
            Xyz(-1, 0, 0),
            Xyz(0, 1, 0),
            Xyz(0, -1, 0),
            Xyz(0, 0, 1),
            Xyz(0, 0, -1),
        )
        for ((leafIndex, leaf) in leaves.withIndex()) {
            var taps = 0
            val q = ArrayDeque<Pair<Xyz, Int>>()
            q.add(leaf to 0)
            val visited = mutableSetOf<Xyz>()
            while (q.isNotEmpty()) {
                val (pos, weight) = q.pop()

                // skip because already visited
                if (!visited.add(pos)) continue

                // trunk found
                if (pos.x == 0 && pos.z == 0) {
                    tapPoints[pos.y - 1][leafIndex] = weight

                    // end condition met
                    if (++taps == treeHeight) break
                }

                // get neighbors
                offsets
                    .map { pos + it }
                    .filter { it !in visited && it in tree }
                    .map { (x, y, z) -> Xyz(x, y, z) to weight + 1 }
                    .forEach { state -> q.add(state) }
            }
        }
        return tapPoints.map { it.sum() }.filter { it != 0 }.min()
    }

    private fun String.splitAt(index: Int): Pair<String, String> {
        require(index in 0..length) { "Index must be within the bounds of the string length." }
        return substring(0, index) to substring(index)
    }

    private fun growBranch(input: String) = input
        .splitToSequence(',')
        .flatMap { instruction ->
            val (dir, dist) = instruction.splitAt(1)
            val direction = dir[0]
            val distance = dist.toInt()
            generateSequence { direction }.take(distance)
        }.runningFold(Xyz(0, 0, 0)) { pos, direction ->
            pos + when (direction) {
                'U' -> Xyz(0, 1, 0)
                'D' -> Xyz(0, -1, 0)
                'L' -> Xyz(-1, 0, 0)
                'R' -> Xyz(1, 0, 0)
                'F' -> Xyz(0, 0, 1)
                'B' -> Xyz(0, 0,-1)
                else -> throw IllegalArgumentException("Unrecognized direction: $direction")
            }
        }.drop(1) // initial value for runningFold not part of branch

}

fun main() = Day.runDay(Y24D14::class)

//    Quest 1: 154 (15ms)
//    Quest 2: 4908 (25ms)
//    Quest 3: 1512 (248ms)
//    Total time: 289ms
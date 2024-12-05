package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y15D24(input: String) : Day {

    private val lines = input.lines()

    private fun List<Int>.product() = this.fold(1L, Long::times)

    private fun uniqueCombos(validCombos: List<List<Int>>, taken: List<List<Int>>, uniqueGroups: Int): List<Int>? {
        if (validCombos.isEmpty()) {
            return null
        } else {
            for (i in validCombos.indices) {
                if (validCombos[i].intersect(taken.flatten().toSet()).isEmpty()) {
                    return if (uniqueGroups > 3) {
                        val subUnique = uniqueCombos(
                            validCombos.drop(i),
                            taken + listOf(validCombos[i]),
                            uniqueGroups - 1
                        )
                        if (subUnique == null) continue else taken.first()
                    } else {
                        taken.first()
                    }
                }
            }
            return null
        }
    }

    private val weights = lines
        .map(String::toInt)
        .sortedDescending()

    data class Package(val weight: Int, val range: IntRange)

    fun solve(numberOfGroups: Int): Long {
        val groupWeight = weights.sum() / numberOfGroups

        val packages = weights.mapIndexed { i, weight ->
            val lowEnd = groupWeight - weights.drop(i).sum()
            val highEnd = groupWeight - weight
            Package(weight, lowEnd..highEnd)
        }

        var combos = packages
            .filter { 0 in it.range }
            .map { listOf(it.weight) }

        var validCombos = mutableListOf<List<Int>>().apply {
            addAll(combos.filter { it.sum() == groupWeight })
        }

        // do a pass
        while (combos.isNotEmpty()) {
            val newCombos = mutableListOf<List<Int>>()

            val startIndex = packages.indexOf(packages.find { it.weight == combos.first().last() }) + 1

            var i = startIndex
            loop@while (i < packages.size) {
                for (combo in combos) {
                    if (combo.last() <= packages[i].weight) {
                        i++
                        continue@loop
                    }
                    if (combo.sum() in packages[i].range) {
                        val latestCombo = combo + packages[i].weight
                        if (latestCombo.sum() == groupWeight) {
                            validCombos.add(latestCombo)
                        } else {
                            newCombos.add(latestCombo)
                        }
                    }
                }
                i++
            }
            combos = newCombos
            if (validCombos.size >= numberOfGroups - 1) {
                validCombos = validCombos.sortedBy { it.product() }.toMutableList()
                val unique = uniqueCombos(validCombos.drop(1), listOf(validCombos.first()), 3)
                if (unique != null) {
                    return unique.product()
                }
            }
        }
        return -1L
    }


    override fun part1() = solve(3)

    override fun part2() = solve(4)
}

fun main() = Day.runDay(Y15D24::class)

//    Class creation: 19ms
//    Part 1: 11846773891 (230ms)
//    Part 2: 80393059 (2ms)
//    Total time: 253ms
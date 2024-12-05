package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.combinatorics.getPermutations
import org.gristle.pdxpuzzles.utilities.parsing.groupValues

class Y15D13(input: String) : Day {

    private val arrangements: Map<String, Map<String, Int>> = buildMap<String, MutableMap<String, Int>> {
        val pattern = """(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).""".toRegex()
        input
            .groupValues(pattern)
            .forEach { (p1, gainOrLose, units, p2) ->
                val happinessUnits = units.toInt().let { if (gainOrLose == "gain") it else -it }
                getOrPut(p1) { mutableMapOf() }[p2] = happinessUnits
            }
    }

    private val people: Set<String> = arrangements.keys

    fun solve(people: Set<String>): Int = people
        .drop(1) // remove first person because the arrangement has to start somewhere
        .getPermutations() // get all permutations of remaining people
        .map { it + people.first() } // re-add the first person to all permutations
        .maxOf { peopleList -> // calculate the happiness units for each permutation and return the maximum
            val pairs = peopleList.zipWithNext() + (peopleList.last() to peopleList.first())
            pairs.sumOf { (left, right) ->
                val leftRight = arrangements[left]?.get(right) ?: 0
                val rightLeft = arrangements[right]?.get(left) ?: 0
                leftRight + rightLeft
            }
        }

    override fun part1() = solve(people)

    override fun part2() = solve(people + "me")
}

fun main() = Day.runDay(Y15D13::class)

//    Class creation: 23ms
//    Part 1: 664 (47ms)
//    Part 2: 640 (138ms)
//    Total time: 209ms
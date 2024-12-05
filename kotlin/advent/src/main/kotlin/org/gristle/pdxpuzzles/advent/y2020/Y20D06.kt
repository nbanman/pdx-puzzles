package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit

class Y20D06(input: String) : Day {
    // Read input and split into separate groups.
    private val groups: List<List<Set<Char>>> = input
        .blankSplit()
        .map { it.split('\n').map(String::toSet) }

    // Both parts involve looking at each group separately, counting the answers in a particular way, then 
    // returning the sum of those counts. 
    fun solve(fn: Set<Char>.(Set<Char>) -> Set<Char>) = groups.sumOf { group -> group.reduce(fn).size }

    // For each group, count the number of questions to which *anyone* answered "yes."
    override fun part1() = solve(Set<Char>::union)

    // For each group, count the number of questions to which *everyone* answered "yes."
    override fun part2() = solve(Set<Char>::intersect)
}

fun main() = Day.runDay(Y20D06::class)

//    Class creation: 19ms
//    Part 1: 6297 (4ms)
//    Part 2: 3158 (4ms)
//    Total time: 29ms
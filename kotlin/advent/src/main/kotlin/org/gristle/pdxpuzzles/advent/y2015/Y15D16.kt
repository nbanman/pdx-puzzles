package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.gvs

class Y15D16(input: String) : Day {

    // regex to extract items and amounts from a string
    private val pattern = Regex("""([a-z]+): (\d+)""")

    // takes a string and makes a "Sue." Items mapped to associated amounts
    private fun buildSue(s: String): Map<String, Int> = buildMap {
        s.gvs(pattern).forEach { (item, amt) -> put(item, amt.toInt()) }
    }

    // parses input
    private val sues: List<Map<String, Int>> = input
        .lineSequence()
        .map(::buildSue)
        .toList()

    // the gift-giving sue is defined by the ticker tape given in the puzzle
    private val auntSue: Map<String, Int>

    init {
        val tickerTape = """children: 3\n" +
            "cats: 7\n" +
            "samoyeds: 2\n" +
            "pomeranians: 3\n" +
            "akitas: 0\n" +
            "vizslas: 0\n" +
            "goldfish: 5\n" +
            "trees: 3\n" +
            "cars: 2\n" +
            "perfumes: 1"""
        auntSue = buildSue(tickerTape)
    }

    // returns true when for each item that the gift-giving Sue has, the Sue in question either doesn't have
    // an entry or has the same amount. Otherwise, returns false.
    private fun modernRetroencabulator(sue: Map<String, Int>): Boolean = auntSue.entries
        .all { (item, amt) -> (sue[item] ?: amt) == amt }

    // returns true when for each item that the gift-giving Sue has, the Sue in question either doesn't have
    // an entry or has the correct amount as defined in part 2 of the puzzle. Otherwise, returns false.
    private fun outdatedRetroencabulator(sue: Map<String, Int>): Boolean = auntSue.entries
        .all { (item, amt) ->
            if (sue.contains(item)) { // if Sue in question has an entry for the item, further analysis needed...
                val sueAmt = sue.getValue(item)
                when (item) {
                    "cats", "trees" -> sueAmt > amt
                    "pomeranians", "goldfish" -> sueAmt < amt
                    else -> sueAmt == amt
                }
            } else { // Sue in question has no entry, so move on to next item
                true
            }
        }

    override fun part1() = sues.indexOfFirst(::modernRetroencabulator) + 1

    override fun part2() = sues.indexOfFirst(::outdatedRetroencabulator) + 1
}

fun main() = Day.runDay(Y15D16::class)

//    Class creation: 39ms
//    Part 1: 40 (2ms)
//    Part 2: 241 (1ms)
//    Total time: 43ms
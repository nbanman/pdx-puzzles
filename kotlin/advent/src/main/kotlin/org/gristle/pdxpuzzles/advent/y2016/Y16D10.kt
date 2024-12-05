package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.groupValues
import org.gristle.pdxpuzzles.utilities.parsing.gvs
import org.gristle.pdxpuzzles.utilities.iteration.minMax

class Y16D10(input: String) : Day {

    // Parse bots from input and store the instructions in a map.
    private val bots: Map<String, Pair<String, String>> = input
        .groupValues("""(bot \d+) gives low to ((?:bot|output) \d+) and high to ((?:bot|output) \d+)""")
        .associate { (id, low, high) -> id to (low to high) }

    // Part 1 asks for the number of the bot responsible for comparing 61 and 17. This is incidental to the 
    // overall chip assignment process, so assign a null variable, and when the two numbers are compared, assign
    // the bot number to the variable.
    private var responsibleBot: Int? = null

    // Assigns a chip to a bot or output. If a chip is assigned to a bot already holding a chip, the function is
    // called recursively to make assignments in accordance with the bot instructions.
    private fun MutableMap<String, Int>.assign(recipient: String, value: Int) {

        // Gets the chip already assigned to the recipient, or null if none assigned.
        val current = get(recipient)

        if (current == null) { // if no chip assigned to recipient...
            put(recipient, value) // assign chip to recipient...
        } else { // ...otherwise, execute instructions of recipient
            val (low, high) = minMax(current, value)

            // If the recipient is given both 17 and 61, the recipient number is the answer for part 1. 
            if (low == 17 && high == 61) {
                responsibleBot = recipient.takeLastWhile { it.isDigit() }.toInt()
            }

            // Reassign the chips in accordance with the instructions of that bot 
            val (lowRecipient, highRecipient) = bots.getValue(recipient)
            assign(lowRecipient, low)
            assign(highRecipient, high)

            // Remove bot from the registry, as it now holds no chips
            remove(recipient)
        }
    }

    // Registry of assigned values. Chips are seeded by assignments in the input, but get moved around by the bots
    // once each bot receives two chips.
    private val registry: Map<String, Int> = buildMap {
        input
            .gvs("""value (\d+) goes to (bot \d+)""")
            .forEach { (value, bot) -> assign(bot, value.toInt()) }
    }

    override fun part1() = responsibleBot ?: "Responsible bot not found."

    override fun part2() = registry
        .entries
        .filter { (id, _) -> id.takeLastWhile { it.isDigit() }.toInt() <= 2 } // only look at bins 0, 1, and 2
        .fold(1) { acc, (_, value) -> acc * value } // multiply values together
}

fun main() = Day.runDay(Y16D10::class)

//    Class creation: 29ms
//    Part 1: 101 (0ms)
//    Part 2: 37789 (0ms)
//    Total time: 29ms
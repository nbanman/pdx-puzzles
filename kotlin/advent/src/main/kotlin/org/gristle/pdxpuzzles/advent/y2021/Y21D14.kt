package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.iteration.minMax
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit

private typealias PropagationRules = Map<String, Pair<String, String>>
private typealias ProteinPairs = Map<String, Long>

class Y21D14(input: String) : Day {

    private val proteinPairs: ProteinPairs
    private val edgeProteins: String
    private val rules: PropagationRules

    init {
        // Intermediate parsing step for defining above values, splitting the template input from the rules input
        val (templateInput, rulesInput) = input.blankSplit()

        // Initial polymer state represented as a map of pairs of proteins along with the number of times the pair
        // exists in the String.    
        proteinPairs = buildMap {
            templateInput.windowed(2).forEach { key ->
                this[key] = (this[key] ?: 0L) + 1 // bumps count
            }
        }

        // The protein counting algorithm does not properly count proteins that are at the ends of the polymer, 
        // so those proteins need to be retained.
        edgeProteins = "${templateInput.first()}${templateInput.last()}"

        // Propagation rules. The solving algorithm counts number of neighboring proteins.
        // Thus, the insertion rule is represented by a pair of strings. "AB => C" becomes "AC" to "CB"
        rules = buildMap {
            rulesInput // start with input string
                .filter { it.isLetter() } // discard formatting and only look at "proteins"
                .toList() // convert to List<Char> for nifty destructuring
                .chunked(3) // groups of 3 proteins
                .forEach { (a, b, c) -> put("$a$b", "$a$c" to "$c$b") } // assign values to map
        }
    }

    /**
     * The protein expands exponentially, so rather than try to store the protein as a string, the algorithm
     * updates a map that keeps track of how many times a pair has been encountered. Then each step, that pair
     * re-propagates using the propagation rules, and the resulting pairs' amounts are increased by the amount of
     * the original pair.
     */
    private fun solve(steps: Int): Long = proteinPairs
        .polymerize(steps) // Runs the stepping process n number of times.
        .countProteins() // Count tne number of each protein.
        .minMax() // Grab the highest and lowest values
        .let { (min, max) -> max - min } // return the difference

    /**
     * Runs the polymerization process n number of times.
     */
    private tailrec fun ProteinPairs.polymerize(steps: Int): ProteinPairs {
        return if (steps == 0) {
            this
        } else {
            val newPairs = mutableMapOf<String, Long>()
            for ((proteins, amt) in this) {
                val (a, b) = rules.getValue(proteins)
                newPairs[a] = (newPairs[a] ?: 0L) + amt
                newPairs[b] = (newPairs[b] ?: 0L) + amt
            }
            newPairs.polymerize(steps - 1)
        }
    }

    /**
     * Take the map of protein pairs and return a list of counts of proteins.
     */
    private fun ProteinPairs.countProteins(): List<Long> {

        // iterates through the pairs and adds the pair count amount to each individual protein. Note that this
        // double-counts the proteins because they occupy the first position in one pair and the last position
        // in another pair.
        val doubleCounts: Map<Char, Long> = buildMap {
            for ((proteins, amt) in this@countProteins) {
                for (protein in proteins) {
                    this[protein] = (this[protein] ?: 0L) + amt
                }
            }

            // Proteins at the very beginning and end are not double-counted, so bump the count for these to make
            // it consistent. 
            for (protein in edgeProteins) {
                this[protein] = (this[protein] ?: 0L) + 1L
            }
        }

        // Return a list of counts by dividing by 2.
        return doubleCounts.map { it.value / 2 }
    }

    override fun part1() = solve(10)
    override fun part2() = solve(40)
}

fun main() = Day.runDay(Y21D14::class)

//    Class creation: 10ms
//    Part 1: 3555 (4ms)
//    Part 2: 4439442043739 (3ms)
//    Total time: 18ms
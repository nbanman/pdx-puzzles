package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y23D12(input: String) : Day {
    data class SpringRow(val conditions: String, val damageReport: List<Int>) {
        // expands the SpringRow in accordance with Part 2 rules
        fun expand(): SpringRow = SpringRow(
            List(5) { conditions }.joinToString("?"),
            List(5) { damageReport }.flatten()
        )
        
        // cache used for DP function 'arrangements,' below. 
        private val cache = mutableMapOf<Pair<Int, Int>, Long>()
        
        // DP function that uses state objects to delve deeper and deeper into the string using DFS, until a base case 
        // is found. Cache is updated along the way, so that subsequent dives down the string become instant as soon as
        // there is any repetition of remaining state.
        fun arrangements(
            conditionsIndex: Int = 0,
            damageReportIndex: Int = 0
        ): Long = cache.getOrPut(conditionsIndex to damageReportIndex) {
            // Base case. Takes states that have assigned broken springs corresponding to the entire damage report 
            // and returns 1 if valid, 0 if invalid. Valid states are those with no remaining '#' in the string.
            if (damageReportIndex == damageReport.size) {
                val damagedSpringRemaining = (conditionsIndex..conditions.lastIndex).any { conditions[it] == '#' }
                return@getOrPut if (damagedSpringRemaining) 0 else 1
            }
            // Otherwise, we go recursive by trying to fit the next group of broken springs in every place along the 
            // block. This starts as a sequence of indexes, from 0 until the length of the block minus the 
            // fulfillment size (to account for the size of the fulfillment itself in the string).
            val brokenGroup = damageReport[damageReportIndex]
            (conditionsIndex..conditions.length - brokenGroup)
                .asSequence()
                // stop the sequence if a '#' precedes the index b/c '#' cannot be skipped
                .takeWhile { index -> conditions.getOrNull(index - 1) != '#' }
                .filter { index ->
                    // filter out invalid placements, in cascading fashion
                    // if the placement includes a '.', invalid b/c '.' means not broken
                    // if the placement has no part of the string after it, valid b/c nothing else to consider
                    // if the character following the placement is '#', invalid b/c that extra '#' would overfulfill
                    // otherwise valid
                    when {
                        (index until index + brokenGroup).any { conditions[it] == '.' } -> false
                        index + brokenGroup == conditions.length -> true
                        conditions[index + brokenGroup] == '#' -> false
                        else -> true
                    }
                }.sumOf { index -> arrangements(index + brokenGroup + 1, damageReportIndex + 1) }
        }
    }

    // parsing input to SpringRows
    private val springRows: List<SpringRow> = input.lines().map { line ->
        val (conditions, damageReportStr) = line.split(' ')
        val damageReport: List<Int> = damageReportStr.split(',').map(String::toInt)
        SpringRow(conditions, damageReport)
    }

    override fun part1(): Long = springRows.sumOf(SpringRow::arrangements)
    
    override fun part2(): Long = springRows.map(SpringRow::expand).sumOf(SpringRow::arrangements)
}

fun main() = Day.runDay(Y23D12::class)

//    Class creation: 17ms
//    Part 1: 7344 (18ms)
//    Part 2: 1088006519007 (135ms)
//    Total time: 171ms

@Suppress("unused")
private val sampleInput = listOf(
    """???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"""
)
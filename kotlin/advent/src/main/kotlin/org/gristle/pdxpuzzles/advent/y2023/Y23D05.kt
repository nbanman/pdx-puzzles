package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit
import org.gristle.pdxpuzzles.utilities.parsing.getLongList
import kotlin.math.min

class Y23D05(input: String) : Day {

    data class Listing(val sourceStart: Long, val offset: Long, val length: Long) {
        val sourceEnd = sourceStart + length - 1
    }

    private val seeds: List<Long>
    private val conversions: List<List<Listing>>

    init {
        val stanzas = input.blankSplit().map { it.getLongList() }
        seeds = stanzas[0]
        conversions = stanzas
            .drop(1)
            .map { mapNumbers ->
                mapNumbers
                    .chunked(3) { (destinationStart, sourceStart, length) ->
                        Listing(sourceStart, destinationStart - sourceStart, length)
                    }.sortedBy(Listing::sourceStart)
            }
    }

    private fun solve(seedRanges: List<LongRange>): Long = seedRanges
        .minOf { seedRange -> // for each seedRange, find the smallest end number
            // feed a list of ranges through a gauntlet of conversion stages. the list of ranges will grow each step
            // as each conversion handles different parts of the range differently.
            val subRanges = conversions.fold(listOf(seedRange)) { ranges, listings ->
                ranges.flatMap { range ->
                    buildList {
                        // go through each listing in ascending order, adding subRanges where appropriate, mapping to
                        // destination where appropriate. In order to avoid using a mutable "last" variable, we
                        // put everything in a fold that updates an internal "next" and ultimately outputs a "last"
                        // value to use to complete the subRanges once we've run through all the listings.
                        val last = listings.fold(range.first) { next, listing ->
                            if (range.last >= listing.sourceStart && next <= listing.sourceEnd) {
                                if (next < listing.sourceStart) {
                                    add(next until listing.sourceStart)
                                }
                                val mapEnd = min(range.last, listing.sourceEnd)
                                add(next + listing.offset..mapEnd + listing.offset)
                                mapEnd + 1
                            } else {
                                next
                            }
                        }
                        // clean up subRanges
                        if (last <= range.last) add(last..range.last)
                    }
                }
            }
            // we only care about the lowest number of all these subRanges
            subRanges.minOf { it.first }
        }

    override fun part1() = solve(seeds.map { it..it })

    override fun part2() = solve(seeds.chunked(2) { (start, length) -> start until start + length })
}

fun main() = Day.runDay(Y23D05::class)

//    Class creation: 11ms
//    Part 1: 379811651 (4ms)
//    Part 2: 27992443 (4ms)
//    Total time: 20ms

@Suppress("unused")
private val sampleInput = listOf(
    """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4""",
)
package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.mapToGridIndexed
import org.gristle.pdxpuzzles.utilities.objects.toGrid

typealias CollectionArea = Grid<Char>

class Y18D18(input: String) : Day {

    // Provides successive generations of the collection area, starting at minute 0 (initial).
    private val generator: Sequence<CollectionArea> = generateSequence(input.toGrid()) { prev ->
        prev.mapToGridIndexed { index, acre ->
            val neighbors = prev.getNeighbors(index, true)
            when (acre) { // particular game of life rules
                '.' -> if (neighbors.count { it == '|' } >= 3) '|' else acre
                '|' -> if (neighbors.count { it == '#' } >= 3) '#' else acre
                else -> if ('#' in neighbors && '|' in neighbors) '#' else '.'
            }
        }
    }

    // Calculates resource value of a collection area by counting the number of '|' and '#' and multiplying them.
    private fun CollectionArea.resourceValue() = count { it == '|' } * count { it == '#' }

    // Generate 10 minutes' worth of changes, then get the resource value.
    override fun part1() = generator
        .take(11) // take 11 instead of 10 because the first is minute 0 (initial state)
        .last() // grab last generated value
        .resourceValue() // get resource value

    // Generates new states and stores the states in a cache. When the new state is the same as a previous state,
    // stop generating. The cache plus the first repeat value provides the information needed to provide the state
    // after 1 billion generations.
    override fun part2(): Int {
        // Cache is a simple *ordered* Set of states. 
        val cache = mutableSetOf<CollectionArea>()

        // Runs the generator, with each minute adding the latest state into the cache. Stops and returns the first
        // state that is contained within the cache; i.e., is a repeated state.
        val repeated = generator.first { area -> !cache.add(area) }

        // The index of the first instance of a repeated state, which means all values with an index lower than that
        // are not repeats.
        val repeatStartIndex = cache.indexOf(repeated)

        // We want to get the state in the cache that we would have if we ran 1 billion times. The values before
        // the repeatStartIndex are unique values before the permutations stabilized. So the "base" index for the
        // value is repeatStartIndex. From there, we add an index value corresponding to the place in the repeating
        // loop. Once we get that state we return its resource value.
        return cache
            .elementAt(repeatStartIndex + (1_000_000_000 - repeatStartIndex) % (cache.size - repeatStartIndex))
            .resourceValue()
    }
}

fun main() = Day.runDay(Y18D18::class)

//    Class creation: 23ms
//    Part 1: 605154 (50ms)
//    Part 2: 200364 (375ms)
//    Total time: 448ms
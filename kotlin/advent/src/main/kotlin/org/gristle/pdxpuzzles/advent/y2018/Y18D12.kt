package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y18D12(input: String) : Day {
    private val lines = input.lines()

    // parse initial row of plants
    private val initialRow = lines[0].drop(15).map { it == '#' }.toBooleanArray()

    private val commands = lines
        .drop(2)
        .filter { it.last() == '#' }
        .map { line ->
            line.take(5).fold(0) { acc, ch -> (acc shl 1) + if (ch == '#') 1 else 0 }
        }.toSet()

    // sequence that provides successive generations of plant rows
    private val generator = generateSequence(initialRow) { plant ->
        BooleanArray(plant.size + 4) { i ->
            val index = i - 2
            val rng = (index - 2).coerceAtLeast(0)..(index + 2).coerceAtMost(plant.lastIndex)
            var pattern = rng.fold(0) { acc, i ->
                (acc shl 1) + if (plant[i]) 1 else 0
            }
            pattern = pattern shl (index + 2 - plant.lastIndex).coerceAtLeast(0)
            pattern in commands
        }
    }

    /**
     * take the row and obtain the sum of pot numbers. "generations" is used to account for growth of the row
     */
    private fun BooleanArray.sumOfPotNumbers(generations: Int) =
        mapIndexed { index, b -> if (b) (index - (generations * 2)) else 0 }
            .sum()

    override fun part1(): Int {
        val generations = 20
        // take the sequence, run it 21 times (first time yields the initial row), then get the pot number sum
        // of the last row generation generated.
        return generator
            .take(generations + 1) // the generator also includes the original row, thus generations + 1
            .last() // terminate sequence on last row
            .sumOfPotNumbers(generations) // get sum of pot numbers
    }

    override fun part2(): Long {
        // too many generations to naively compute!
        val generations = 50_000_000_000

        // upon observation, the growth is chaotic at first but then finds a stable pattern where growth is constant.
        // Thus, the strategy is to look at generations 10 at a time. When the difference between each is the 
        // same, we can surmise that the growth has stabilized. That group provides enough information to 
        // solve part 2.
        val groupSize = 10
        val firstStable: List<IndexedValue<Int>> = generator
            .withIndex() // pair up rows with their index, which is the number of generations
            // transform rows to their pot sum
            .map { (index, value) -> IndexedValue(index, value.sumOfPotNumbers(index)) }
            // look at generations 10 at a time
            .chunked(groupSize)
            // terminate and provide the first group where the difference in pot sums between each in the group is
            // the same
            .first { group ->
                group
                    // compare each value with the next one, map to the difference in value between the two
                    .zipWithNext { a, b -> b.value - a.value }
                    .groupingBy { it }
                    .eachCount() // separate into groups based on the difference
                    .size == 1 // return true if there is only one group; i.e., they all have the same difference
            }

        // the last "chaotic" value obtained by the generator
        val lastUnstableValue: Int = firstStable.first().value

        // the generation of this last chaotic value, so we don't double count generations when applying the stable
        // generation count
        val repeatIndex: Int = firstStable.first().index

        // the amount that each successive generation adds to the pot number count
        val stableIncrement = firstStable.let { it[1].value - it[0].value }

        // putting it all together
        return lastUnstableValue + stableIncrement * (generations - repeatIndex)
    }
}

fun main() = Day.runDay(Y18D12::class)

//    Class creation: 21ms
//    Part 1: 4110 (3ms)
//    Part 2: 2650000000466 (45ms)
//    Total time: 70ms
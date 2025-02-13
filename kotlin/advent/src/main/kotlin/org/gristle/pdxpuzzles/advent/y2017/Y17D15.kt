package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y17D15(input: String) : Day {
    private val seedA: Long
    private val seedB: Long

    init {
        val seeds = input
            .lineSequence()
            .map { line -> line.takeLastWhile { it.isDigit() }.toLong() }
            .toList()
        seedA = seeds.first()
        seedB = seeds.last()
    }

    private val factorA = 16807
    private val factorB = 48271

    /* The generators both work on the same principle. To create its next value, a generator will take the previous 
    value it produced, multiply it by a factor (generator A uses 16807; generator B uses 48271), and then keep the 
    remainder of dividing that resulting product by 2147483647. That final remainder is the value it produces next.
        
    To calculate each generator's first value, it instead uses a specific starting value as its "previous value" 
    (as listed in your puzzle input). */
    private fun generator(seed: Long, factor: Int, multiples: Int? = null): Sequence<Short> =
        generateSequence(seed) { (it * factor) % Int.MAX_VALUE }
            .drop(1) // drop the seed value
            .let { sequence -> // filter for part2 rules (skip for part1)   
                if (multiples == null) sequence else sequence.filter { it % multiples == 0L }
            }.map(Long::toShort) // only look at lowest 16 bits

    /**
     * Used for both parts. Zips the two generator sequences together to create one sequence that generates pairs
     * of values generated by the generators. Returns how often the pairs match given the number of comparisions.
     */
    fun solve(comparisons: Int, generatorA: Sequence<Short>, generatorB: Sequence<Short>): Int =
        (generatorA zip generatorB) // join the two Generator sequences together
            .take(comparisons) // number of times the judge compares the generated values
            .count { (aValue, bValue) -> aValue == bValue } // count how many of those values compared are equal

    override fun part1() = solve(
        40_000_000,
        generator(seedA, factorA),
        generator(seedB, factorB)
    )

    override fun part2() = solve(
        5_000_000,
        generator(seedA, factorA, 4),
        generator(seedB, factorB, 8)
    )
}

fun main() = Day.runDay(Y17D15::class)

//    Class creation: 16ms
//    Part 1: 594 (1300ms)
//    Part 2: 328 (962ms)
//    Total time: 2279ms
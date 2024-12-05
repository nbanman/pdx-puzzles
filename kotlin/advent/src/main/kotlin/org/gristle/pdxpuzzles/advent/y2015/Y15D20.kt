package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.isEven
import kotlin.math.sqrt

class Y15D20(input: String) : Day {

    private val minimumPresents = input.toInt()
    private fun primeFactors(number: Int): List<Int> {
        val factors = mutableListOf<Int>()
        var n = number

        while (n.isEven()) {
            factors.add(2)
            n /= 2
        }

        for (i in 3..sqrt(n.toFloat()).toInt() step 2) {
            while (n % i == 0) {
                factors.add(i)
                n /= i
            }
        }

        if (n > 2) factors.add(n)

        return factors
    }

    private tailrec fun expandFactors(primeFactors: List<Int>, factors: List<Int> = listOf(1)): List<Int> {
        return if (primeFactors.isNotEmpty()) {
            val latest = primeFactors
                .dropLastWhile { it != primeFactors.first() }
                .drop(1)
                .runningFold(primeFactors.first(), Int::times)
            val newFactors = factors.fold(listOf<Int>()) { acc, i ->
                acc + listOf(i) + latest.map { it * i }
            }
            expandFactors(primeFactors - latest.toSet(), newFactors)
        } else {
            factors
        }
    }

    fun solve(
        multiplier: Int,
        predicate: (houseNumber: Int, elf: Int) -> Boolean = { _, _ -> true }
    ) = generateSequence(1) { it + 1 }
        .indexOfFirst { houseNumber ->
            val elves = expandFactors(primeFactors(houseNumber)).filter { predicate(houseNumber, it) }
            val presents = elves.fold(0) { acc, i -> acc + i * multiplier }
            presents >= minimumPresents
        } + 1


    override fun part1() = solve(10)

    override fun part2() = solve(11) { houseNumber, elf -> elf * 50 > houseNumber }
}

fun main() = Day.runDay(Y15D20::class)

//    Class creation: 16ms
//    Part 1: 776160 (4049ms)
//    Part 2: 786240 (3216ms)
//    Total time: 7282ms
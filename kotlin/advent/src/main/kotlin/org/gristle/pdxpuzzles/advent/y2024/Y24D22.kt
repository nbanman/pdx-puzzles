package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getLongList

class Y24D22(input: String) : Day {

    private val buyers = input.getLongList()

    private fun nextSecret(prev: Long): Long {
        val a = prev.mixAndPrune(prev * 64)
        val b = a.mixAndPrune(a / 32)
        return b.mixAndPrune(b * 2048)
    }

    private fun Long.mixAndPrune(n: Long): Long {
        val mix = this xor n
        val prune = mix % 16777216
        return prune
    }

    override fun part1(): Long = buyers.sumOf { generateSequence(it, ::nextSecret).take(2001).last() }

    override fun part2(): Int {
        val exchangeRate = mutableMapOf<Long, MutableMap<List<Int>, Int>>()
        for (buyer in buyers) {
            val prices = generateSequence(buyer, ::nextSecret)
                .take(2001)
                .map { n -> (n % 10).toInt() }

            val changesSeq = prices
                .zipWithNext()
                .map { (a, b) -> b - a }
                .windowed(4)

            for ((price, changes) in prices.drop(4) zip changesSeq) {
                val buyerRate = exchangeRate.getOrPut(buyer) { mutableMapOf() }
                if (buyerRate.containsKey(changes)) continue
                buyerRate[changes] = price
            }
        }

        // add up buyer rates
        val totalRates = buildMap {
            for ((_, rate) in exchangeRate) {
                for ((changes, price) in rate) {
                    val newPrice = (get(changes) ?: 0) + price
                    this[changes] = newPrice
                }
            }
        }

        return totalRates.values.max()
    }
}

fun main() = Day.runDay(Y24D22::class)

//    Class creation: 3ms
//    Part 1: 16953639210 (40ms)
//    Part 2: 1863 (945ms)
//    Total time: 989ms
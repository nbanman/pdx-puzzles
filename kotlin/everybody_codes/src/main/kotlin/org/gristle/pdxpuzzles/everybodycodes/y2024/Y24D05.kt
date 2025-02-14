package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.pow
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import java.util.LinkedList

object Y24D05 : Day {
    private fun parseInput(input: String): Pair<Int, List<LinkedList<Int>>> {
        val numberOfColumns = 1 + input
            .takeWhile { it != '\n' }
            .count { it == ' ' }
        val columns = List(numberOfColumns) { LinkedList<Int>() }
        input.getInts().forEachIndexed { idx, n ->
            columns[idx % numberOfColumns].addLast(n)
        }
        return numberOfColumns to columns
    }

    private fun playRound(round: Int, numberOfColumns: Int, columns: List<LinkedList<Int>>): String {
        val clapperCol = (round - 1) % numberOfColumns
        val nextCol = round % numberOfColumns
        val nextLen = columns[nextCol].size
        val clapper = columns[clapperCol].removeFirst()
        val pos = ((clapper - 1) % (nextLen * 2))
            .let { it.coerceAtMost(nextLen) - (it - nextLen).coerceAtLeast(0) }
        columns[nextCol].add(pos, clapper)
        return columns
            .map { column -> column.peekFirst() }
            .joinToString("")
    }

    override fun part1(input: String): Int {
        val (numberOfColumns, columns) = parseInput(input)
        return generateSequence(1) { round -> round + 1 }
            .map { round -> playRound(round, numberOfColumns, columns) }
            .take(10)
            .last()
            .toInt()
    }

    override fun part2(input: String): Long {
        val (numberOfColumns, columns) = parseInput(input)
        val digits = input.lineSequence().first().count { it.isDigit() }
        val counter = IntArray(10.pow(digits).toInt())
        return generateSequence(1) { round -> round + 1 }
            .map { round ->
                val shouted = playRound(round, numberOfColumns, columns).toInt()
                round to shouted
            }.first { (_, shouted) ->
                counter[shouted] += 1
                counter[shouted] == 2024
            }.let { (round, shouted) -> round.toLong() * shouted }
    }

    override fun part3(input: String): Long {
        val (numberOfColumns, columns) = parseInput(input)
        val cache = mutableSetOf<String>()
        var highestNumber = 0L
        var round = 1
        do {
            val shouted = playRound(round, numberOfColumns, columns).toLong()
            if (highestNumber < shouted) highestNumber = shouted
            val state = columns
                .flatMap { column -> column.map { n -> n.toString() } }
                .joinToString("")
            round++
        } while (cache.add(state))
        return highestNumber
    }
}

fun main() = Day.runDay(Y24D05::class)

//    Quest 1: 2444 (2ms)
//    Quest 1: 21128465428212 (425ms)
//    Quest 1: 4974100410041002 (394ms)
//    Total time: 821ms
package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y17D16(input: String) : Day {

    sealed interface DanceMove {
        @JvmInline
        value class Spin(val value: Int) : DanceMove
        class Exchange(val a: Int, val b: Int) : DanceMove
        class Partner(val a: Char, val b: Char) : DanceMove
    }

    // parsing input to DanceMoves
    private val danceMoves: List<DanceMove> by lazy {
        input
            .split(',')
            .map {
                when (it[0]) {
                    's' -> DanceMove.Spin(it.drop(1).toInt())
                    'x' -> it.drop(1).split('/').let { (a, b) -> DanceMove.Exchange(a.toInt(), b.toInt()) }
                    'p' -> it.drop(1).split('/').let { (a, b) -> DanceMove.Partner(a[0], b[0]) }
                    else -> throw IllegalArgumentException("Cannot parse input: $it")
                }
            }
    }

    // takes a string, runs all dance moves on it, and returns a string
    private fun danceParty(positions: String): String {

        // convert String to mutable primitive array for performance
        val pos = positions.toCharArray()

        // rather than reorder entire array after each spin, just keep track of the offset and reverse it at the end 
        var offset = 0

        // execute each instruction
        for (danceMove in danceMoves) {
            when (danceMove) {
                is DanceMove.Exchange -> {
                    val a = (danceMove.a - offset).mod(pos.size)
                    val b = (danceMove.b - offset).mod(pos.size)
                    pos[a] = pos[b].also { pos[b] = pos[a] }
                }

                is DanceMove.Partner -> {
                    val a = pos.indexOf(danceMove.a)
                    val b = pos.indexOf(danceMove.b)
                    pos[a] = pos[b].also { pos[b] = pos[a] }
                }

                is DanceMove.Spin -> offset = (danceMove.value + offset).mod(pos.size)
            }
        }

        // generate a string from the charArray, using the offset information to put the first character first, etc.
        return unShift(pos, offset)
    }

    private fun unShift(pos: CharArray, index: Int) = buildString {
        pos.indices.forEach { i -> append(pos[(i - index).mod(pos.size)]) }
    }

    private val start = "abcdefghijklmnop"

    // store results of first round so that it can be reused for part 2
    private val oneRound: String by lazy { danceParty(start) }

    override fun part1() = oneRound

    override fun part2(): String {

        // cache stores strings representing program arrangements that have already occurred  
        val cache = mutableSetOf(start)

        // run danceParty on its own output over and over again, storing each result to cache, until a duplicate is
        // found
        generateSequence(oneRound, ::danceParty)
            .first { !cache.add(it) }

        // use mod to calculate what would result if the dance party were run 1 billion times
        return cache.elementAt(1_000_000_000 % cache.size)
    }
}

fun main() = Day.runDay(Y17D16::class)

//    Class creation: 16ms
//    Part 1: hmefajngplkidocb (2ms)
//    Part 2: fbidepghmjklcnoa (35ms)
//    Total time: 53ms
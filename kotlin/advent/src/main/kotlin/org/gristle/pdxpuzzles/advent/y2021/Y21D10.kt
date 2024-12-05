package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import java.util.*

class Y21D10(input: String) : Day {
    private val lines = input.lines()

    // Functions and definitions to use with the "parse" function below.

    // pairs opening characters with corresponding closing characters
    private val counterparts = mapOf('(' to ')', '[' to ']', '{' to '}', '<' to '>')

    // character scoring provided for part1
    private val syntaxErrorScore = mapOf<Char, Long>(')' to 3, ']' to 57, '}' to 1197, '>' to 25137)

    // character scoring provided for part2
    private val pointValue = mapOf(')' to 1, ']' to 2, '}' to 3, '>' to 4)

    // completion string scoring per part2
    private fun Iterable<Char>.toScore() = fold(0L) { acc, c -> acc * 5 + pointValue.getValue(c) }

    /**
     * Parses each character in a string. If it's an opening character, add the corresponding closing character
     * to a stack. If it's a closing character, pop from the stack and compare the two. If they are the same, they
     * cancel out and continue. If they are different, then the string is corrupt.
     *
     * The function accepts two functions as parameters that return a nullable Long. onCorrupt gets called early
     * if the string is corrupt. onFinish gets called after every character in the string has been parsed.
     */
    private inline fun String.parse(
        onCorrupt: (Char) -> Long?,
        onFinish: (Iterable<Char>) -> Long?
    ): Long? {
        val stack: Deque<Char> = ArrayDeque()
        forEach { candidate ->
            if (candidate !in counterparts.values) {
                stack.push(counterparts[candidate])
            } else {
                if (candidate != stack.pop()) return onCorrupt(candidate)
            }
        }
        return onFinish(stack)
    }

    // sums the syntax error scores
    override fun part1() = lines
        .sumOf { line ->
            line.parse(
                onCorrupt = { syntaxErrorScore[it] },
                onFinish = { null }
            ) ?: 0 // only corrupt strings have a syntax error score 
        }

    override fun part2() = lines
        .mapNotNull { line ->
            line.parse(
                onCorrupt = { null }, // combined with mapNotNull above, this discards corrupt strings
                onFinish = { it.toScore() } // converts the closing characters in the stack to a score
            )
        }.sorted() // sorts the scores
        .let { it[it.size / 2] } // returns the middle score
}

fun main() = Day.runDay(Y21D10::class)

//    Class creation: 23ms
//    Part 1: 167379 (3ms)
//    Part 2: 2776842859 (3ms)
//    Total time: 31ms
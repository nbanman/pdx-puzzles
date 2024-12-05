package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getIntList

class Y18D09(input: String) : Day {

    private val players: Int
    private val highestValue: Int

    init {
        input.getIntList().let { (a, b) ->
            players = a
            highestValue = b
        }
    }

    class Dll<E>(val value: E, leftNode: Dll<E>? = null, rightNode: Dll<E>? = null) {
        var left = leftNode ?: this
        var right = rightNode ?: this
        fun addRight(other: Dll<E>): Dll<E> {
            other.left = this
            other.right = right
            right.left = other
            right = other
            return other
        }

        tailrec fun goLeft(n: Int, dll: Dll<E> = this): Dll<E> {
            if (n == 0) return dll 
            return goLeft(n - 1, dll.left)
        }

        fun remove(): Dll<E> {
            left.right = right
            right.left = left
            return right
        }

        override fun toString() = "$value"
    }

    fun solve(multiplier: Int = 1): Long {
        val lastMarble = highestValue * multiplier
        val scores = LongArray(players) { 0 }
        var currentMarble = Dll(0)
        for (x in 1..lastMarble) {
            if (x % 23 == 0) {
                currentMarble = currentMarble.goLeft(7)
                scores[((x - 1) % players)] = scores[((x - 1) % players)] + x + currentMarble.value
                currentMarble = currentMarble.remove()
            } else {
                currentMarble = currentMarble.right.addRight(Dll(x))
            }
        }
        return scores.max()
    }

    override fun part1() = solve()

    override fun part2() = solve(100)
}

fun main() = Day.runDay(Y18D09::class)

//    Class creation: 17ms
//    Part 1: 422980 (11ms)
//    Part 2: 3552041936 (505ms)
//    Total time: 533ms
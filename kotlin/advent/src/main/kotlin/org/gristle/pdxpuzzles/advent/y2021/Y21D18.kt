package org.gristle.pdxpuzzles.advent.y2021

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.runBlocking
import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.combinatorics.getPairs

class Y21D18(input: String) : Day {

    sealed interface Snailfish {
        fun split(): SplitStatus
        fun magnitude(): Int
        fun explode(level: Int): ExplodeStatus
        fun placeLeft(toPlace: Int)
        fun placeRight(toPlace: Int)
        fun clone(): Snailfish

        fun reduce() {
            while (true) {
                while (true) {
                    val status = this.explode(1)
                    if (status == Nothing) break
                }
                val status = this.split()
                if (status == NoSplit) break
            }
        }

        operator fun plus(other: Snailfish): Snailfish {
            val sf = Pair(this, other)
            sf.reduce()
            return sf
        }

        companion object {
            fun from(s: String): Snailfish {
                val charIterator = s.iterator()
                return from(charIterator)
            }

            private fun from(charIterator: CharIterator): Snailfish {
                val next = charIterator.nextChar()
                return when {
                    next == '[' -> {
                        val left = from(charIterator)
                        charIterator.nextChar()
                        val right = from(charIterator)
                        charIterator.nextChar()
                        Pair(left, right)
                    }
                    next.isDigit() -> {
                        Number(next.digitToInt())
                    }
                    else -> throw IllegalArgumentException("Invalid char: $next")
                }
            }
        }
    }

    data class Number(var v: Int) : Snailfish {
        override fun split(): SplitStatus = if (v >= 10) {
            val left = v / 2
            val right = left + if (v and 1 == 1) 1 else 0
            val pair = Pair(Number(left), Number(right))
            DoSplit(pair)
        } else {
            NoSplit
        }

        override fun magnitude(): Int = v

        override fun explode(level: Int): ExplodeStatus = Nothing

        override fun placeLeft(toPlace: Int) {
            v += toPlace
        }

        override fun placeRight(toPlace: Int) {
            v += toPlace
        }

        override fun clone(): Snailfish = Number(v)

        override fun toString(): String = v.toString()
    }

    data class Pair(var left: Snailfish, var right: Snailfish) : Snailfish {
        override fun split(): SplitStatus {
            when (val leftSplit = left.split()) {
                is DoSplit -> {
                    left = leftSplit.pair
                    return SplitDone
                }
                SplitDone -> return SplitDone
                NoSplit -> {}
            }
            return when (val rightSplit = right.split()) {
                is DoSplit -> {
                    right = rightSplit.pair
                    SplitDone
                }
                else -> rightSplit
            }
        }


        override fun magnitude(): Int = left.magnitude() * 3 + right.magnitude() * 2

        override fun explode(level: Int): ExplodeStatus {
            if (level >= 5) {
                if (left is Number) {
                    val lv = (left as Number).v
                    if (right is Number) {
                        val rv = (right as Number).v
                        return Place(lv, rv)
                    }
                }
            }
            when (val leftStatus = left.explode(level + 1)) {
                Nothing -> {} // continue to right...
                is Place -> {
                    left = Number(0)
                    right.placeLeft(leftStatus.rv)
                    return if (level == 1) {
                        this.explode(level)
                    } else {
                        PlaceLeft(leftStatus.lv)
                    }
                }
                is PlaceLeft -> {
                    return if (level == 1) {
                        this.explode(level)
                    } else {
                        leftStatus
                    }
                }
                is PlaceRight -> {
                    right.placeLeft(leftStatus.v)
                    return if (level == 1) {
                        this.explode(level)
                    } else {
                        Reduced
                    }
                }
                Reduced -> {
                    return if (level == 1) {
                        this.explode(level)
                    } else {
                        Reduced
                    }
                }
            }

            return when (val rightStatus = right.explode(level + 1)) {
                Nothing -> Nothing
                is Place -> {
                    right = Number(0)
                    left.placeRight(rightStatus.lv)
                    return if (level == 1) {
                        this.explode(level)
                    } else {
                        PlaceRight(rightStatus.rv)
                    }
                }
                is PlaceLeft -> {
                    left.placeRight(rightStatus.v)
                    return if (level == 1) {
                        this.explode(level)
                    } else {
                        Reduced
                    }
                }
                is PlaceRight -> {
                    if (level == 1) {
                        this.explode(level)
                    } else {
                        rightStatus
                    }
                }
                Reduced -> {
                    if (level == 1) {
                        this.explode(level)
                    } else {
                        rightStatus
                    }
                }
            }
        }

        override fun placeLeft(toPlace: Int) {
            left.placeLeft(toPlace)
        }

        override fun placeRight(toPlace: Int) {
            right.placeRight(toPlace)
        }

        override fun clone(): Snailfish = Pair(left.clone(), right.clone())

        override fun toString(): String = "[$left,$right]"
    }

    sealed interface ExplodeStatus
    data class PlaceLeft(val v: Int) : ExplodeStatus
    data class PlaceRight(val v: Int) : ExplodeStatus
    data class Place(val lv: Int, val rv: Int) : ExplodeStatus
    data object Reduced : ExplodeStatus
    data object Nothing : ExplodeStatus

    sealed interface SplitStatus
    data class DoSplit(val pair: Pair) : SplitStatus
    data object SplitDone : SplitStatus
    data object NoSplit : SplitStatus

    private val snailfish = input.lines().map(Snailfish::from)

    override fun part1(): Int {
        val snailfish = snailfish.map { it.clone() }
        val sum = snailfish.reduce(Snailfish::plus)
        return sum.magnitude()
    }

    override fun part2(): Int = runBlocking(Dispatchers.Default) {
        val comboChunks = snailfish
            .getPairs()
            .let { half ->
                val reversed = half.map { (a, b) -> b to a }
                half + reversed
            }
            .chunked(1000)
        comboChunks
            .map { combos ->
                async {
                    combos.maxOf { (a, b) -> (a.clone() + b.clone()).magnitude() }
                }
            }.awaitAll()
            .max()
    }
}

fun main() = Day.runDay(Y21D18::class)

//    Class creation: 2ms
//    Part 1: 3806 (10ms)
//    Part 2: 4727 (31ms)
//    Total time: 44ms
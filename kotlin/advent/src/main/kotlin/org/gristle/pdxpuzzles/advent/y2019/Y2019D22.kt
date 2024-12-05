package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.groupValues

class Y19D22(private val input: String) : Day {

    private val pattern = """(cut|deal)[^-\d\n]+(-?\d+)?""".toRegex()

    class Deck(input: String, private val deckSize: Long, numberOfTimes: Long) {
        sealed class Technique(val n: Long) {

            abstract fun nextPositionForCard(pos: Long, deckSize: Long): Long

            abstract fun combine(other: Technique, deckSize: Long): List<Technique>

            class Cut(n: Long) : Technique(n) {
                override fun nextPositionForCard(pos: Long, deckSize: Long) =
                    Math.floorMod(pos - n, deckSize)

                override fun combine(other: Technique, deckSize: Long): List<Technique> {
                    return if (other is Cut) {
                        listOf(Cut(Math.floorMod(n + other.n, deckSize)))
                    } else {
                        listOf(other, Cut(mulMod(n, other.n, deckSize)))
                    }
                }

                override fun toString(): String {
                    return "Cut(n=$n)"
                }
            }

            class Deal(n: Long) : Technique(n) {
                override fun nextPositionForCard(pos: Long, deckSize: Long) =
                    Math.floorMod(pos * n, deckSize)

                override fun combine(other: Technique, deckSize: Long): List<Technique> {
                    return if (other is Deal) {
                        listOf(Deal(mulMod(n, other.n, deckSize)))
                    } else {
                        listOf(this, other)
                    }
                }

                override fun toString(): String {
                    return "Deal(n=$n)"
                }
            }

            internal fun mulMod(a: Long, b: Long, mod: Long) =
                a.toBigInteger().multiply(b.toBigInteger()).mod(mod.toBigInteger()).toLong()

        }

        private val shuffles = deckSize - 1 - numberOfTimes

        private val techniques = input
            .groupValues("""(cut|deal)[^-0-9\n]+(-?\d+)?""")
            .flatMap { (technique, increment) ->
                if (technique == "cut") {
                    listOf(Technique.Cut(increment.toLong()))
                } else {
                    if (increment == "") {
                        listOf(Technique.Deal(-1), Technique.Cut(1))
                    } else listOf(Technique.Deal(increment.toLong()))
                }
            }.reduceAndRepeat()

        private fun List<Technique>.combined(): List<Technique> {
            var combinedList = this
            while (combinedList.size > 2) {
                combinedList = combinedList.drop(1).fold(listOf(first())) { acc, technique ->
                    acc.dropLast(1) + acc.last().combine(technique, deckSize)
                }
            }
            return combinedList
        }

        private fun List<Technique>.reduceAndRepeat(): List<Technique> {
            var current = this.combined()
            val returnList = mutableListOf<Technique>()
            for (bit in shuffles.toString(2).reversed()) {
                if (bit == '1') returnList.addAll(current)
                current = (current + current).combined()
            }
            return returnList.combined()
        }

        fun numberInPosition(pos: Long) = techniques.fold(pos) { acc, tech -> tech.nextPositionForCard(acc, deckSize) }
    }

    override fun part1(): Int {
        val cardsInDeck = 10_007
        val techniques = input.groupValues(pattern)
        var deck = IntArray(cardsInDeck) { it }
        techniques.forEach { tech ->
            if (tech[0] == "cut") {
                deck = deck.shift(tech[1].toInt())
            } else {
                if (tech[1] == "") {
                    deck.reverse()
                } else {
                    val increment = tech[1].toInt()
                    val newDeck = IntArray(deck.size)
                    for (i in deck.indices) {
                        newDeck[i * increment % deck.size] = deck[i]
                    }
                    deck = newDeck
                }
            }
        }

        return deck.indexOf(2019)
    }

    override fun part2(): Long {
        val p2CardsInDeck = 119_315_717_514_047
        val numberOfTimes = 101_741_582_076_661

        val deck = Deck(input, p2CardsInDeck, numberOfTimes)
        return deck.numberInPosition(2020L)
    }

    private fun IntArray.shift(n: Int): IntArray {
        val nMod = n % size
        val newIndex = if (nMod >= 0) nMod else size + nMod
        return sliceArray(newIndex..lastIndex) + sliceArray(0 until newIndex)
    }
}

fun main() = Day.runDay(Y19D22::class)

//    Class creation: 18ms
//    Part 1: 6129 (13ms)
//    Part 2: 71345377301237 (9ms)
//    Total time: 41ms
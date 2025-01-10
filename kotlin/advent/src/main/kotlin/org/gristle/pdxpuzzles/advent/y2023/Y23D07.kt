package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y23D07(input: String) : Day {
    private val hands = input.lines().map { line ->
        val (cards, bid) = line.split(' ')
        Hand(cards, bid.toInt())
    }

    // takes the hands, sorts by the hand strength as defined by each puzzle part, assigns points using rank and
    // bid amount, then returns sum of all points
    private fun solve(hands: List<Hand>, jokersWild: Boolean) = hands
        .map { hand -> hand.bid to hand.getStrength(jokersWild) }
        .sortedBy { (_, strength) -> strength }
        .mapIndexed { index, (bid, _) -> (index + 1) * bid }
        .sum()

    class Hand(private val cards: String, val bid: Int) {
        fun getStrength(jokersWild: Boolean): Int {
            val cardValues = cards.map { card ->
                when (card) {
                    'T' -> 10
                    'J' -> if (jokersWild) 1 else 11
                    'Q' -> 12
                    'K' -> 13
                    'A' -> 14
                    else -> card.digitToInt()
                }
            }
            val groups = IntArray(15)
            var cardStrength = 0
            for (value in cardValues) {
                groups[value]++
                cardStrength = (cardStrength shl 4) or value
            }
            val jokers: Int
            if (jokersWild) {
                jokers = groups[1]
                groups[1] = 0
            } else {
                jokers = 0
            }
            
            val (first, second) = groups.sortedDescending()

            val handStrength = ((first + jokers) shl 24) + (second shl 20) + cardStrength
            return handStrength
        }
    }

    override fun part1() = solve(hands, false)

    // Part 2 is the same thing, except 'J' cards are identified as jokers and are dealt with appropriately in the 
    // Hand class logic.
    override fun part2() = solve(hands, true)
}

fun main() = Day.runDay(Y23D07::class)

//    Class creation: 14ms
//    Part 1: 253866470 (10ms)
//    Part 2: 254494947 (2ms)
//    Total time: 27ms

@Suppress("unused")
private val sampleInput = listOf(
    """32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
""",
)
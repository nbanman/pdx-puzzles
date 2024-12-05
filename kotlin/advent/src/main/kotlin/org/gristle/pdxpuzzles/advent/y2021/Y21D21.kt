package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.iteration.max

class Y21D21(input: String) : Day {

    class DeterministicDie {
        private var die = 0

        var counter = 0L

        fun roll(n: Int): List<Int> {
            counter += n
            return (1..n).fold(listOf()) { acc, _ ->
                die = if (die == 100) 1 else die + 1
                acc + die
            }
        }
    }

    class Game(private val spaces: Int, private val die: DeterministicDie = DeterministicDie()) {

        data class GameResult(val winner: String, val winScore: Long, val loseScore: Long, val dieRolls: Long) {
            fun solve() = loseScore * dieRolls
        }

        private fun Int.advance(n: Int) = (this - 1 + n) % spaces + 1

        fun play(p1Start: Int, p2Start: Int, winCondition: Long = 1000L): GameResult {
            var p1Score = 0L
            var p2Score = 0L
            var p1Pos = p1Start
            var p2Pos = p2Start

            while (p2Score < winCondition) {
                val p1Roll = die.roll(3)
                p1Pos = p1Pos.advance(p1Roll.sum())
                p1Score += p1Pos
//                println("Player 1 rolls $p1Roll and moves to space $p1Pos for a total score of $p1Score.")
                if (p1Score >= winCondition) {
                    return GameResult("p1", p1Score, p2Score, die.counter)
                }
                val p2Roll = die.roll(3)
                p2Pos = p2Pos.advance(p2Roll.sum())
                p2Score += p2Pos
//                println("Player 2 rolls $p2Roll and moves to space $p2Pos for a total score of $p2Score.")
            }
            return GameResult("p2", p2Score, p1Score, die.counter)
        }
    }

    private val matchValues = """(\d+)${'$'}"""
        .toRegex(RegexOption.MULTILINE)
        .findAll(input)
        .map { it.value.toInt() }

    private val p1Start = matchValues.first()
    private val p2Start = matchValues.last()

    override fun part1(): Long {
        val game = Game(10)
        return game.play(p1Start, p2Start).solve()
    }

    private val rf = listOf(3 to 1, 4 to 3, 5 to 6, 6 to 7, 7 to 6, 8 to 3, 9 to 1)

    private fun wins(p1: Int, t1: Int, p2: Int, t2: Int): Pair<Long, Long> {
        if (t2 <= 0) return 0L to 1L

        var w1 = 0L
        var w2 = 0L
        for ((roll, frequency) in rf) {
            val (c2, c1) = wins(p2, t2, (p1 + roll) % 10, t1 - 1 - (p1 + roll) % 10)
            w1 += frequency * c1
            w2 += frequency * c2
        }
        return w1 to w2
    }

    override fun part2() = wins(p1Start - 1, 21, p2Start - 1, 21).max()
}

fun main() = Day.runDay(Y21D21::class)

//    Class creation: 17ms
//    Part 1: 605070 (2ms)
//    Part 2: 218433063958910 (1004ms)
//    Total time: 1024ms
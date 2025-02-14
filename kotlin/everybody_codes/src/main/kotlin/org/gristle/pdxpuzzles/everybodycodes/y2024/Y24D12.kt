package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.isOdd
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.parsing.getInts

object Y24D12 : Day {
    private enum class Ruins {
        BLOCK,
        ROCK,
    }

    private fun attackRuins(input: String): Int = getRuins(input).sumOf { target ->
        (0..2).asSequence()
            .mapNotNull { catapult -> getRanking(catapult, target) }
            .first()
    }

    private fun getRuins(input: String) = input
        .lines()
        .reversed()
        .drop(1)
        .flatMapIndexed { y, line ->
            line.withIndex()
                .mapNotNull { (x, c) ->
                    when (c) {
                        'T' -> Coord(x, y) to Ruins.BLOCK
                        'H' -> Coord(x, y) to Ruins.ROCK
                        else -> null
                    }
                }
        }.sortedWith(
            compareBy<Pair<Coord, Ruins>> { (pos) -> pos.x }.thenByDescending { (pos) -> pos.y }
        )


    private fun getRanking(catapult: Int, target: Pair<Coord, Ruins>): Int? {
        val (pos, ruin) = target
        val diff = pos - Coord(1, catapult)
        val adjX = diff.x + diff.y
        return if (adjX % 3 == 0) {
            val power = adjX / 3
            score(catapult, power, ruin)
        } else {
            null
        }
    }

    private fun score(catapult: Int, power: Int, ruins: Ruins) =
        (catapult + 1) * power * if (ruins == Ruins.BLOCK) 1 else 2

    private fun missileCommand(input: String): Int = input
        .getInts()
        .chunked(2) { (x, y) -> Coord(x, y) }
        .sumOf { meteor ->
            (0..2)
                .mapNotNull { catapult -> intercept(catapult, meteor) }
                .min()
        }

    private fun intercept(catapult: Int, meteor: Coord): Int? {
        val meteor = if (meteor.x.isOdd()) {
            meteor - Coord(1, 1)
        } else {
            meteor
        }

        val x = meteor.x / 2
        // Step 1: if xm - ym + offset == 0, then you get it on the upswing, and power is xm / 2
        if (meteor.x - meteor.y + catapult == 0) {
            val power = x
            val score = score(catapult, power, Ruins.BLOCK)
            return score
        }

        // Step 2: y == bc == p + offset equation
        // at x, y will have dropped by x amount
        val y = meteor.y - x
        val power = y - catapult
        if (x in power..power * 2) {
            val score = score(catapult, power, Ruins.BLOCK)
            return score
        }

        // Step 3: Apply formula from pts 1 + 2.
        val adjX = x + y - catapult
        return if (adjX % 3 == 0 && y - catapult <= x) {
            val power = adjX / 3
            score(catapult, power, Ruins.BLOCK)
        } else {
            null
        }
    }

    override fun part1(input: String): Int = attackRuins(input)
    override fun part2(input: String): Int = attackRuins(input)
    override fun part3(input: String): Int = missileCommand(input)
}

fun main() = Day.runDay(Y24D12::class)

//    Quest 1: 253 (4ms)
//    Quest 2: 20579 (6ms)
//    Quest 3: 729524 (6ms)
//    Total time: 17ms
package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getLongs

class Y24D13(input: String) : Day {
    data class ClawMachine(
        val a1: Long,
        val a2: Long,
        val b1: Long,
        val b2: Long,
        val c1: Long,
        val c2: Long,
    ) {
        fun minimumTokens(): Pair<Long, Long>? {
            // use elimination method of solving system of equations. Using these equations:
            //   1) a1x + b1y = c1
            //   2) a2x + b2y = c2
            // we'll find the lcm of a1 and a2. then we'll multiply equation 1 so that a1 is the lcm,
            // and we'll multiply equation 2 so that a2 is -lcm. Then add the two equations together,
            // which gives all we need to solve for y
            val commonMultiple = a1 * a2
            val f1 = commonMultiple / a1
            val f2 = -(commonMultiple / a2)
            val yNumerator = (c1 * f1 + c2 * f2)
            val yDenominator = (b1 * f1 + b2 * f2)
            val y = yNumerator / yDenominator
            if (y * yDenominator != yNumerator) return null
            val xNumerator = (c1 - b1 * y)
            val xDenominator = a1
            val x = xNumerator / xDenominator
            if (x * xDenominator != xNumerator) return null
            return x to y
        }
    }

    private val machines = input
        .getLongs()
        .chunked(6)
        .map { ClawMachine(it[0], it[1], it[2], it[3], it[4], it[5]) }
        .toList()

    override fun part1() = machines
        .mapNotNull { it.minimumTokens() }
        .filter { (a, b) -> a <= 100 && b <= 100 }
        .sumOf { (a, b) -> a * 3 + b }
    override fun part2() = machines
        .map { it.copy(c1 = it.c1 + 10000000000000L, c2 = it.c2 + 10000000000000L) }
        .mapNotNull { it.minimumTokens() }
        .sumOf { (a, b) -> a * 3 + b }
}

fun main() = Day.runDay(Y24D13::class)

//    Class creation: 8ms
//    Part 1: 37128 (4ms)
//    Part 2: 74914228471331 (4ms)
//    Total time: 17ms

private val test = listOf("""Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
""")
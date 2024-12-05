package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day

data class Command(val op: Ops, val p: Int, val a: Int, val b: Int, val c: Int) {
    fun execute(reg: LongArray) = op.fn(reg, p, a, b, c)
}

class Y18D19(input: String) : Day {
    private val data = input.lines()

    override fun part1(): Long {
        val p = data.first().takeLast(1).toInt()
        val commands = data
            .drop(1)
            .map { line ->
                line
                    .split(" ")
                    .let { fields ->
                        val ops = Ops.from(fields[0])
                        Command(ops, p, fields[1].toInt(), fields[2].toInt(), fields[3].toInt())
                    }
            }

        val register = LongArray(6)
        while (register[p] in commands.indices) {
            commands[register[p].toInt()].execute(register)
        }
        return register[0]
    }

    override fun part2(): Int {

        // Loop is such that R3 starts as 1, R5 goes up by 1. R2 is R3 * R5. When R2 equals 105k, R0+= R3 and R3++, R5 resets
        // If R2 goes past 105K w/o equaling it (not divisible), then R3++ and R5 resets w/o RO going up. Thus, RO adds all
        // the numbers that divide evenly into 105k.
        val targetNum = 10_551_370
        return (1..targetNum).fold(0) { acc, i ->
            if (targetNum % i == 0) acc + i else acc
        }
    }
}

fun main() = Day.runDay(Y18D19::class)

//    Class creation: 14ms
//    Part 1: 1764 (128ms)
//    Part 2: 18992484 (104ms)
//    Total time: 247ms

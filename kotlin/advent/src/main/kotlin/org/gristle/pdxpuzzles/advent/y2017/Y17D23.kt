package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.y2017.shared.Command
import org.gristle.pdxpuzzles.utilities.math.isPrime

// not refactored! ugly!
class Y17D23(input: String) : Day {

    val commands = input
        .split(' ', '\n')
        .chunked(3) { (name, arg1, arg2) -> Command(name, arg1, arg2) }

    private var registers = mutableMapOf<String, Long>()

    private fun valueOf(arg: String) = arg
        .toLongOrNull()
        ?: registers[arg]
        ?: 0L

    override fun part1(): Int {
        var index = 0
        var p1 = 0
        while (index in commands.indices) {
            val command = commands[index]
            when (command.name) {
                "set" -> {
                    registers[command.arg1] = valueOf(command.arg2)
                }

                "sub" -> {
                    registers[command.arg1] = valueOf(command.arg1) - valueOf(command.arg2)
                }
                "mul" -> {
                    p1++
                    registers[command.arg1] = valueOf(command.arg1) * valueOf(command.arg2)
                }
                "jnz" -> {
                    if (valueOf(command.arg1) != 0L) {
                        index += valueOf(command.arg2).toInt()
                        continue
                    }
                }
            }
            index++
        }
        return p1
    }

    override fun part2(): Int {
        val b = commands.first().arg2.toInt() * 100 + 100_000
        return (b..b + 17_000 step 17).count {
            !it.isPrime()
        }
    }
}

fun main() = Day.runDay(Y17D23::class)

//    Class creation: 3ms
//    Part 1: 3025 (9ms)
//    Part 2: 915 (4ms)
//    Total time: 17ms
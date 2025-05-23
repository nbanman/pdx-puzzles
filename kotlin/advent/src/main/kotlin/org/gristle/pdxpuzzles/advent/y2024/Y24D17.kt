package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.pow
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y24D17(input: String) : Day {
    var register = LongArray(3)

    val program: List<Int>

    init {
        val iter = input.getInts().iterator()
        for (i in register.indices) {
            register[i] = iter.next().toLong()
        }
        program = iter.asSequence().toList()
    }

    private fun comboValue(operand: Int): Long = when (operand) {
        in 0..3 -> operand.toLong()
        in 4..6 -> register[operand - 4]
        else -> -1
    }

    fun solve(a: Long): List<Int> {
        register[0] = a
        var cursor = 0
        val out = mutableListOf<Int>()
        while (cursor < program.size) {
            val opcode = program[cursor++]
            val operand = program[cursor++]

            when (opcode) {
                0 -> register[0] /= 2L.pow(comboValue(operand))     // adv
                1 -> register[1] = register[1] xor operand.toLong() // bxl
                2 -> register[1] = comboValue(operand) % 8          // bst
                3 -> if (register[0] != 0L) cursor = operand        // jnz
                4 -> register[1] = register[1] xor register[2]      // bxc
                5 -> out.add((comboValue(operand) % 8).toInt())     // out
                6 -> register[1] = register[0] / 2L.pow(comboValue(operand))     // bdv
                7 -> register[2] = register[0] / 2L.pow(comboValue(operand))     // cdv
            }
        }
        return out
    }

    override fun part1() = solve(register[0]).joinToString(",")
    override fun part2(): Long {
        var counter = 1L
        while (true) {
            val ans = solve(counter)
            val matching = ans == program.takeLast(ans.size)
            if (matching) {
                if (ans.size == program.size) break
                counter *= 8
            } else {
                counter++
            }
        }
        return counter
    }
}

fun main() = Day.runDay(Y24D17::class)

//    Class creation: 1ms
//    Part 1: 5,1,3,4,3,7,2,1,7 (2ms)
//    Part 2: 216584205979245 (5ms)
//    Total time: 9ms

@Suppress("unused")
private val test = listOf("""Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
""", """Register A: 0
Register B: 0
Register C: 9

Program: 2,6""", """Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4""", """Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0""", """Register A: 0
Register B: 29
Register C: 0

Program: 1,7""", """Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0""", """Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0""")
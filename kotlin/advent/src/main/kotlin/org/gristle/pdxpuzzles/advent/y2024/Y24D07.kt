package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getLongList

class Y24D07(input: String) : Day {
    enum class Operation {
        Add { override fun operate(a: Long, b: Long) = a + b },
        Multiply { override fun operate(a: Long, b: Long) = a * b },
        Concatenate { override fun operate(a: Long, b: Long): Long = (a.toString() + b.toString()).toLong() };

        abstract fun operate(a: Long, b: Long): Long
    }

    data class Equation(val test: Long, val values: List<Long>) {
        fun isValid(operations: List<Operation>): Boolean {
            fun dfs(current: Long, index: Int): Boolean = when {
                current > test -> false
                index > values.lastIndex -> current == test
                else -> {
                    operations.any { operation ->
                        dfs(operation.operate(current, values[index]), index + 1)
                    }
                }
            }
            return dfs(values[0], 1)
        }

        companion object {
            fun new(s: String): Equation {
                val (test, values) = s.split(':').map { it.getLongList() }
                return Equation(test[0], values)
            }
        }
    }

    private val equations = input.lines().map(Equation::new)

    override fun part1(): Long = equations
        .filter { it.isValid(listOf(Operation.Multiply, Operation.Add)) }
        .sumOf { it.test }

    override fun part2(): Long = equations
        .filter { it.isValid(listOf(Operation.Multiply, Operation.Concatenate, Operation.Add)) }
        .sumOf { it.test }
}

fun main() = Day.runDay(Y24D07::class)

//    Class creation: 27ms
//    Part 1: 945512582195 (15ms)
//    Part 2: 271691107779347 (232ms)
//    Total time: 275ms

@Suppress("unused")
private val test = listOf("""190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
""")
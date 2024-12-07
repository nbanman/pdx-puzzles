package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getLongList

class Y24D07(input: String) : Day {
    enum class Operation {
        Sub { override fun operate(a: Long, b: Long) = a - b },
        Divide { override fun operate(a: Long, b: Long) = if (a % b == 0L) { a / b } else null },
        Slough {
            override fun operate(a: Long, b: Long): Long? {
                val aString = a.toString()
                val bString = b.toString()
                return if (aString.takeLast(bString.length) == bString) {
                    aString.dropLast(bString.length).toLongOrNull()
                } else {
                    null
                }
            }
        };
        abstract fun operate(a: Long, b: Long): Long?
    }

    @JvmInline
    value class Equation(val values: List<Long>) {
        fun isValid(operations: List<Operation>): Boolean {
            val test = values[1]
            fun dfs(current: Long, index: Int): Boolean = when {
                current < test -> false
                index == 1 -> current == test
                else -> {
                    operations.any { operation ->
                        operation.operate(current, values[index])
                            ?.let {
                                dfs(it, index - 1)
                            }
                            ?: false
                    }
                }
            }
            return dfs(values.first(), values.lastIndex)
        }
    }

    private val equations = input.lines().map { Equation(it.getLongList()) }

    private fun solve(operations: List<Operation>): Long = equations
        .filter { it.isValid(operations) }
        .sumOf { it.values.first() }

    override fun part1(): Long = solve(listOf(Operation.Sub, Operation.Divide))
    override fun part2(): Long = solve(listOf(Operation.Divide, Operation.Slough, Operation.Sub))
}

fun main() = Day.runDay(Y24D07::class)

//    Class creation: 19ms
//    Part 1: 945512582195 (5ms)
//    Part 2: 271691107779347 (9ms)
//    Total time: 34ms

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
""",
    """161011: 16 10 13"""
)
package org.gristle.pdxpuzzles.advent.y2025

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import org.gristle.pdxpuzzles.utilities.parsing.getLongList

class Y25D06(private val input: String) : Day {
    private val operators: List<Long.(Long) -> Long> = input.lines().last().replace(" ", "").map {
        when (it) {
            '*' -> Long::times
            '+' -> Long::plus
            else -> error("invalid")
        }
    }

    private val grid = input.dropLastWhile { it != '\n' }.toGrid()

    private val ops = input.takeLastWhile { it != '\n' }
    override fun part1() = input
        .getLongList()
        .toGrid(operators.size)
        .columnsSequence()
        .zip(operators.asSequence())
        .sumOf { (n, op) -> n.reduce(op) }

    override fun part2(): Long {
        var sum = 0L
        var colVal = 0L
        var operator = ' '
        for ((col, op) in grid.columnsSequence().zip(ops.asSequence())) {
            when (op) {
                '*' -> {
                    sum += colVal
                    colVal = 1L
                    operator = '*'
                }
                '+' -> {
                    sum += colVal
                    colVal = 0L
                    operator = '+'
                }
                ' ' -> {}
                else -> error("'$op' not recognized")
            }
            val digit = col.filter { it.isDigit() }.joinToString("").toLongOrNull()

            if (digit != null) {
                if (operator == '*') {
                    colVal *= digit
                } else if (operator == '+') {
                    colVal += digit
                }
            }
        }
        return sum + colVal
    }
}

fun main() = Day.runDay(Y25D06::class)

//    Class creation: 3ms
//    Part 1: 4277556 (2ms)
//    Part 2: 3263827 (3ms)
//    Total time: 10ms
package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y17D08(input: String) : Day {

    private val pattern = """([a-z]+) (inc|dec) (-?\d+) if ([a-z]+) (<=|<|==|!=|>|>=) (-?\d+)""".toRegex()

    data class Instruction(
        val operand: String,
        val amount: Int,
        val conVar: String,
        val conOp: String,
        val conAmt: Int
    ) {

        fun execute(register: MutableMap<String, Int>): Int {
            val conVal = register.getValue(conVar)
            val meetsCondition = when (conOp) {
                "<=" -> conVal <= conAmt
                "<" -> conVal < conAmt
                "==" -> conVal == conAmt
                "!=" -> conVal != conAmt
                ">=" -> conVal >= conAmt
                else -> conVal > conAmt
            }
            if (meetsCondition) {
                register[operand] = register.getValue(operand) + amount
            }
            return register.getValue(operand)
        }
    }

    private val instructions = pattern
        .findAll(input)
        .map { matchResult ->
            val (operand, operation, amountStr, conVar, conOp, conAmtStr) = matchResult.destructured
            val amount = amountStr.toInt().let { if (operation == "dec") -it else it }
            val conAmt = conAmtStr.toInt()
            Instruction(operand, amount, conVar, conOp, conAmt)
        }.toList()

    private val solution = let {
        val register = mutableMapOf<String, Int>().withDefault { 0 }
        val highest = instructions.maxOf { it.execute(register) }
        register.values.max() to highest
    }

    override fun part1() = solution.first

    override fun part2() = solution.second
}

fun main() = Day.runDay(Y17D08::class)

//    Class creation: 35ms
//    Part 1: 6343 (0ms)
//    Part 2: 7184 (0ms)
//    Total time: 35ms

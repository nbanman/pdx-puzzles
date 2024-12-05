package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y20D18(input: String) : Day {

    sealed class SubExpression(val length: Int) {

        object Plus : SubExpression(1) {
            override fun toString() = "+"
        }

        object Times : SubExpression(1) {
            override fun toString() = "*"
        }

        class Parens(val components: List<SubExpression>, length: Int) : SubExpression(length) {
            companion object {
                fun fromString(s: String): Parens {

                    fun getComponent(s: String): SubExpression {
                        require(s.isNotBlank())
                        return when {
                            Character.isDigit(s[0]) -> Num(s.takeWhile { Character.isDigit(it) }.toLong())
                            s[0] == '*' -> Times
                            s[0] == '+' -> Plus
                            s[0] == '(' -> {
                                fromString(s.drop(1))
                            }
                            else -> Sentinel()
                        }
                    }

                    var parser = 0
                    val components = mutableListOf<SubExpression>()

                    while (parser < s.length && s[parser] != ')' ) {
                        components.add(getComponent(s.drop(parser)))
                        parser += components.last().length
                    }
                    return Parens(components, parser + 2)
                }
            }

            override fun toString() = "($components)"
        }
        class Num(val value: Long) : SubExpression(value.toString().length) {
            override fun toString() = value.toString()
        }
        class Sentinel : SubExpression(0) {
            override fun toString() = "ERROR"
        }
    }

    private val expressions = input
        .replace(" ", "")
        .lines()
        .map { SubExpression.Parens.fromString(it).components }

    private fun List<SubExpression>.p1Eval(): Long {
        val ml = toMutableList()
        while (ml.size > 1) {
            val leftValue = when (val left = ml[0]) {
                is SubExpression.Num -> left.value
                is SubExpression.Parens -> left.components.p1Eval()
                else -> throw IllegalArgumentException()
            }
            val rightValue = when (val right = ml[2]) {
                is SubExpression.Num -> right.value
                is SubExpression.Parens -> right.components.p1Eval()
                else -> throw IllegalArgumentException()
            }
            val newValue = when {
                ml[1] is SubExpression.Plus -> leftValue + rightValue
                ml[1] is SubExpression.Times -> leftValue * rightValue
                else -> throw IllegalArgumentException()
            }
            ml[1] = SubExpression.Num(newValue)
            ml.removeAt(0)
            ml.removeAt(1)
        }
        return (ml.first() as SubExpression.Num).value
    }


    private fun List<SubExpression>.p2Eval(): Long {
        val ml = toMutableList()
        while (ml.size > 1) {
            val plusIndex = ml.indexOf(SubExpression.Plus)
            if (plusIndex == -1) return ml
                .mapNotNull { subExpression ->
                    when (subExpression) {
                        is SubExpression.Num -> subExpression.value
                        is SubExpression.Parens -> subExpression.components.p2Eval()
                        else -> null
                    }
                }.reduce { acc, l -> acc * l }
            val leftValue = when (val left = ml[plusIndex - 1]) {
                is SubExpression.Num -> left.value
                is SubExpression.Parens -> left.components.p2Eval()
                else -> throw IllegalArgumentException()
            }
            val rightValue = when (val right = ml[plusIndex + 1]) {
                is SubExpression.Num -> right.value
                is SubExpression.Parens -> right.components.p2Eval()
                else -> throw IllegalArgumentException()
            }
            val sum = leftValue + rightValue
            ml[plusIndex] = SubExpression.Num(sum)
            ml.removeAt(plusIndex + 1)
            ml.removeAt(plusIndex - 1)
        }
        return (ml.first() as SubExpression.Num).value
    }

    override fun part1() = expressions.sumOf { it.p1Eval() }

    override fun part2() = expressions.sumOf { it.p2Eval() }
}

fun main() = Day.runDay(Y20D18::class)

//    Class creation: 35ms
//    Part 1: 510009915468 (3ms)
//    Part 2: 321176691637769 (3ms)
//    Total time: 42ms
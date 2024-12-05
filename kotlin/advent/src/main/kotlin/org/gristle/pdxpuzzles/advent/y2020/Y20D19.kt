package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit
import org.gristle.pdxpuzzles.utilities.parsing.getIntList

class Y20D19(input: String) : Day {

    sealed class Rule(val name: Int) {

        abstract fun expand(register: Map<Int, Rule>): String

        class Value(name: Int, val value: Char) : Rule(name) {
            override fun expand(register: Map<Int, Rule>) = "$value"

            override fun toString() = "$value: $name"
        }

        class Seq(name: Int, private val subRules: List<Int>) : Rule(name) {
            override fun expand(register: Map<Int, Rule>): String {
                return subRules.joinToString("") { register.getValue(it).expand(register) }
            }

            override fun toString() = "Seq: $name, $subRules"
        }

        class Fork(name: Int, val left: List<Int>, val right: List<Int>) : Rule(name) {
            private var counter = 0
            override fun expand(register: Map<Int, Rule>): String {
                if (name == 8 || name == 11) counter++

                return if (counter < 5) {
                    buildString {
                        append('(')
                        append(left.joinToString("") {
                            register[it]!!.expand(register)
                        })
                        append('|')
                        append(right.joinToString("") {
                            register[it]!!.expand(register)
                        })
                        append(')')
                    }
                } else {
                    left.joinToString("") { register.getValue(it).expand(register) }
                }
            }

            override fun toString() = "Fork: $name, $left | $right"
        }
    }

    private val rules: List<Rule>
    private val messages: List<String>

    init {
        val (ruleLines, messageLines) = input.blankSplit().map { it.lines() }
        rules = ruleLines
            .map { ruleString ->
                val ints = ruleString.getIntList()
                when (ints.size) {
                    1 -> Rule.Value(ints[0], ruleString[ruleString.lastIndex - 1])
                    2 -> Rule.Seq(ints[0], ints.drop(1))
                    3 -> {
                        if ('|' in ruleString) {
                            Rule.Fork(ints[0], ints.slice(1..1), ints.slice(2..2))
                        } else {
                            Rule.Seq(ints[0], ints.drop(1))
                        }
                    }

                    else -> Rule.Fork(ints[0], ints.slice(1..2), ints.slice(3..4))
                }
            }
        messages = messageLines
    }

    fun solve(rules: List<Rule>): Int {
        val register = rules.associateBy { it.name }
        val matchPattern = register.getValue(0).expand(register).toRegex()

        return messages.count { matchPattern.matches(it) }
    }

    override fun part1() = solve(rules)

    override fun part2(): Int {
        val newRules = rules
            .sortedBy { it.name }
            .mapIndexed { index, rule ->
                when (index) {
                    8 -> Rule.Fork(8, listOf(42), listOf(42, 8))
                    11 -> Rule.Fork(11, listOf(42, 31), listOf(42, 11, 31))
                    else -> rule
                }
            }
        return solve(newRules)
    }
}

fun main() = Day.runDay(Y20D19::class)

//    Class creation: 32ms
//    Part 1: 151 (16ms)
//    Part 2: 386 (35ms)
//    Total time: 84ms
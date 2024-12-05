package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.groupValues
import org.gristle.pdxpuzzles.utilities.parsing.gvs
import java.util.*

class Y20D07(input: String) : Day {

    class HeldBag(val color: String, private val amount: Int) {
        fun bagsInside(bagMap: Map<String, Rule>): Int {
            return amount + amount * bagMap.getValue(color).heldBags.sumOf { it.bagsInside(bagMap) }
        }
    }

    class Rule(val color: String, val heldBags: List<HeldBag>) {

        fun contains(other: String, bagMap: Map<String, Rule>): Boolean {
            val visited = mutableSetOf(color)
            val q: Deque<HeldBag> = ArrayDeque()
            q.addAll(heldBags)
            while (q.isNotEmpty()) {
                val current = q.poll().color
                if (current == other) return true
                visited.add(current)
                q.addAll(bagMap.getValue(current).heldBags.filter { !visited.contains(it.color) })
            }
            return false
        }

        fun bagsInside(bagMap: Map<String, Rule>): Int {
            return HeldBag(color, 1).bagsInside(bagMap) - 1
        }
    }

    private val bagRx = """(\d+) (\w+ \w+) bag""".toRegex()

    private val rules: List<Rule> = input
        .gvs("""(\w+ \w+) bags contain ([^.]+)\.""")
        .map { (container, contained) ->
            val heldBags = contained
                .groupValues(bagRx)
                .map { (amountString, bag) -> HeldBag(bag, amountString.toInt()) }
            Rule(container, heldBags)
        }.toList()

    private val bagMap: Map<String, Rule> by lazy {
        buildMap { rules.forEach { rule -> put(rule.color, rule) } }
    }

    override fun part1() = rules.count { it.contains("shiny gold", bagMap) }

    override fun part2() = bagMap.getValue("shiny gold").bagsInside(bagMap)
}

fun main() = Day.runDay(Y20D07::class)

//    Class creation: 56ms
//    Part 1: 252 (17ms)
//    Part 2: 35487 (0ms)
//    Total time: 74ms
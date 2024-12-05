package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.IndexedLinkedList
import org.gristle.pdxpuzzles.utilities.objects.shift

class Y20D23(private val input: String) : Day {

    fun IndexedLinkedList<Int>.move(moves: Int, largest: Int) {
        var current = header
        (1..moves).forEach { _ ->
            val cut = current.cut(3)
            val cutValues = listOf(cut.value, cut.next!!.value, cut.next!!.next!!.value)
            val pastePos = let {
                var nextValue = (current.value - 1).let { if (it > 0) it else largest }
                while (nextValue in cutValues) {
                    nextValue = (nextValue - 1).let { if (it > 0) it else largest }
                }
                index.getValue(nextValue)
            }
            pastePos.add(cut)
            current = current.next!!
        }
    }

    override fun part1(): Int {
        val moves = 100
        val elements = input.map { Character.getNumericValue(it) }
        val circle = IndexedLinkedList(elements, true)
        circle.move(moves, elements.maxOrNull()!!)
        return circle
            .toList()
            .let {
                it.shift(it.indexOf(1))
            }.drop(1)
            .joinToString("")
            .toInt()
    }

    override fun part2(): Long {
        val moves = 10_000_000
        val size = 1_000_000
        val elements = List(size) { i ->
            if (i in input.indices) Character.getNumericValue(input[i]) else i + 1
        }
        val circle = IndexedLinkedList(elements, true)
        circle.move(moves, size)
        val circleList = circle.toList()
        val indexOf1 = circleList.indexOf(1)
        return circleList[indexOf1 + 1].toLong() * circleList[indexOf1 + 2]
    }
}

fun main() = Day.runDay(Y20D23::class)

//    Class creation: 16ms
//    Part 1: 94238657 (1ms)
//    Part 2: 3072905352 (2999ms)
//    Total time: 3018ms
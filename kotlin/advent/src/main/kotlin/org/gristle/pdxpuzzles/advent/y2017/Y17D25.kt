package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y17D25(private val input: String) : Day {

    enum class Dir {
        LEFT,
        RIGHT,
    }
    data class Action(val write: Boolean, val dir: Dir, val change: Int)
    data class State(val zero: Action, val one: Action)

    override fun part1(): Int {
        val stanzas = input.splitToSequence("\n\n").iterator()
        val steps = stanzas.next()
            .dropLastWhile { c -> !c.isDigit() }
            .takeLastWhile { c -> c.isDigit() }
            .toInt()
        val states = stanzas.asSequence()
            .map { stanza ->
                val args = stanza.lines().map { line ->
                    line.dropLast(1).takeLastWhile { c -> c.isLetterOrDigit() }
                }
                val write0 = args[2] == "1"
                val dir0 = if (args[3] == "left") Dir.LEFT else Dir.RIGHT
                val change0 = args[4][0] - 'A'
                val write1 = args[6] == "1"
                val dir1 = if (args[7] == "left") Dir.LEFT else Dir.RIGHT
                val change1 = args[8][0] - 'A'
                State(
                    Action(write0, dir0, change0),
                    Action(write1, dir1, change1),
                )
            }.toList()
        val slots = ArrayDeque<Boolean>()
        slots.add(false)
        var state = states[0]
        var node = 0

        for (_i in 0 until steps) {
            val action = if (slots[node]) state.one else state.zero
            slots[node] = action.write
            when (action.dir) {
                Dir.LEFT -> if (node == 0) slots.addFirst(false) else node--
                Dir.RIGHT -> {
                    if (node == slots.size - 1) slots.addLast(false)
                    node++
                }
            }
            state = states[action.change]
        }
        return slots.count { it }
    }

    override fun part2() = true
}

fun main() = Day.runDay(Y17D25::class)

//    Class creation: 1ms
//    Part 1: 3745 (76ms)
//    Total time: 77ms
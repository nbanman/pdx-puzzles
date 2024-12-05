package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.y2019.Intcode.IntCode
import java.util.*

class Y19D23(private val input: String) : Day {

    data class Nat(val x: Long, val y: Long)

    private fun solve(): Pair<Long, Long> {
        val initialState = input.split(',').map { it.toLong() }
        val inputs = List(50) { LinkedList<Long>() }
        val outputs = List(50) { LinkedList<Long>() }
        val nics = List(50) { i ->
            IntCode("$i", initialState, null, inputs[i], outputs[i]).apply {
                inputs[i].add(i.toLong())
                run()
            }
        }
        val nat = mutableListOf<Nat>()
        var lastYSent = -1L
        while (true) {
            inputs.forEach { if (it.isEmpty()) it.add(-1L) }
            nics.forEach { nic ->
                val id = nic.name.toInt()
                nic.run()
                for ((recipient, x, y) in outputs[id].chunked(3)) {
                    val recId = recipient.toInt()
                    if (recId == 255) {
                        nat.add(Nat(x, y))
                    } else {
                        inputs[recId].add(x)
                        inputs[recId].add(y)
                    }
                }
                outputs[id].clear()
            }
            if (inputs.all { it.isEmpty() }) {
                if (nat.last().y == lastYSent) break
                inputs[0].add(nat.last().x)
                inputs[0].add(nat.last().y)
                lastYSent = nat.last().y
            }
        }
        return nat.first().y to lastYSent
    }

    private val solution = solve()

    override fun part1() = solution.first

    override fun part2() = solution.second
}

fun main() = Day.runDay(Y19D23::class)

//    Class creation: 99ms
//    Part 1: 23701 (0ms)
//    Part 2: 17225 (0ms)
//    Total time: 99ms
package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import java.util.*

class Y19D19(input: String) : Day {

    private val initialState = input.split(',').map { it.toLong() }
    private val toDroid: Deque<Long> = LinkedList()
    private val toComp: Deque<Long> = LinkedList()
    private val intCode = IntCode("A", initialState, null, toComp, toDroid)

    override fun part1(): Int {
        for (y in 0L..49L) for (x in 0L..49L) {
            toComp.add(x)
            toComp.add(y)
            intCode.reset()
            intCode.run()
        }
        return toDroid.count { it == 1L }
    }

    override fun part2(): Long {
        // Part 2
        toDroid.clear()
        var rightEdge = 0L
        var leftEdge = 0L
        var y = 0L
        val width = 100
        while (leftEdge + width - 1 != rightEdge) {
            y++
            var rightAdd = 2
            while (true) {
                toComp.add(rightEdge + rightAdd)
                toComp.add(y)
                intCode.reset()
                intCode.run()
                if (toDroid.poll() == 0L) {
                    rightEdge += rightAdd - 1
                    break
                }
                rightAdd++
            }
            var leftAdd = 1
            while (true) {
                toComp.add(leftEdge + leftAdd)
                toComp.add(y + width - 1)
                intCode.reset()
                intCode.run()
                if (toDroid.poll() == 1L) {
                    leftEdge += leftAdd
                    break
                }
                leftAdd++
            }
        }
        return leftEdge * 10_000 + y
    }
}

fun main() = Day.runDay(Y19D19::class)

//    Class creation: 18ms
//    Part 1: 179 (237ms)
//    Part 2: 9760485 (98ms)
//    Total time: 354ms

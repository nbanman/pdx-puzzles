package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.adventOfCode.y2016.shared.Assembunny

class Y16D25(private val input: String) : Day {

    private fun Int.isClockSignal() = generateSequence(this) { it shr 1 }
        .takeWhile { it != 0 }
        .withIndex()
        .all { (idx, value) -> idx and 1 == value and 1 }

    // Reverse engineering the code ends with infinite loop that prints reverse binary representation of d over and
    // over again, and defines d = a + 2555. So a = d - 2555. Starting from 2555, the first number that is
    // 10101010...10 is 2730. So that's d. A is 2555 less than that, or 175.    
    override fun part1(): Int {
        
        val instructions = Assembunny.parseInstructions(input)
        
        val offset = Assembunny().runAsSequence(instructions)
            .take(10_000)
            .last()[3]
        
        return generateSequence(1) { it + 1 }
            .first { (it + offset).isClockSignal() }
    } 

    override fun part2() = null
}

fun main() = Day.runDay(Y16D25::class)

//    Class creation: 12ms
//    Part 1: 175 (1ms)
//    Total time: 14ms
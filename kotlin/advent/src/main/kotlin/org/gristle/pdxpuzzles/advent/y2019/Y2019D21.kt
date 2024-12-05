package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.toMutableGrid
import java.util.*

class Y19D21(private val input: String) : Day {

    fun execute(commands: String): Long {
        val initialState = input.split(',').map { it.toLong() }
        val toDroid: Deque<Long> = LinkedList()
        val toComp: Deque<Long> = LinkedList()
        val intCode = IntCode("A", initialState, null, toComp, toDroid)
        intCode.run()
        commands.split("\n").forEach {
            addCommand(it, toComp)
            intCode.run()
        }
        return toDroid.last()
    }


    private fun addCommand(command: String, toComp: Deque<Long>) {
        val commands = command.map { it.code.toLong() }
        toComp.addAll(commands)
        toComp.add('\n'.code.toLong())
    }

    @Suppress("unused")
    private fun printOutput(toDroid: Deque<Long>) {
        val width = toDroid.indexOfFirst { it == 10L }
        val grid = toDroid.mapNotNull { if (it != null && it != 10L) it else null }.toMutableGrid(width)
        println(grid.representation { it.toInt().toChar() })
    }

    override fun part1(): Long {
        val commands = """NOT A T
NOT C J
OR T J
AND D J
WALK"""
        return execute(commands)
    }

    override fun part2(): Long {
        val commands = """OR A J
AND B J  
AND C J  
NOT J J  
AND D J  
OR E T  
OR H T  
AND T J  
RUN"""
        return execute(commands)
    }
}

fun main() = Day.runDay(Y19D21::class)

//    Class creation: 19ms
//    Part 1: 19349530 (18ms)
//    Part 2: 1142805439 (99ms)
//    Total time: 137ms
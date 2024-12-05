package org.gristle.pdxpuzzles.advent.y2019.Intcode

import org.gristle.pdxpuzzles.utilities.debugging.print
import org.gristle.pdxpuzzles.utilities.iteration.nextOrNull
import org.gristle.pdxpuzzles.utilities.parsing.getLongList

data class ICS(val program: List<Long>, val parser: Int, val relativeBase: Int)

interface IntCodeMachine {
    fun reset(): FintCode
    fun restore(save: ICS): FintCode
    fun restore(id: String): FintCode
    fun run(maxSteps: Int = -1): List<Long>
    fun run(command: String, maxSteps: Int = -1): List<Long>
    fun run(command: Iterable<Long>, maxSteps: Int = -1): List<Long>
    fun save(id: String? = null): ICS
}

class FintCode(
    initialState: Iterable<Long>,
    private val memory: Int = 100_000,
    private val verbose: Boolean = false
) : IntCodeMachine {

    constructor(initialState: String, memory: Int = 100_000, verbose: Boolean = false) : this(
        initialState.getLongList(),
        memory,
        verbose
    )

    // STATE VARIABLES
    private val initialState: List<Long> = initialState.toList()

    var program: LongArray = initialState.toProgram()

    private var cursor = 0

    private var relativeBase = 0

    private val saveStates = mutableMapOf<String, ICS>()

    // API FUNCTIONS
    override fun reset(): FintCode {
        program = initialState.toProgram()
        cursor = 0
        relativeBase = 0
        return this
    }

    override fun restore(save: ICS): FintCode {
        program = save.program.toProgram()
        cursor = save.parser
        relativeBase = save.relativeBase
        return this
    }

    override fun restore(id: String): FintCode {
        val saveState = saveStates[id] ?: throw IllegalArgumentException("no save associated with id $id.")
        restore(saveState)
        return this
    }

    override fun save(id: String?): ICS {
        val saveState = ICS(program.toList(), cursor, relativeBase)
        id?.let { saveStates[id] = saveState }
        return saveState
    }

    override fun run(command: Iterable<Long>, maxSteps: Int): List<Long> {
        var count = 0
        val output = mutableListOf<Long>()
        val input = command.iterator()

        while (maxSteps < 0 || count++ < maxSteps) {
            val instruction = program[cursor].toInt()
            when (instruction % 100) {
                99 -> return output

                1 -> {
                    verbose.print(getVerbose(instruction, 3, "add"))
                    program[getWrite(3)] = getVal(1) + getVal(2)
                    cursor += 4
                }

                2 -> {
                    verbose.print(getVerbose(instruction, 3, "multiply"))
                    program[getWrite(3)] = getVal(1) * getVal(2)
                    cursor += 4
                }

                3 -> {
                    verbose.print(getVerbose(instruction, 1, "input"))
                    program[getWrite(1)] = input.nextOrNull()
                        ?: throw IllegalArgumentException("No input to read.")
                    cursor += 2
                }

                4 -> {
                    verbose.print(getVerbose(instruction, 1, "output"))
                    output.add(getVal(1))
                    cursor += 2
                }

                5 -> {
                    verbose.print(getVerbose(instruction, 2, "Jump if !0"))
                    cursor = if (getVal(1) != 0L) {
                        getVal(2).toInt()
                    } else {
                        cursor + 3
                    }
                }

                6 -> {
                    verbose.print(getVerbose(instruction, 2, "Jump if 0"))
                    cursor = if (getVal(1) == 0L) {
                        getVal(2).toInt()
                    } else {
                        cursor + 3
                    }
                }

                7 -> {
                    verbose.print(getVerbose(instruction, 3, "a < b ? 1 : 0"))
                    program[getWrite(3)] =
                        if (getVal(1) < getVal(2)) 1 else 0
                    cursor += 4
                }

                8 -> {
                    verbose.print(getVerbose(instruction, 3, "a == b ? 1 : 0"))
                    program[getWrite(3)] =
                        if (getVal(1) == getVal(2)) 1 else 0
                    cursor += 4
                }

                9 -> {
                    verbose.print(getVerbose(instruction, 1, "base change"))
                    relativeBase += getVal(1).toInt()
                    cursor += 2
                }
            }
        }
        return output
    }

    override fun run(command: String, maxSteps: Int): List<Long> {
        val longCommand = command.map { it.code.toLong() }
        return run(longCommand, maxSteps)
    }

    override fun run(maxSteps: Int): List<Long> = run(emptyList(), maxSteps)

    // UTILITY FUNCTIONS

    private fun <T> Iterable<T>.toProgram(): LongArray = LongArray(memory)
        .also { program -> initialState.forEachIndexed { i, l -> program[i] = l } }

    private fun getMode(parameter: Int): Int {
        val instruction = program[cursor].toInt()
        return when (parameter) {
            1 -> instruction % 1_000 / 100
            2 -> instruction % 10_000 / 1000
            else -> instruction / 10_000
        }
    }

    private fun getVerbose(instruction: Int, parameters: Int, name: String) = buildString {
        append("${instruction % 100}, $instruction, $name, ")
        (1..parameters).forEach { append("${program[cursor + it]}, ") }
        append('\n')
    }

    private fun getVal(parameter: Int): Long = when (getMode(parameter)) {
        0 -> program[program[cursor + parameter].toInt()]
        1 -> program[cursor + parameter]
        else -> program[(relativeBase + program[cursor + parameter]).toInt()]
    }

    private fun getWrite(parameter: Int): Int {
        val offset = if (getMode(parameter) == 2) relativeBase else 0
        val value = program[cursor + parameter].toInt()
        return offset + value
    }
}

/**
 * Converts output to human-readable format.
 */
fun List<Long>.toOutputString(): String = map { it.toInt().toChar() }.joinToString("")

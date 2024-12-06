package org.gristle.pdxpuzzles.advent.y2019.Intcode

import java.util.*

class IntCode(
    val name: String,
    private val initialState: Iterable<Long>,
    private val initialInput: Long? = null,
    var input: Deque<Long>? = null,
    val output: Deque<Long> = LinkedList(),
    private val verbose: Boolean = false
) {
    var program = mutableMapOf(*initialState.mapIndexed { index, l -> index to l }.toTypedArray())
    private var parser = 0
    var isDone = false
    private var initialInputSupplied = initialInput == null
    private var startWithZero: Boolean = input == null
    private var relativeBase = 0L

    fun reset() {
        program = mutableMapOf(*initialState.mapIndexed { index, l -> index to l }.toTypedArray())
        parser = 0
        isDone = false
        initialInputSupplied = initialInput == null
        startWithZero = input == null
        relativeBase = 0L
    }

    fun save() = ICSave(program.toMap(), parser, isDone, initialInputSupplied, startWithZero, relativeBase)

    fun restore(save: ICSave) {
        program = save.program.toMutableMap()
        parser = save.parser
        isDone = save.isDone
        initialInputSupplied = save.initialInputSupplied
        startWithZero = save.startWithZero
        relativeBase = save.relativeBase
    }

    fun run(maxCount: Int = -1): Boolean {
        var count = 0
        if (isDone) return true
        while (maxCount < 0 || count++ < maxCount) {
            val instruction = program.getValue(parser).toInt()
            val opCode = instruction % 100
            when (opCode) {
                99 -> {
                    if (verbose) println("$opCode: Exiting")
                    isDone = true
                    break
                }

                1 -> {
                    if (verbose) getVerbose(instruction, 3, "add")
                    program[getWrite(3)] = getVal(1) + getVal(2)
                    parser += 4
                }

                2 -> {
                    if (verbose) getVerbose(instruction, 3, "multiply")
                    program[getWrite(3)] = getVal(1) * getVal(2)
                    parser += 4
                }

                3 -> {
                    if (verbose) getVerbose(instruction, 1, "input")
                    when {
                        !initialInputSupplied -> {
                            program[getWrite(1)] = initialInput ?: throw Exception("no initial input given")
                            initialInputSupplied = true
                        }

                        input == null -> {
                            program[getWrite(1)] = 0
                        }

                        else -> {
                            program[getWrite(1)] = if (startWithZero) {
                                startWithZero = false
                                0
                            } else {
                                input?.poll() ?: return true
                            }
                        }
                    }
                    parser += 2
                }

                4 -> {
                    if (verbose) getVerbose(instruction, 1, "output")
                    output.add(getVal(1))
                    parser += 2
                }

                5 -> {
                    if (verbose) getVerbose(instruction, 2, "Jump if !0")
                    parser = if (getVal(1) != 0L) {
                        getVal(2).toInt()
                    } else {
                        parser + 3
                    }
                }

                6 -> {
                    if (verbose) getVerbose(instruction, 2, "Jump if 0")
                    parser = if (getVal(1) == 0L) {
                        getVal(2).toInt()
                    } else {
                        parser + 3
                    }
                }

                7 -> {
                    if (verbose) getVerbose(instruction, 3, "a < b ? 1 : 0")
                    program[getWrite(3)] =
                        if (getVal(1) < getVal(2)) 1 else 0
                    parser += 4
                }

                8 -> {
                    if (verbose) getVerbose(instruction, 3, "a == b ? 1 : 0")
                    program[getWrite(3)] =
                        if (getVal(1) == getVal(2)) 1 else 0
                    parser += 4
                }

                9 -> {
                    if (verbose) getVerbose(instruction, 1, "base change")
                    relativeBase += getVal(1)
                    parser += 2
                }
            }
        }
        return false
    }

    private fun getVerbose(instruction: Int, parameters: Int, name: String) {
        print("${instruction % 100}, $instruction, $name, ")
        (1..parameters).forEach { print("${program[parser + it]}, ") }
        println()
    }

    private fun getVal(parameter: Int): Long {
        val instruction = program.getValue(parser).toInt()
        val mode = when (parameter) {
            1 -> instruction % 1_000 / 100
            2 -> instruction % 10_000 / 1000
            else -> instruction / 10_000
        }
        return when (mode) {
            0 -> program[program[parser + parameter]?.toInt()] ?: 0L
            1 -> program[parser + parameter] ?: 0L
            else -> program[(relativeBase + program.getValue(parser + parameter)).toInt()] ?: 0L
        }
    }

    private fun getWrite(parameter: Int): Int {
        val instruction = program.getValue(parser).toInt()
        val mode = when (parameter) {
            1 -> instruction % 1_000 / 100
            2 -> instruction % 10_000 / 1000
            else -> instruction / 10_000
        }
        return when (mode) {
            2 -> (relativeBase + program.getValue(parser + parameter)).toInt()
            else -> program.getValue(parser + parameter).toInt()
        }
    }

    override fun toString(): String {
        return "IntCode($name)"
    }
}

data class ICSave(
    val program: Map<Int, Long>,
    val parser: Int,
    val isDone: Boolean,
    val initialInputSupplied: Boolean,
    val startWithZero: Boolean,
    val relativeBase: Long
)

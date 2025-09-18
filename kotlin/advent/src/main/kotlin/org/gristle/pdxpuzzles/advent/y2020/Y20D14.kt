package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y20D14(input: String) : Day {

    sealed class Instruction {

        companion object {
            private val pattern = Regex("""(mask|mem)(?:\[(\d+)])? = ([X\d]+)""")
            fun fromString(s: String): Instruction {
                val (_, command, register, value) = pattern
                    .find(s)
                    ?.groupValues
                    ?: throw Exception("regex pattern not found in string")
                return when (command) {
                    "mask" -> {
                        val makeMask = { predicate: (Char) -> Boolean ->
                            value.foldRightIndexed(0L) { index, c, acc ->
                                acc + if (predicate(c)) {
                                    1L.shl(value.length - index - 1)
                                } else 0L
                            }
                        }
                        val oneMask = makeMask { it == '1' }
                        val zeroMask = makeMask { it != '0' }
                        val xMask = makeMask { it == 'X' }
                        Mask(oneMask, zeroMask, xMask)
                    }
                    else -> {
                        Mem(register.toLong(), value.toLong())
                    }
                }
            }
        }

        data class Mask internal constructor(val oneMask: Long, val zeroMask: Long, val xMask: Long) : Instruction()

        data class Mem(val register: Long, val value: Long) : Instruction() {
            fun maskedRegisters(oneMask: Long, xMask: Long): List<Long> {
                val oneApplied = register.or(oneMask)
                return (0..35).fold(listOf(0L)) { acc, place ->
                    when {
                        1L.and(xMask.shr(place)) == 1L -> {
                            acc.flatMap { listOf(it, it + 1L.shl(place)) }
                        }
                        1L.and(oneApplied.shr(place)) == 1L -> {
                            acc.map { it + 1L.shl(place) }
                        }
                        else -> {
                            acc
                        }
                    }
                }
            }

            fun maskedValue(oneMask: Long, zeroMask: Long): Long {
                return value.or(oneMask).and(zeroMask)
            }
        }
    }

    private val instructions = input
        .lines()
        .map(Instruction::fromString)

    override fun part1(): Long {
        val registers = mutableMapOf<Long, Long>()
        var oneMask = 0L
        var zeroMask = 0L
        instructions.forEach { instruction ->
            when (instruction) {
                is Instruction.Mask -> {
                    oneMask = instruction.oneMask
                    zeroMask = instruction.zeroMask
                }

                is Instruction.Mem -> {
                    registers[instruction.register] = instruction.maskedValue(oneMask, zeroMask)
                }
            }
        }
        return registers.values.sum()
    }

    override fun part2(): Long {
        val registers = mutableMapOf<Long, Long>()
        var oneMask = 0L
        var xMask = 0L
        instructions.forEach { instruction ->
            when (instruction) {
                is Instruction.Mask -> {
                    oneMask = instruction.oneMask
                    xMask = instruction.xMask
                }

                is Instruction.Mem -> {
                    instruction.maskedRegisters(oneMask, xMask).forEach {
                        registers[it] = instruction.value
                    }
                }
            }
        }
        return registers.values.sum()
    }
}

fun main() = Day.benchmarkDay(Y20D14::class)

//    Class creation: 34ms
//    Part 1: 11926135976176 (0ms)
//    Part 2: 4330547254348 (83ms)
//    Total time: 119ms
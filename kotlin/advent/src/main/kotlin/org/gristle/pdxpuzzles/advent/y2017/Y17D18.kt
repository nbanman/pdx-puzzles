package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import java.util.*

class Y17D18(input: String) : Day {

    enum class CommandType { SND, SET, ADD, MUL, MOD, RCV, JGZ }

    data class Command(val type: CommandType, val arg1: String, val arg2: String)

    data class Program(
        val commands: List<Command>,
        val p: Long,
        val ownDeque: Deque<Long>,
        val otherDeque: Deque<Long>
    ) {
        var index = 0
        var deadlock = false
        var sends = 0
        private val reg = mutableMapOf("p" to p)

        private fun valueOf(arg: String) = arg
            .toLongOrNull()
            ?: reg[arg]
            ?: 0

        fun execute() {
            if (index !in commands.indices) {
                deadlock = true
                return
            }
            val command = commands[index]
            if (deadlock) {
                rcv(command)
            } else {
                when (command.type) {
                    CommandType.SND -> {
                        otherDeque.add(valueOf(command.arg1))
                        sends++
                    }

                    CommandType.SET -> {
                        reg[command.arg1] = valueOf(command.arg2)
                    }

                    CommandType.ADD -> {
                        reg[command.arg1] = valueOf(command.arg1) + valueOf(command.arg2)
                    }

                    CommandType.MUL -> {
                        reg[command.arg1] = valueOf(command.arg1) * valueOf(command.arg2)
                    }

                    CommandType.MOD -> {
                        reg[command.arg1] = valueOf(command.arg1) % valueOf(command.arg2)
                    }

                    CommandType.RCV -> {
                        rcv(command)
                        return
                    }

                    CommandType.JGZ -> {
                        if (valueOf(command.arg1) > 0) {
                            index += this.valueOf(command.arg2).toInt()
                            return
                        }
                    }
                }
                index++
            }
        }

        private fun rcv(command: Command) {
            if (ownDeque.isNotEmpty()) {
                reg[command.arg1] = ownDeque.pop()
                index++
                deadlock = false
            } else {
                deadlock = true
            }
        }
    }

    private val commands = input
        .lineSequence()
        .map {
            val args = it.split(' ')
            val type = when (args[0]) {
                "snd" -> CommandType.SND
                "set" -> CommandType.SET
                "add" -> CommandType.ADD
                "mul" -> CommandType.MUL
                "mod" -> CommandType.MOD
                "rcv" -> CommandType.RCV
                "jgz" -> CommandType.JGZ
                else -> throw IllegalArgumentException("Unrecognized command: ${args[0]}")
            }
            val arg2 = if (args.size == 3) args[2] else ""
            Command(type, args[1], arg2)
        }.toList()

    override fun part1(): Long {
        val registers = mutableMapOf<String, Long>()
        var frequency = 0L

        fun valueOf(arg: String) = arg
            .toLongOrNull()
            ?: registers[arg]
            ?: 0

        var i = 0
        while (true) {
            val command = commands[i]
            when (command.type) {
                CommandType.SND -> frequency = valueOf(command.arg1)
                CommandType.SET -> registers[command.arg1] = valueOf(command.arg2)
                CommandType.ADD -> {
                    registers[command.arg1] = valueOf(command.arg1) + valueOf(command.arg2)
                }

                CommandType.MUL -> {
                    registers[command.arg1] = valueOf(command.arg1) * valueOf(command.arg2)
                }

                CommandType.MOD -> {
                    registers[command.arg1] = valueOf(command.arg1) % valueOf(command.arg2)
                }

                CommandType.RCV -> {
                    if (valueOf(command.arg1) != 0L) {
                        return frequency
                    }

                }

                CommandType.JGZ -> {
                    if (valueOf(command.arg1) > 0) {
                        i += valueOf(command.arg2).toInt()
                        continue
                    }
                }
            }
            i++
        }
    }

    override fun part2(): Int {
        val dequeA: Deque<Long> = ArrayDeque()
        val dequeB: Deque<Long> = ArrayDeque()

        val programA = Program(commands, 0L, dequeA, dequeB)
        val programB = Program(commands, 1L, dequeB, dequeA)

        while (!(programA.deadlock && programB.deadlock)) {
            programA.execute()
            programB.execute()
        }
        return programB.sends
    }
}

fun main() = Day.runDay(Y17D18::class)

//    Class creation: 19ms
//    Part 1: 9423 (2ms)
//    Part 2: 7620 (22ms)
//    Total time: 44ms
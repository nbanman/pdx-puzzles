package org.gristle.adventOfCode.y2016.shared

class Assembunny(private val registers: IntArray = IntArray(4)) {

    companion object {
        fun parseInstructions(input: String) = input.lineSequence().map { it.split(' ') }.toList()
    }

    init {
        require(registers.size == 4) { "Invalid registry value." }
    }

    private fun String.toRegister(): Int = this[0] - 'a'

    private fun Char.toRegister(): Int = this - 'a'

    private fun String.valueOf() = toIntOrNull() ?: registers[toRegister()]

    fun reset(): Assembunny {
        for (idx in registers.indices) registers[idx] = 0
        return this
    }

    operator fun get(register: Char): Int {
        require(register in "abcd")
        return registers[register.toRegister()]
    }

    operator fun set(register: Char, value: Int) {
        require(register in "abcd")
        registers[register.toRegister()] = value
    }

    fun runInstructions(input: String): Assembunny {
        return runInstructions(parseInstructions(input))
    }

    fun runInstructions(instructions: List<List<String>>): Assembunny {

        var parser = 0
        val toggles = BooleanArray(instructions.size)

        fun cpy(instruction: List<String>) {
            registers[instruction[2].toRegister()] = instruction[1].valueOf()
        }

        fun jnz(instruction: List<String>) {
            if (instruction[1].valueOf() != 0) parser += (instruction[2].valueOf() - 1)
        }

        fun inc(instruction: List<String>) {
            registers[instruction[1].toRegister()] = registers[instruction[1].toRegister()] + 1
        }

        fun dec(instruction: List<String>) {
            registers[instruction[1].toRegister()] = registers[instruction[1].toRegister()] - 1
        }

        while (parser in instructions.indices) {
            val instruction = instructions[parser]
            if (toggles[parser]) {
                when (instruction[0][0]) {
                    'c' -> jnz(instruction)
                    'i' -> dec(instruction)
                    'd' -> inc(instruction)
                    'j' -> cpy(instruction)
                    't' -> inc(instruction)
                }
            } else {
                when (instruction[0][0]) {
                    'c' -> cpy(instruction)
                    'i' -> inc(instruction)
                    'd' -> dec(instruction)
                    'j' -> jnz(instruction)
                    't' -> {
                        val tglIndex = parser + instruction[1].valueOf()
                        if (tglIndex in instructions.indices) toggles[tglIndex] = !toggles[tglIndex]
                    }
                }
            }
            parser++
        }
        return this
    }

    fun runAsSequence(instructions: List<List<String>>): Sequence<List<Int>> = sequence {

        var parser = 0
        val toggles = BooleanArray(instructions.size)

        fun cpy(instruction: List<String>) {
            registers[instruction[2].toRegister()] = instruction[1].valueOf()
        }

        fun jnz(instruction: List<String>) {
            if (instruction[1].valueOf() != 0) parser += (instruction[2].valueOf() - 1)
        }

        fun inc(instruction: List<String>) {
            registers[instruction[1].toRegister()] = registers[instruction[1].toRegister()] + 1
        }

        fun dec(instruction: List<String>) {
            registers[instruction[1].toRegister()] = registers[instruction[1].toRegister()] - 1
        }

        while (parser in instructions.indices) {
            val instruction = instructions[parser]
            yield(registers.toList())
            if (toggles[parser]) {
                when (instruction[0][0]) {
                    'c' -> {
                        jnz(instruction)
                    }
                    'i' -> {
                        dec(instruction)
                    }
                    'd' -> {
                        inc(instruction)
                    }
                    'j' -> {
                        cpy(instruction)
                    }
                    't' -> {
                        inc(instruction)
                    }
                }
            } else {
                when (instruction[0][0]) {
                    'c' -> {
                        cpy(instruction)
                    }
                    'i' -> {
                        inc(instruction)
                    }
                    'd' -> {
                        dec(instruction)
                    }
                    'j' -> jnz(instruction)
                    't' -> {
                        val tglIndex = parser + instruction[1].valueOf()
                        if (tglIndex in instructions.indices) toggles[tglIndex] = !toggles[tglIndex]
                    }
                }
            }
            parser++
        }
    }
} 
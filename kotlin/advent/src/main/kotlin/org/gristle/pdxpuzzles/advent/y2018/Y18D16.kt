package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.groupValues

fun List<Int>.store(location: Int, result: Int) = mapIndexed { index, i ->
    if (index == location) result else i
}

class Y18D16(private val input: String) : Day {
    private enum class Ops(val fn: (reg: List<Int>, a: Int, b: Int, c: Int) -> List<Int>) {
        ADDR({ reg, a, b, c -> reg.store(c, reg[a] + reg[b]) }),
        ADDI({ reg, a, b, c -> reg.store(c, reg[a] + b) }),
        MULR({ reg, a, b, c -> reg.store(c, reg[a] * reg[b]) }),
        MULI({ reg, a, b, c -> reg.store(c, reg[a] * b) }),
        BANR({ reg, a, b, c -> reg.store(c, reg[a] and reg[b]) }),
        BANI({ reg, a, b, c -> reg.store(c, reg[a] and b) }),
        BORR({ reg, a, b, c -> reg.store(c, reg[a] or reg[b]) }),
        BORI({ reg, a, b, c -> reg.store(c, reg[a] or b) }),
        SETR({ reg, a, _, c -> reg.store(c, reg[a]) }),
        SETI( { reg, a, _, c -> reg.store(c, a) } ),
        GTIR( { reg, a, b, c -> reg.store(c, if (a > reg[b]) 1 else 0) } ),
        GTRI( { reg, a, b, c -> reg.store(c, if (reg[a] > b) 1 else 0) } ),
        GTRR( { reg, a, b, c -> reg.store(c, if (reg[a] > reg[b]) 1 else 0) } ),
        EQIR( { reg, a, b, c -> reg.store(c, if (a == reg[b]) 1 else 0) } ),
        EQRI( { reg, a, b, c -> reg.store(c, if (reg[a] == b) 1 else 0) } ),
        EQRR( { reg, a, b, c -> reg.store(c, if (reg[a] == reg[b]) 1 else 0) } );
    }
    
    private val pattern =
        """Before: \[(\d+), (\d+), (\d+), (\d+)]\n(\d+) (\d+) (\d+) (\d+)\nAfter: {2}\[(\d+), (\d+), (\d+), (\d+)]""".toRegex()

    private data class Trainer(val before: List<Int>, val after: List<Int>, val opcode: Int,
                               val a: Int, val b: Int, val c: Int) {
        fun validOps(ops: Set<Ops> = Ops.values().toSet()) = ops.filter { op -> after == op.fn(before, a, b, c) }
    }

    private val trainers = input
        .groupValues(pattern)
        .map { gv ->
            val gvi = gv.map { it.toInt() }
            val before = listOf(gvi[0], gvi[1], gvi[2], gvi[3])
            val after = listOf(gvi[8], gvi[9], gvi[10], gvi[11])
            Trainer(before, after, gvi[4], gvi[5], gvi[6], gvi[7])
        }

    override fun part1() = trainers.count { it.validOps().size >= 3 }

    override fun part2(): Int {
        var trainers = this.trainers
        val ops = Ops.values().toMutableSet()
        val translator = mutableMapOf<Int, Ops>()
        while (ops.isNotEmpty()) {
            val singles = trainers.filter { it.validOps(ops).size == 1 }
            singles.forEach { trainer ->
                if (translator[trainer.opcode] == null) {
                    val op = trainer.validOps(ops).first()
                    translator[trainer.opcode] = op
                    ops.remove(op)
                    trainers = trainers.filter { it.opcode != trainer.opcode }
                }
            }
        }
        val pattern = """(\d+) (\d+) (\d+) (\d+)""".toRegex()
        val testProgram = input
            .takeLastWhile { it != ']' }
            .groupValues(pattern)
            .map { gv -> gv.map { it.toInt() } }
        val p2 = testProgram.fold(listOf(0, 0, 0, 0)) { acc, line ->
            translator.getValue(line[0]).fn(acc, line[1], line[2], line[3])
        }
        return p2[0]
    }
}

fun main() = Day.runDay(Y18D16::class)

//    Class creation: 34ms
//    Part 1: 529 (19ms)
//    Part 2: 573 (32ms)
//    Total time: 85ms

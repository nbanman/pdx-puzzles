package org.gristle.pdxpuzzles.advent.y2018

enum class Ops(val fn: (reg: LongArray, p: Int, a: Int, b: Int, c: Int) -> Unit) {
    ADDR({ reg, p, a, b, c -> reg.store(p, c, reg[a] + reg[b]) }),
    ADDI({ reg, p, a, b, c -> reg.store(p, c, reg[a] + b) }),
    MULR({ reg, p, a, b, c -> reg.store(p, c, reg[a] * reg[b]) }),
    MULI({ reg, p, a, b, c -> reg.store(p, c, reg[a] * b) }),
    BANR({ reg, p, a, b, c -> reg.store(p, c, reg[a] and reg[b]) }),
    BANI({ reg, p, a, b, c -> reg.store(p, c, reg[a] and b.toLong()) }),
    BORR({ reg, p, a, b, c -> reg.store(p, c, reg[a] or reg[b]) }),
    BORI({ reg, p, a, b, c -> reg.store(p, c, reg[a] or b.toLong()) }),
    SETR({ reg, p, a, _, c -> reg.store(p, c, reg[a]) }),
    SETI({ reg, p, a, _, c -> reg.store(p, c, a.toLong()) }),
    GTIR({ reg, p, a, b, c -> reg.store(p, c, if (a > reg[b]) 1 else 0) }),
    GTRI({ reg, p, a, b, c -> reg.store(p, c, if (reg[a] > b) 1 else 0) }),
    GTRR({ reg, p, a, b, c -> reg.store(p, c, if (reg[a] > reg[b]) 1 else 0) }),
    EQIR({ reg, p, a, b, c -> reg.store(p, c, if (a.toLong() == reg[b]) 1 else 0) }),
    EQRI({ reg, p, a, b, c -> reg.store(p, c, if (reg[a] == b.toLong()) 1 else 0) }),
    EQRR({ reg, p, a, b, c -> reg.store(p, c, if (reg[a] == reg[b]) 1 else 0) });

    companion object {
        fun from(s: String) = when (s) {
            "addr" -> ADDR
            "addi" -> ADDI
            "mulr" -> MULR
            "muli" -> MULI
            "banr" -> BANR
            "bani" -> BANI
            "borr" -> BORR
            "bori" -> BORI
            "setr" -> SETR
            "seti" -> SETI
            "gtir" -> GTIR
            "gtri" -> GTRI
            "gtrr" -> GTRR
            "eqir" -> EQIR
            "eqri" -> EQRI
            else -> EQRR
        }
    }
}

private fun LongArray.store(pointer: Int, location: Int, result: Long) {
    this[location] = result
    this[pointer]++
}
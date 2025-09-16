package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit
import org.gristle.pdxpuzzles.utilities.parsing.toLong

class Y24D24(input: String) : Day {
    sealed interface Gate {
        fun toOutputOrNull(wiring: Map<String, Gate>): Output?

        fun getOutputs(left: String, right: String, wiring: Map<String, Gate>): Pair<Output, Output>? {
            val leftGate = wiring.getValue(left)
            if (leftGate !is Output) return null
            val rightGate = wiring.getValue(right)
            if (rightGate !is Output) return null
            return leftGate to rightGate
        }
    }

    data class Output(val value: Boolean): Gate {
        override fun toOutputOrNull(wiring: Map<String, Gate>): Output = this
    }

    data class And(val left: String, val right: String): Gate {
        override fun toOutputOrNull(wiring: Map<String, Gate>): Output? {
            val (leftGate, rightGate) = getOutputs(left, right, wiring) ?: return null
            return Output(leftGate.value and rightGate.value)
        }
    }

    data class Or(val left: String, val right: String): Gate {
        override fun toOutputOrNull(wiring: Map<String, Gate>): Output? {
            val (leftGate, rightGate) = getOutputs(left, right, wiring) ?: return null
            return Output(leftGate.value or rightGate.value)    }
    }

    data class Xor(val left: String, val right: String): Gate {
        override fun toOutputOrNull(wiring: Map<String, Gate>): Output? {
            val (leftGate, rightGate) = getOutputs(left, right, wiring) ?: return null
            return Output(leftGate.value xor rightGate.value)    }
    }

    private val wiring = buildMap {
        val (outputGates, conditionalGates) = input.blankSplit()

        for (s in outputGates.lines()) {
            val (id, value) = s.split(": ")
            put(id, Output(value == "1"))
        }

        for (s in conditionalGates.lines()) {
            val (left, op, right, _, id) = s.split(' ')
            val gate = when (op) {
                "AND" -> And(left, right)
                "OR"  -> Or(left, right)
                "XOR" -> Xor(left, right)
                else -> throw IllegalArgumentException("invalid input")
            }
            put(id, gate)
        }
    }

    private fun wireNumber(wireStart: Char, wiring: MutableMap<String, Gate>): Long = wiring
        .toList()
        .filter { (id, _) -> id.startsWith(wireStart) }
        .sortedByDescending { (id, _) -> id }
        .map { (_, output) -> (output as Output).value }
        .toBooleanArray()
        .toLong()

    override fun part1(): Long {
        val wiring = wiring.toMutableMap()
        val pending = wiring
            .filter { it.value !is Output }
            .toMutableMap()
        var remainingZs = wiring.keys.count { it.startsWith('z') }
        while (remainingZs > 0) {
            for ((id, gate) in pending.toList()) {
                val output = gate.toOutputOrNull(wiring) ?: continue
                wiring[id] = output
                pending.remove(id)
                if (id.startsWith('z')) remainingZs--
            }
        }
        return wireNumber('z', wiring)
    }

    override fun part2(): String {
        val upstream: Map<String, Map<String, List<Pair<String, Gate>>>> =
            buildMap<String, MutableMap<String, MutableList<Pair<String, Gate>>>>
            {
                for ((output, gate) in wiring) {
                    val (left, right) = when (gate) {
                        is And -> gate.left to gate.right
                        is Or -> gate.left to gate.right
                        is Xor -> gate.left to gate.right
                        is Output -> continue
                    }
                    getOrPut(left) { mutableMapOf() }
                        .getOrPut(right) { mutableListOf() }
                        .add(output to gate)

                    getOrPut(right) { mutableMapOf() }
                        .getOrPut(left) { mutableListOf() }
                        .add(output to gate)
                }
            }

        // I needed a hint for part 2. Three words: "ripple carry adder." These circuits create one, and
        // have to in order for them to add their values like the problem says they do. Only some gates
        // output to the wrong ids. This is a series of full adder circuits, connected by the carry value.
        // See https://en.wikipedia.org/wiki/Adder_(electronics).
        // Inspecting the input, I make two assumptions that are true for my input but not necessarily
        // generally. 1) 4 of the 8 errors are in assignment of the z outputs. And the circuits from A00 and
        // B00 to Z00 and C00 are correct.

        // Next get the first carry value
        var carry = upstream
            .getValue("x00")
            .getValue("y00")
            .first { it.second is And }
            .first

        val separate: (String, String) -> Pair<String, String> = { x, y ->
            upstream
                .getValue(x)
                .getValue(y)
                .let { (a, b) ->
                    if (a.second is Xor) {
                        a.first to b.first
                    } else {
                        b.first to a.first
                    }
                }
        }

        val otherErrors = mutableListOf<String>()

        for (i in 1 until wiring.count { (_, gate) -> gate is Output } / 2) {
            val intString = String.format("%02d", i)
            val x = "x$intString"
            val y = "y$intString"
            var (xor1, and1) = separate(x, y)

            // check if xor1 is correct
            if (xor1 != upstream.getValue(carry).keys.first()) {
               xor1 = and1.also { and1 = xor1 }
                otherErrors.add(xor1)
                otherErrors.add(and1)
            }
            val (swap, and2) = separate(xor1, carry)
            if (and1.startsWith('z')) {
                otherErrors.add(swap)
                carry = upstream
                    .getValue(and2)
                    .values
                    .first()
                    .first()
                    .first
            } else if (and2.startsWith('z')) {
                otherErrors.add(swap)
                carry = upstream
                    .getValue(and1)
                    .values
                    .first()
                    .first()
                    .first
            } else {
                val or = upstream
                    .getValue(and1)
                    .getValue(and2)
                    .first()
                    .first
                if (or.startsWith('z') && or != "z45") {
                    otherErrors.add(swap)
                    carry = swap
                } else {
                    carry = or
                }
            }
        }

        val zErrors = wiring
            .toList()
            .filter { (id, gate) ->
                id.startsWith('z') && gate !is Xor && id != "z45"
            }.map { it.first }

        val combined = (zErrors + otherErrors).sorted()

        return combined.joinToString(",")
    }
}



fun main() = Day.runDay(Y24D24::class)

//    Class creation: 4ms
//    Part 1: 51410244478064 (5ms)
//    Part 2: gst,khg,nhn,tvb,vdc,z12,z21,z33 (4ms)
//    Total time: 14ms

@Suppress("unused")
private val test = listOf("""x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
""", """x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
""", """x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00
""")

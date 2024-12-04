package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.rep
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import org.gristle.pdxpuzzles.utilities.parsing.ocr

class Y22D10(input: String) : Day {

    // Parsing is a little unconventional. Rather than reading each line as a command and figuring out what each 
    // command does, we simply want to arrive at a list of cycles paired to their respective registry value. We can
    // start by splitting the string into "words" of non-whitespace characters, attempting to convert them all to
    // Ints, and substituting a 0 when conversion fails because the word is "addx" or "noop." This returns a list of
    // adjustments to the registry that occur at the end of a particular cycle.
    // 
    // The next step is to do a runningFold with a plus function on the list, which returns the list of registry 
    // values at the end of each cycle. We add the cycle information with .withIndex(), which allows us to filter
    // unused registry values without losing track of what cycle we are in.
    private val cpu: Sequence<IndexedValue<Int>> = input
        .splitToSequence('\n', ' ') // split into "words"
        .map { it.toIntOrNull() ?: 0 } // map each word to an Int, or if that fails to 0
        .runningFold(1, Int::plus) // register state per cycle
        .withIndex() // keeps track of which cycle the cpu is in.

    // Note that for both part 1 and part 2, our List gives the registry value *after* a particular cycle is completed.
    // The registry value is updated at the *end* of the cycle, which means we want to use the registry value of the
    // *previous* cycle rather than the cycle mentioned in the instructions. So both evaluations start at cycle 0 
    // (initial state) rather than cycle 1, and perform each evaluation one cycle early.
    override fun part1() = cpu
        .filter { (cycle, _) -> (cycle + 19) % 40 == 0 }
        .sumOf { (cycle, register) -> (cycle + 1) * register }

    override fun part2() = cpu
        .take(240) // The OCR grid needs 240 exact, while the sequence delivers 241 values (initial + 240 additional)
        .map { (cycle, register) -> ((cycle) % 40) in (register - 1)..(register + 1) }
        .toGrid(40)
        .apply { println("\n${rep()}") }
        .ocr()
}

fun main() = Day.runDay(Y22D10::class)

//    Class creation: 21ms
//    Part 1: 16406 (5ms)
//    Part 2: ZKJFBJFZ (12ms)
//    Total time: 38ms
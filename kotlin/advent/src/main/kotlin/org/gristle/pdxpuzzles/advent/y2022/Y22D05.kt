package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.transpose
import org.gristle.pdxpuzzles.utilities.parsing.getIntList
import org.gristle.pdxpuzzles.utilities.parsing.splitOnBlank

private typealias Instruction = Triple<Int, Int, Int>

class Y22D05(input: String) : Day {

    private val crates: List<List<Char>>
    private val instructions: List<Instruction>

    init {
        // Split the input into two strings, the first describing the crate arrangement, and the second giving
        // rearrangement instructions.
        val (crateStrings, instructionsStrings) = input.lines().splitOnBlank()
        crates = crateStrings
            .dropLast(1) // gets rid of line labeling the stack numbers
            .transpose() // flip the strings vertically
            .filter { it.last().isLetter() } // get rid of strings that don't have crate information
            .map { it.trimStart().toList() } // create and load up stacks
        instructions = instructionsStrings.map {
            it.getIntList().let { (amount, from, to) -> Instruction(amount, from - 1, to - 1) }
        }
    }

    /**
     * Solve by cloning the stacks, following the instructions to rearrange the crates, then outputting the top
     * crate in each stack.
     *
     * Takes a 'rearrange' function to delegate the rearrangement to each part.
     */
    private fun solve(rearrange: (amount: Int, fromStack: ArrayDeque<Char>, toStack: ArrayDeque<Char>) -> Unit): String {
        // create arrayDeques for mutation
        val stacks: List<ArrayDeque<Char>> = crates.map { ArrayDeque(it) }
        instructions.forEach { (amount, fromIndex, toIndex) ->
            rearrange(amount, stacks[fromIndex], stacks[toIndex])
        }
        return buildString { stacks.forEach { stack -> append(stack.first()) } }
    }

    /**
     * Building block for the 'rearrange' function, moving crates from one stack to another.
     */
    private fun move(amount: Int, fromStack: ArrayDeque<Char>, toStack: ArrayDeque<Char>) {
        repeat(amount) {
            val letter = fromStack.removeFirst()
            toStack.addFirst(letter)
        }
    }

    override fun part1() = solve { amount, fromStack, toStack ->
        move(amount, fromStack, toStack)
    }

    // Same as part1 except instead of moving directly from one stack to another, we put them in a separate
    // stack to reverse the order that they are moved twice (thus canceling each other out and maintaining the 
    // original order).
    override fun part2() = solve { amount, fromStack, toStack ->
        val holdingBay = ArrayDeque<Char>()
        move(amount, fromStack, holdingBay) // move to holding stack
        move(amount, holdingBay, toStack) // move from holding stack to new stack
    }
}

fun main() = Day.runDay(Y22D05::class)

//    Class creation: 34ms
//    Part 1: ZSQVCCJLL (4ms)
//    Part 2: QZFJRWHGS (1ms)
//    Total time: 40ms
package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y15D12(private val input: String) : Day {

    // stores information about a particular object or array
    data class JsonBlock(val value: Int, val end: Int)

    private val dividers = charArrayOf('[', ']', '{', '}')

    // given a start index, return the next block
    private fun nextBlock(start: Int, alreadyRed: Boolean = false): JsonBlock {
        var index = start
        if (alreadyRed) {

            // if it's already red, run a simplified algorithm that counts nesting blocks without worrying about
            // numbers, outputting a Block with value 0
            var depth = 0
            while (depth >= 0) {
                index++
                when (input[index]) {
                    '[', '{' -> depth++
                    ']', '}' -> depth--
                }
            }
            return JsonBlock(0, index)
        } else {

            // if it's not already red, run standard algorithm
            var isRed = false
            var value = 0

            // loop continues forever, but since input is well-formed it is guaranteed to return. The purpose is to
            // keep looping until the block's closing bracket is found.
            while (true) {

                // get index of closing bracket 
                val endIndex = input.indexOfAny(dividers, index + 1)

                // get snippet of block before any nesting or closing bracket
                val snippet = input.substring(index, endIndex)

                // logic to add value of any numbers in snippet, or to flag as red if :"Red" is found in snippet
                if (!isRed) {
                    if (snippet.contains(":\"red\"")) {
                        isRed = true
                        value = 0
                    } else {
                        value += snippet.getInts().sum()
                    }
                }

                if (input[endIndex] in "]}") { // if block closes...
                    return JsonBlock(value, endIndex) // ..return it
                } else { // ..else if a nesting block exists
                    val innerBlock = nextBlock(endIndex, isRed) // ..get the inner block by recursion 
                    value += innerBlock.value // update the value
                    index = innerBlock.end // move the index to after the nested block
                }
            }
        }
    }

    // finds all the numbers in the string and adds them
    override fun part1() = input.getInts().sum()

    // finds the value of the main block
    override fun part2() = nextBlock(0).value
}

fun main() = Day.runDay(Y15D12::class)

//    Class creation: 15ms
//    Part 1: 111754 (4ms)
//    Part 2: 65402 (7ms)
//    Total time: 27ms
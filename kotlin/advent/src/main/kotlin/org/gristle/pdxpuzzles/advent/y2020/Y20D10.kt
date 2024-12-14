package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y20D10(input: String) : Day {
    // parse adapters from outlet, sort from lowest rating. Then add charging outlet and end devices.
    // Finally convert to a list of the joltage differences between devices.
    private val joltageDifferences: List<Int> = input // raw String
        .lines() // to List of String broken up by line
        .map(String::toInt) // convert List<String> to List<Int> 
        .sorted() // sort List lowest to highest
        .let { adapters -> // Then add charging outlet and end devices.
            ArrayList<Int>(adapters.size + 2).apply {
                add(0)
                addAll(adapters)
                add(adapters.last() + 3)
            }
        }.zipWithNext { a, b -> b - a } // map to List of delta from previous device

    override fun part1() = joltageDifferences
        .count { it == 3 } // count number of 3-jolt differences
        .let { jolt3s ->
            val jolt1s = joltageDifferences.size - jolt3s // derive number of 1-jolt differences
            jolt1s * jolt3s // answer to pt 1
        }

    // basic idea to reduce the calculations is divide and conquer. Wherever there is a 3-jolt difference
    // that adapter and the adapter before it *must* be in the combination. So split the list using the 3-jolt 
    // differences. You then have a bunch of sublists with 1-jolt differences. The maximum number of 1s you see is
    // 4, so you can use a lookup table to count the number of possible permutations in each sublist. Multiply them all 
    // together and you get your answer.
    override fun part2() = joltageDifferences
        .joinToString("") // join differences to one string before splitting in a different way
        .split('3') // both devices 3 apart must be in chain so don't permute
        .map { // each string of 1s represents devices one away from each other.
            // the last '1' in string is required though b/c it's 3 away from the next device
            when (it.length) { // convert to number of permutations for the optional devices
                4 -> 7 // 1111, 1101, 1011, 1001, 0111, 0101, 0011 (0001 not allowed b/c that's 4 apart)
                3 -> 4 // 111, 101, 011, 001  
                2 -> 2 // 11, 01
                else -> 1 // 1
            }
        }.fold(1L, Long::times) // multiply them by each other to get answer to pt 2
}

fun main() = Day.runDay(Y20D10::class)

//    Class creation: 4ms
//    Part 1: 1890 (2ms)
//    Part 2: 49607173328384 (2ms)
//    Total time: 9ms
package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.lcm
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit

class Y23D08(input: String) : Day {

    private val directions: Sequence<Char>
    private val network: Map<String, Pair<String, String>>
    
    init {
        val (dirStr, netStr) = input.blankSplit()
        
        // infinite sequence of directions (repeats after string ends)
        directions = generateSequence(dirStr.asSequence()) { it }.flatten()
        
        // representing a puzzle map with a Kotlin map
        network = netStr
            .lines()
            .associate { line -> 
                val (node, left, right) = line.split(" = (", ", ", ")")
                node to Pair(left, right)
            }
    }
        
    // deliver steps needed for traveler to go from startNode to an end condition,
    private inline fun traverse(startNode: String, endCondition: (String) -> Boolean): Int = directions
        .runningFold(startNode) { node, dir ->
            network.getValue(node).let { (left, right) -> if (dir == 'L') left else right }
        }.indexOfFirst(endCondition)
    
    override fun part1() = traverse("AAA") { it == "ZZZ" }

    // find how long it takes for each ghost to make it from a to z, making the *massive* assumption that
    // they immediately cycle (they do). These are all different. So lcm gives you the answer.
    override fun part2(): Long = network.keys
        .filter { it.last() == 'A' }
        .map { node -> traverse(node) { it.last() == 'Z' }.toLong() }
        .lcm()
}

fun main() = Day.runDay(Y23D08::class)

//    Class creation: 16ms
//    Part 1: 19241 (13ms)
//    Part 2: 9606140307013 (31ms)
//    Total time: 61ms

@Suppress("unused")
private val sampleInput = listOf(
    """RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
""", """LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
""",
)

@Suppress("unused")
private val sampleInput2 = listOf(
    """LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
""" to "6"
)
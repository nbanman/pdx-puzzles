package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y23D15(input: String) : Day {

    private val initialization = input.split(',')
    
    private fun String.toHash() = fold(0) { acc, c -> (acc + c.code) * 17 % 256 }
    
    override fun part1() = initialization.sumOf { it.toHash() }

    override fun part2(): Int {
        val boxes = List(256) { mutableMapOf<String, Int>()}
        initialization.forEach { step ->
            val (label, operation) = step.partition { it.isLetter() }
            val box = label.toHash()
            if (operation == "-") {
                boxes[box].remove(label)
            } else {
                boxes[box][label] = operation.last().digitToInt() 
            }
        }
        return boxes
            .mapIndexed { boxIndex, box ->
                box.values.foldIndexed(0) { lensIndex, acc, focalLength -> 
                    acc + (boxIndex + 1) * (lensIndex + 1) * focalLength 
                }
            }.sum()
    }
}

fun main() = Day.runDay(Y23D15::class)

//    Class creation: 3ms
//    Part 1: 505427 (1ms)
//    Part 2: 243747 (7ms)
//    Total time: 12ms

@Suppress("unused")
private val sampleInput = listOf(
    """rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7""",
)
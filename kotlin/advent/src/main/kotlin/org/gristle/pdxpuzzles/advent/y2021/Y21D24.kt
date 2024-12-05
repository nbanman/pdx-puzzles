package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import java.util.*
import kotlin.math.max
import kotlin.math.min

class Y21D24(input: String) : Day {
    /**
     * Input boils down to an 18 line function run 14 times. The function has two parameters, x and y, and
     * the algorithm depends on whether x is positive. 7 of these have a positive x value and 7 have a
     * negative x value. When x is positive, z := 26 * z + y + the supplied digit. Thus, z increases. When x
     * is negative, if (z % 26) + x equals the supplied digit, z := z / 26, rounded down to nearest integer. If
     * it doesn't equal the supplied digit, z becomes an even larger number.
     *
     * In order for the final value to be 0, every step with a negative x must result in division by 26. Thus,
     * when such a step is encountered, the z value must be such that (z % 26) + x is a number between 1..9. We
     * use a stack to keep track of what z will have to be for the negative x steps. We pair the z-increasing
     * functions with the z-decreasing functions, and find shared values that will satisfy both.
     */

    // Tuple to carry step information.
    data class Step(val order: Int, val x: Int, val y: Int)

    // Tuple to hold "push" and "pop" steps together. 
    data class PairedSteps(val push: Step, val pop: Step)

    // Take the input, glean the parameters to the function, and pair the "push" and "pop" steps using a stack. 
    private val pairedSteps = input
        .lines()
        .chunked(18) // Split the program into the 18-line repeating function
        .mapIndexed { index, lines ->
            val xInc = lines[5].substringAfterLast(" ").toInt() // Extract x
            val yInc = lines[15].substringAfterLast(" ").toInt() // Extract y
            Step(index, xInc, yInc) // Map to a step
        }.let { steps -> // Take those steps and pair them together.
            buildList<PairedSteps> {
                val orderStack: Deque<Step> = ArrayDeque() // We will use a stack to pair the steps.
                steps.forEach { step ->
                    if (step.x >= 0) { // If x is positive, this is a "push" step.
                        orderStack.push(step)
                    } else { // If x is negative, pop the last "pushed" step, and pair the two together.
                        add(PairedSteps(orderStack.pop(), step))
                    }
                }
                sortBy { it.push.order } // Sort by the order of the "push" steps so that z is increased properly
            }
        }

    fun solve(findIntersection: (pushMax: Int, popMax: Int) -> Int): Long {
        var z = 0L

        // The steps are paired and are run in the order that the "push" steps occur. This presents a problem
        // because the "pull" steps can occur out of order. However, we know that the "pull" steps always result
        // in z:= z / 26. So this kludge allows us to figure how many intervening "pull" steps occurred since the
        // last "push" step, and reduce z accordingly.
        var lastIncrease = 0

        // This is our answer, an array of digits. It is a mutable list because the digits are not found in order. 
        val modelNumber = MutableList(pairedSteps.size * 2) { 0 }

        pairedSteps.forEach { step ->
            val increaseZ = step.push.y + 26 * z // The amount z will increase by, apart from the supplied digit
            val pushMax = ((increaseZ + 9) % 26).toInt() // The highest amount z can increase by, mod 26.
            val popMax = -step.pop.x + 9 // The highest allowed value for (z % 26) that still results in z := z / 26

            /* 
             * If we are looking for the highest number, we grab the minimum between the pushMax and the popMax.
             * Those numbers represent the two individual maximums; by taking the minimum between the two, we grab
             * the largest number that works for both.
             * 
             * If we are looking for the lowest number, we grab the maximum between the pushMax and the popMax, 
             * then subtract by 8. The subtraction effectively makes it a comparison between the pushMin and the
             * popMin. The maximum between these two represents the smallest number that works for both.
             */
            val intersection = findIntersection(pushMax, popMax)

            // Finds the digit corresponding to the intersection z value for both push and pop places. 
            modelNumber[step.push.order] = 9 - (pushMax - intersection)
            modelNumber[step.pop.order] = 9 - (popMax - intersection)

            // Applies the z-decrease step for the number of times that a pop had occured since the last push
            repeat(step.push.order - lastIncrease - 1) { z /= 26 }

            // Applies the z-increase step.
            z = increaseZ + modelNumber[step.push.order]
            lastIncrease = step.push.order
        }

        return modelNumber.joinToString("").toLong()
    }

    override fun part1() = solve { pushMax, popMax -> min(pushMax, popMax) }

    override fun part2() = solve { pushMax, popMax -> max(pushMax, popMax) - 8 }
}

fun main() = Day.runDay(Y21D24::class)
//    var time = System.nanoTime()
//    val c = Y21D24(readRawInput("y2021/d24"))
//    println("Class creation: ${elapsedTime(time)}ms")
//    time = System.nanoTime()
//    println("Part 1: ${c.part1()} (${elapsedTime(time)}ms)") // 92969593497992
//    time = System.nanoTime()
//    println("Part 2: ${c.part2()} (${elapsedTime(time)}ms)") // 81514171161381
//}
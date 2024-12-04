@file:Suppress("UNUSED")

package org.gristle.pdxpuzzles.advent.utilities

import org.gristle.pdxpuzzles.utilities.objects.Stopwatch
import org.gristle.pdxpuzzles.utilities.objects.TimeUnits
import org.gristle.pdxpuzzles.utilities.parsing.getIntList
import kotlin.reflect.KClass
import kotlin.reflect.KFunction

interface Day {
    fun part1(): Any?
    fun part2(): Any?

    companion object {
        fun <T : Any> runDay(
            kClass: KClass<T>,
            sampleInput: String? = null,
        ) {
            val constructor = kClass.constructors.first()
            val (year, day) = kClass.simpleName?.getIntList()
                ?: throw IllegalArgumentException("Class does not have a name")
            println("[$year Day $day]")
            val input = sampleInput?.trimEnd { it == '\n' } ?: getInput(year, day)
            val part2: String
            val timer = Stopwatch(true)
            val c = constructor.call(input) as Day
            println("Class creation: ${timer.lap()}ms")
            val part1 = c.part1().toString()
            println("\tPart 1: $part1 (${timer.lap()}ms)")
            if (day != 25) {
                part2 = c.part2().toString()
                println("\tPart 2: $part2 (${timer.lap()}ms)")
            }
            println("Total time: ${timer.elapsed()}ms")
        }

        fun <T : Any> runDay(
            kClass: KClass<T>,
            sampleInput: List<String>,
            displayInput: Boolean = false,
        ) {
            val constructor = kClass.constructors.first()
            val (year, day) = kClass.simpleName?.getIntList()
                ?: throw IllegalArgumentException("Class does not have a name")
            println("[$year Day $day]")
            sampleInput.forEachIndexed { index, sample ->
                val trimmedSample = sample.trimEnd { it == '\n' }
                print("${index + 1}:")
                val inputString = if (displayInput) " $trimmedSample\t" else "\t"
                print(inputString)
                val c = constructor.call(trimmedSample) as Day
                print("Part 1: ${c.part1()}")
                println("\tPart 2: ${c.part2()}")
            }
        }

        fun <T : Any> testPart(
            kClass: KClass<T>,
            part: Int,
            sampleInput: List<Pair<String, String>>,
            displayInput: Boolean = false,
        ) {
            val constructor = kClass.constructors.first()
            val (year, day) = kClass.simpleName?.getIntList()
                ?: throw IllegalArgumentException("Class does not have a name")
            println("[$year Day $day]")
            sampleInput.forEachIndexed { index, (sample, answer) ->
                val trimmedSample = sample.trimEnd { it == '\n' }
                print("${index + 1}:\t")
                val c = constructor.call(trimmedSample) as Day
                val result =
                    when {
                        part == 1 -> c.part1()
                        part == 2 && day != 25 -> c.part2()
                        else -> throw IllegalArgumentException("Invalid part number: $part")
                    }.convertToString()
                if (result == answer) {
                    print("SUCCESS\t")
                } else {
                    print("FAILURE\t")
                }
                print("$result ($answer)")
                if (displayInput) println("\t$trimmedSample") else println()
            }
        }

        fun <T : Any> testDay(
            kClass: KClass<T>,
            sampleInput: String? = null,
            skipPartOne: Boolean = false,
            skipPartTwo: Boolean = false
        ): Pair<String, String> {
            val constructor: KFunction<T> = kClass.constructors.first()
            val (year, day) = kClass.simpleName?.getIntList()
                ?: throw IllegalArgumentException("Class does not have a name")
            val input = sampleInput ?: getInput(year, day)
            val c = constructor.call(input) as Day
            val part1 = if (skipPartOne) "skipped" else c.part1().toString()
            val part2 = if (skipPartTwo || day == 25) "skipped" else c.part2().toString()
            return part1 to part2
        }

        fun <T : Any> benchmarkDay(
            kClass: KClass<T>,
            warmups: Int = 5,
            iterations: Int = 50,
            reinstantiate: Boolean = true,
            sampleInput: String? = null
        ) {
            val constructor = kClass.constructors.first()
            val (year, day) = kClass.simpleName?.getIntList()
                ?: throw IllegalArgumentException("Class does not have a name")
            val input = sampleInput ?: getInput(year, day)
            val timer = Stopwatch(false, TimeUnits.US)
            println("${kClass.simpleName} Part 1\n")
            val p1Average = benchmark(constructor, input, warmups, iterations, timer, 1, reinstantiate)
            println("\n${kClass.simpleName} Part 2\n")
            val p2Average = benchmark(constructor, input, warmups, iterations, timer, 2, reinstantiate)
            val not = if (reinstantiate) "" else " not"
            println("\nInstance$not reinstantiated after each run.")
            println("Parts 1 and 2: ${p1Average + p2Average} us/op [Average]")
        }

        private fun <T: Any> benchmark(
            constructor: KFunction<T>,
            input: String,
            warmups: Int,
            iterations: Int,
            timer: Stopwatch,
            part: Int,
            reinstantiate: Boolean
        ): Long {
            timer.start()
            var c: Day? = null
            for (warmup in 1..warmups) {
                print("Warm-up $warmup: ")
                timer.lap()
                if (c == null || reinstantiate) c = constructor.call(input) as Day
                when (part) {
                    1 -> c.part1()
                    2 -> c.part2()
                    else -> throw IllegalArgumentException("Function does not exist for part $part")
                }
                println("${timer.lap()} us/op")
            }
            val times = (1..iterations)
                .map { iteration ->
                    print("Iteration $iteration: ")
                    timer.lap()
                    if (c == null || reinstantiate) c = constructor.call(input) as Day
                    when (part) {
                        1 -> c!!.part1()
                        2 -> c!!.part2()
                        else -> throw IllegalArgumentException("Function does not exist for part $part")
                    }
                    timer.lap().also { println("$it us/op") }
                }
            val average = times.average()
            println("\n$average us/op [Average]")
            return average.toLong()
        }
    }
}

private fun <E : Any?> E.convertToString(): String? = when (this) {
    is String -> this
    is Int -> this.toString()
    is Long -> this.toString()
    is Boolean -> this.toString()
    else -> null
}
@file:Suppress("UNUSED")

package org.gristle.pdxpuzzles.everybodycodes.utilities

import org.gristle.pdxpuzzles.utilities.objects.Stopwatch
import org.gristle.pdxpuzzles.utilities.parsing.getIntList
import kotlin.reflect.KClass

interface Day {
    fun part1(input: String): Any?
    fun part2(input: String): Any?
    fun part3(input: String): Any?

    companion object {
        fun <T : Any> runDay(kClass: KClass<T>) {
            val c = kClass.objectInstance as Day
            val (year, day) = kClass.simpleName?.getIntList()
                ?: throw IllegalArgumentException("Class does not have a name")
            println("[$year Day $day]")
            val inputs: List<String?> = getInputs(year, day)
            val timer = Stopwatch(true)
            for ((quest, input) in inputs.withIndex()) {
                if (input != null) {
                    val quest = c.runQuest(quest, input)
                    println("\tQuest 1: $quest (${timer.lap()}ms)")
                }
            }
            println("Total time: ${timer.elapsed()}ms")
        }
    }
}

private fun Day.runQuest(quest: Int, input: String): String =
    when (quest) {
        0 -> part1(input)
        1 -> part2(input)
        2 -> part3(input)
        else -> null
    }.toString()


package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getIntList
import org.gristle.pdxpuzzles.utilities.objects.transpose
import java.time.LocalDateTime

class Y18D04(input: String) : Day {

    // Parses to map using the guard ID as the key and a list of specific sleep schedules for days as the value
    private val guards: Map<Int, List<List<Boolean>>> = buildMap<Int, MutableList<List<Boolean>>> {

        // Parse input to list of log entries sorted by dateTime
        val log = input
            .lineSequence()
            .map { line ->
                val ints = line.getIntList()
                val (year, month, day, hour, minute) = ints
                val date = LocalDateTime.of(year, month, day, hour, minute)
                val id = if (ints.size == 6) ints[5] else -1
                date to id
            }.sortedBy { (dateTime, _) -> dateTime }
            .toList()

        // index of log listings, as there is a nested while loop requiring more fine-tuned control 
        var index = 0

        // run loop until all log listings have been processed
        do {

            // get id of guard on duty and set various variables to track the sleep log of the particular day
            val (_, id) = log[index]
            val hour = BooleanArray(60)
            var minute = 0
            var asleep = false
            index++

            // nested while loop populates the hour array with sleep info
            while (index < log.size && log[index].second == -1) {
                val nextMinute = log[index].first.minute
                for (min in minute until nextMinute) hour[min] = asleep
                minute = nextMinute
                asleep = !asleep
                index++
            }

            // fills out the rest of the hour array
            for (min in minute..59) hour[min] = asleep

            // adds the hour to the map
            getOrPut(id) { mutableListOf() }.add(hour.toList())
        } while (index < log.size)
    }

    override fun part1() = guards
        .maxBy { (_, days) -> days.sumOf { hour -> hour.count { it } } } // gets guard with most hours asleep
        .let { (id, days) ->

            // finds the minute where the guard is most often asleep
            val mostAsleep = (0..59).maxBy { minute -> days.count { day -> day[minute] } }

            // returns the id * the minute
            id * mostAsleep
        }

    override fun part2(): Int = guards
        .map { (id, days) -> // for each guard...
            days
                .transpose()
                .mapIndexed { minute, sleepRecord -> // ...make a table of each minute and how many times asleep...
                    minute to sleepRecord.count { it }
                }.maxBy { (_, asleep) -> asleep } // ...then grab the minute that guard was most often asleep...
                .let { (minute, asleep) -> Triple(id, minute, asleep) } // ...and package it all together
        }.maxBy { (_, _, asleep) -> asleep } // find guard who had the minute that was most asleep
        .let { (id, minute) -> id * minute } // multiply that minute by the guard's ID
}

fun main() = Day.runDay(Y18D04::class)

//    Class creation: 50ms
//    Part 1: 19025 (4ms)
//    Part 2: 23776 (5ms)
//    Total time: 59ms
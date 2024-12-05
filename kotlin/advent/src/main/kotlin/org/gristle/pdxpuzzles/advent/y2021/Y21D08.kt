package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y21D08(input: String) : Day {

    data class Display(val wires: List<Set<Char>>, val display: List<Set<Char>>) {

        // digitMap uses the set of wires as a key, and the corresponding digit as the value. But building out the
        // map involves bootstrapping, using digits that have already been matched to match other digits. It is
        // useful to access these by digit. Therefore, the map is first built out with the digit as the key and the
        // set of wires as the value. Once built out, the keys and values are swapped.
        private val digitMap = buildMap {
            val wireGroups = wires.groupBy { it.size } // separate digits by number of wires
            // 1, 4, 7, 8 all have unique numbers of wires
            put(1, wireGroups.getValue(2).first()) 
            put(4, wireGroups.getValue(4).first())
            put(7, wireGroups.getValue(3).first())
            put(8, wireGroups.getValue(7).first())
            // the remaining digits can be derived from comparisons with numbers that have already been found
            put(6, wireGroups.getValue(6).first { it.intersect(getValue(1)).size == 1 })
            put(9, wireGroups.getValue(6).first { it.intersect(getValue(4)).size == 4 })
            put(5, wireGroups.getValue(5).first { it.intersect(getValue(6)).size == 5 })
            put(2, wireGroups.getValue(5).first { it.intersect(getValue(5)).size == 3 })
            put(3, wireGroups.getValue(5).first { it.intersect(getValue(5)).size == 4 })
            put(0, wireGroups.getValue(6).first { it.intersect(getValue(5)).size == 4 })
        }.entries.associate { it.value to it.key } // reverse map so that the value becomes the key and vice-versa

        // Readout takes the list of wires used for the display, applies the digitMap to get their digit values,
        // then concatenates them.
        val outputValue = display
            .map { digitMap.getValue(it) }
            .joinToString("")
            .toInt()
    }

    // A list of Displays, created by parsing each line.
    private val displays = input
        .lines()
        .map { line -> // For each line...
            val (wires, display) = line // Make lists of all the wires and lists of all on display
                .split(" | ") // ...by first splitting the input by the " | " symbol into two strings
                .map { s -> // ...then turning the two strings into Sets of Char
                    s.split(' ').map { it.toSet() }
                }
            // Finally make a Display for each line.
            Display(wires, display)
        }

    // Calculates how many times digits 1, 4, 7, or 8 appear by looking at the sets of wires in the Display
    // and counting the sets that do not use 5 or 6 wires. All other sets will correspond to digits 1, 4, 7, or 8.
    override fun part1() = displays.flatMap(Display::display).count { it.size !in 5..6 }

    // Sums the output values of all the displays
    override fun part2() = displays.sumOf(Display::outputValue)
}

fun main() = Day.runDay(Y21D08::class)

//    Class creation: 51ms
//    Part 1: 397 (0ms)
//    Part 2: 1027422 (0ms)
//    Total time: 52ms
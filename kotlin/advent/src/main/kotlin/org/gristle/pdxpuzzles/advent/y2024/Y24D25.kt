package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.map
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit

class Y24D25(private val input: String) : Day {

    override fun part1(): Int {
        val (locks, keys) = input
            .blankSplit()
            .partition { it[0] == '#' }
            .map { schematics ->
                schematics.map { lockOrKey ->
                    lockOrKey.toGrid().columns().map { pin -> pin.count { it == '#' } }
                }
            }
        var fits = 0
        for (lock in locks) {
            keyLoop@for (key in keys) {
                for (pin in 0 until 5) {
                    for ((lockPin, keyPin) in lock.zip(key)) {
                        if (lockPin + keyPin > 7) continue@keyLoop
                    }
                }
                fits++
            }
        }
        return fits
    }

    override fun part2() = "Merry Xmas!"
}

fun main() = Day.runDay(Y24D25::class)

//    Class creation: 1ms
//    Part 1: 3287 (38ms)
//    Total time: 40ms

@Suppress("unused")
private val test = listOf("""#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
""")
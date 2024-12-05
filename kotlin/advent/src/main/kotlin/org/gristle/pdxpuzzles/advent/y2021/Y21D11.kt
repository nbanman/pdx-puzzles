 package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import org.gristle.pdxpuzzles.utilities.objects.toMutableGrid

 class Y21D11(input: String) : Day {

    private val grid = input.toGrid(Character::getNumericValue)

    // a Sequence which keeps track of mutating state of the grid through any number of turns, emitting the
    // number of flashes that occurred each turn
    private val flashSequence = sequence {
        // cave is mutable state internal to the sequence. The grid will keep updating each iteration.
        val cave = grid.toMutableGrid()
        // outer loop runs forever. The yield turns this into a generator. Each time, it increases the energy
        // level of each octopus, then runs an inner loop that handles the flashing, then yields the number of
        // flashes
        do {
            cave.indices.forEach { index -> cave[index] = cave[index] + 1 } // increases energy level 
            do { // inner loop to handle flashing
                // gets indices of all octopuses ready to flash
                val flasherIndices = cave.withIndex().filter { it.value > 9 }.map { it.index } 

                // for each flashing octopus, reset energy to 0
                flasherIndices.forEach { index -> cave[index] = 0 }
                
                // for each flashing octopus, get its neighbors, filter out those that have already flashed,
                // and add one to the energy
                flasherIndices
                    .flatMap { cave.getNeighborIndices(it, true) }
                    .filterNot { cave[it] == 0 }
                    .forEach { cave[it] = cave[it] + 1 }
            } while (flasherIndices.isNotEmpty()) // run the inner loop until no more octopuses flash
            yield(cave.count { it == 0 }) // emit the number of octopuses that flashed this turn
        } while (true)
    }

    override fun part1() = flashSequence.take(100).sum() // sum up all the flashes that occurred in 1st 100 turns

    // look for the first turn where every octopus has flashed (ie, been reset). Get the index of that turn and 
    // add one to compensate for zero-indexing.
    override fun part2() = flashSequence.indexOfFirst { it == grid.size } + 1
}

fun main() = Day.runDay(Y21D11::class)

//    Class creation: 15ms
//    Part 1: 1669 (28ms)
//    Part 2: 351 (20ms)
//    Total time: 64ms
package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.MutableGrid
import org.gristle.pdxpuzzles.utilities.objects.toMutableGrid

typealias Grove = MutableGrid<Boolean>

class Y22D23(input: String) : Day {

    enum class Direction {
        N, S, W, E;

        fun advance(n: Int): Direction = entries[(ordinal + n) % 4]
    }

    private val grove: Grove = input.toMutableGrid { it == '#' }

    // Checks to see if an elf has any neighbors.
    private fun Coord.adjacentEmpty(grove: Grove, dir: Direction): Boolean = when (dir) {
        Direction.N -> listOf(Coord(x - 1, y - 1), copy(y = y - 1), Coord(x + 1, y - 1))
        Direction.S -> listOf(Coord(x - 1, y + 1), copy(y = y + 1), Coord(x + 1, y + 1))
        Direction.W -> listOf(Coord(x - 1, y - 1), copy(x = x - 1), Coord(x - 1, y + 1))
        Direction.E -> listOf(Coord(x + 1, y - 1), copy(x = x + 1), Coord(x + 1, y + 1))
    }.all { !grove.validCoord(it) || !grove[it] }

    // Checks to see if elf has neighbors in a given direction.
    private fun Coord.adjacentEmpty(grove: Grove): Boolean = listOf(
        Coord(x - 1, y - 1), copy(y = y - 1), Coord(x + 1, y - 1),
        copy(x = x - 1), copy(x = x + 1),
        Coord(x - 1, y + 1), copy(y = y + 1), Coord(x + 1, y + 1),
    ).all { !grove.validCoord(it) || !grove[it] }

    // Moves an elf in a particular direction. Using the Coord native move function makes the program 2.5x slower!  
    private fun Coord.moveElf(dir: Direction): Coord = when (dir) {
        Direction.N -> copy(y = y - 1)
        Direction.S -> copy(y = y + 1)
        Direction.W -> copy(x = x - 1)
        Direction.E -> copy(x = x + 1)
    }

    // The meat of the program. Accepts a Grove and outputs a new Grove with the elves moved.
    private fun Grove.move(dir: Direction): Grove {

        // The boundaries of the elves move. To keep the grid the right size, find the bounds of the grove
        // then make a new grid with a padding of 1 around the boundary. Offset used to map old values to new grid.
        val (dimensions, offset) = getDimensionsAndOffset(1) { it }
        val nextGrove: Grove = MutableGrid(dimensions.x, dimensions.y) { false }

        // For each elf, try to make a move. Moves are greedy in that if a space is open, an elf will take it.
        // If a later elf thinks that's a good spot too, the spot in the new grove is checked. If an elf has taken
        // that spot, the current elf stays in place and the old elf moves back.
        coords() // get all positions of grove as a Coord
            .filter { this[it] } // only look at the Coords with elves 
            .forEach { pos -> // pos is the position of the elf in the previous grove
                val offsetPos = pos + offset // offsetPos is where the elf would be in nextGrove

                if (pos.adjacentEmpty(this)) { // if no other elves adjacent...
                    nextGrove[offsetPos] = true // stay in place
                } else {
                    (0..3).firstOrNull { i -> // try to move in each direction...
                        val currDir = dir.advance(i)
                        if (pos.adjacentEmpty(this, currDir)) { // if no elves in that dir...
                            // check to see if another elf has already moved there
                            val prospect = offsetPos.moveElf(currDir)
                            if (nextGrove[prospect]) { // if elf already exists
                                nextGrove[offsetPos] = true // stay in place
                                nextGrove[prospect] = false // remove previous move
                                nextGrove[prospect.moveElf(currDir)] = true // put previous elf back in place
                            } else { // ...else move the elf there!
                                nextGrove[prospect] = true
                            }
                            true
                        } else if (i == 3) { // if we've reached the final direction to check... 
                            nextGrove[offsetPos] = true // stay in place
                            true
                        } else { // if we haven't yet reached the final direction to check, we want to keep going 
                            false
                        }
                    } ?: let { // if above returns false, no move possible so stay in place 
                        nextGrove[offsetPos] = true
                    }
                }
            }

        return nextGrove
    }

    // Sequence delivering subsequent versions of the grove, with shifting winds.
    private val movement = generateSequence(grove to Direction.N) { (current, dir) ->
        current.move(dir) to dir.advance(1)
    }

    override fun part1(): Int = movement
        .take(11) // iterate 10 times from initial
        .last() // we just want the last one
        .let { (grove, _) ->
            // find the width and height of the box if there were no padding left
            val (dimensions, _) = grove.getDimensionsAndOffset(0) { it }
            // the empty spaces are the area of that hypothetical box minus the number of elves
            dimensions.x * dimensions.y - grove.count { it }
        }

    override fun part2(): Int = movement
        .zipWithNext() // zip each grove with the previous grove
        .indexOfFirst { (prev, next) -> // compare the two groves...
            prev.first == next.first // grab the first time when they are the same
        } + 1 // add one because we actually want the second time they are the same
}

// pt1: 3812 (65ms) (original 10228ms)
// pt2: 1003 (697ms) (original 374635ms!!!)
fun main() = Day.runDay(Y22D23::class)

//    Class creation: 11ms
//    Part 1: 3812 (82ms)
//    Part 2: 1003 (1100ms)
//    Total time: 1194ms
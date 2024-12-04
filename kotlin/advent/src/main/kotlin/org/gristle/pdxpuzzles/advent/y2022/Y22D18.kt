package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.graph.Graph
import org.gristle.pdxpuzzles.utilities.objects.Xyz
import org.gristle.pdxpuzzles.utilities.objects.getBounds
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y22D18(input: String) : Day {

    // set of all cubes in the droplet
    private val cubes: Set<Xyz> = input
        .getInts()
        .chunked(3) { (x, y, z) -> Xyz(x, y, z) }
        .toSet()

    // utility function that returns the 6 spaces adjacent to a given cube
    private fun Xyz.adjacent() = Xyz.CROSS.map { it + this }

    // for every cube in the droplet, count the number of adjacent spaces that match a given predicate, then
    // sum that count. 
    private inline fun surfaceArea(predicate: (Xyz) -> Boolean): Int = cubes
        .sumOf { cube -> cube.adjacent().count(predicate) }

    // Predicate returns true when the space isn't occupied by a cube in the droplet. This fulfils the surface area 
    // rules of Part 1.
    override fun part1() = surfaceArea { it !in cubes }

    // Starting from outside the cube, find all the spaces within the bounds that do not contain a droplet cube.
    // The predicate returns true when the space is one of those exterior spaces. This fulfils the surface area rules
    // of Part 2.
    override fun part2(): Int {

        // bounds are IntRanges, one for each of three dimensions, giving the boundaries of the droplet with
        // a 1-space buffer on each side
        val (xBounds, yBounds, zBounds) = cubes.getBounds(padding = 1)

        // Use a BFS flood fill starting from outside the droplet. Since the bounds ranges allow a space of at least
        // one in every dimension, the BFS will go around the entire droplet and try to penetrate it.
        // Returns a set of points in and around the droplet that are part of the exterior.
        val exterior: Set<Xyz> = Graph
            .bfsSequence(
                startId = Xyz(xBounds.first, yBounds.first, zBounds.first),
                defaultEdges = { pos ->
                    pos.adjacent().filter {
                        !cubes.contains(it) // the space is not a cube
                                && it.x in xBounds // is in-bounds on x-axis
                                && it.y in yBounds // y-axis
                                && it.z in zBounds // z-axis
                    }
                }
            ).map { it.id }
            .toSet()

        return surfaceArea { it in exterior }
    }
}

fun main() = Day.runDay(Y22D18::class) // (1286ms original)

//    Class creation: 47ms
//    Part 1: 4332 (33ms)
//    Part 2: 2524 (64ms)
//    Total time: 146ms
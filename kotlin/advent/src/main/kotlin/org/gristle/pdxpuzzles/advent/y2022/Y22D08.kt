package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y22D08(input: String) : Day {

    // Representations of the positions and heights of all the trees in the forest.
    private val treeHeights = input.toGrid(Char::digitToInt)

    // utility function that makes the coordinates aware of whether they are in the forest
    private fun Coord.outOfForest(): Boolean = !treeHeights.validCoord(this)

    // for a given position, provide a list of sequences that generate coordinates radiating away from the position
    // in each of the four directions
    private fun rays(tree: Coord): List<Sequence<Coord>> = Nsew.entries.map { direction ->
        generateSequence(tree.move(direction)) { it.move(direction) }
    }

    // determines whether a sequence should be terminated, returning true if the position is out of the forest 
    // or if the tree at the position blocks the starting tree's line of sight (LOS).
    private fun terminating(pos: Coord, tree: Coord): Boolean {
        if (pos.outOfForest()) return true
        val posHeight = treeHeights[pos]
        return posHeight >= treeHeights[tree]
    }

    // for a given position, checks all directions and returns true if *any* allow LOS out of the forest
    private fun Coord.isVisible(): Boolean {
        val tree = this // unnecessary but provides semantic value
        return rays(tree).any {
            it
                // keep delivering coordinates until out of forest or LOS blocked
                .first { pos -> terminating(pos, tree) }
                .outOfForest() // true if made it out of forest before LOS blocked
        }
    }

    // for a given position, counts number of visible trees in each direction, then multiplies them together
    private fun scenicScore(treehouse: Coord) = rays(treehouse)
        .map {
            it
                // need to track both the position and the index because the index will be used to count the trees
                // and the position will be used to add 1 if LOS is blocked (since a blocking tree 
                // should be counted).    
                .withIndex()
                .first { (_, pos) -> terminating(pos, treehouse) }
                .let { (index, pos) -> index + if (pos.outOfForest()) 0 else 1 }
        }.reduce(Int::times)

    override fun part1(): Int = treeHeights.coords().count { tree -> tree.isVisible() }

    override fun part2(): Int = treeHeights.coords().maxOf(::scenicScore)
}

fun main() = Day.runDay(Y22D08::class)

//    Class creation: 10ms
//    Part 1: 1708 (34ms)
//    Part 2: 504000 (36ms)
//    Total time: 82ms
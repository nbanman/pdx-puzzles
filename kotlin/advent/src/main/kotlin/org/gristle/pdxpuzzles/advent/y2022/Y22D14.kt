package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y22D14(input: String) : Day {

    // Takes each line, builds Coords from the numbers, then builds lines from pairs of coordinates. Adds this to 
    // a set of Coords that constitutes the cavern.
    private val cavern: Set<Coord> = buildSet {
        input.lineSequence().forEach { line ->
            line.getInts() // only look at the numbers in the line
                .chunked(2) { (x, y) -> Coord(x, y) } // turn them into Coords
                .zipWithNext() // zip them together
                .forEach { (prev, next) -> addAll(prev.lineTo(next)) } // create a line of coords between each
        }
    }

    // The deepest part of the cavern recorded.
    private val depth = cavern.maxOf(Coord::y)

    /**
     * Drops a grain of sand one step. Returns that location. Returns null if there is nowhere for the grain to go.
     */
    private fun Coord.fall(cavern: Set<Coord>): Coord? {
        return when {
            !cavern.contains(south()) -> south()
            !cavern.contains(southwest()) -> southwest()
            !cavern.contains(southeast()) -> southeast()
            else -> null
        }
    }

    /**
     * Introduces a grain of sand to the cavern and runs fall() on it repeatedly until fall() returns null
     * (indicating that the grain has nowhere to go and has settled), or the sand has exceeded the depth of the
     * cavern.
     */
    private fun settle(cavern: Set<Coord>): Coord = generateSequence(Coord(500, 0)) { it.fall(cavern) }
        .takeWhile { it.y <= depth + 1 }
        .last()

    /**
     * Keep adding sand until the last grain matches the predicate, then return the index of that grain.
     * For speed reasons, this is an effectful sequence, mutating the cavern rather than returning a new cavern on each
     * iteration.
     */
    private inline fun solve(predicate: (Coord) -> Boolean): Int {

        // make shadowed mutable copy of original cavern
        val cavern = cavern.toMutableSet()

        return generateSequence { settle(cavern) }
            .onEach { cavern.add(it) } // adds the settled grain to the cavern
            .indexOfFirst(predicate)
    }

    /**
     * The fall() sequence can return a grain that is farther than the depth, so part 1 can be solved by getting the
     * index of the first grain that does that.
     */
    override fun part1() = solve { it.y > depth }

    /**
     * The part 2 rules say that there is a floor two spaces below the depth, so sand will start to pile up on this
     * floor until it gets back to the source.
     */
    override fun part2() = solve { it == Coord(500, 0) } + 1
}

fun main() = Day.runDay(Y22D14::class)

//    Class creation: 32ms
//    Part 1: 825 (18ms)
//    Part 2: 26729 (131ms)
//    Total time: 181ms
package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.algorithms.Graph
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y22D12(input: String) : Day {

    private val area = input.toGrid()

    private fun Char.height(): Int = when (this) {
        'S' -> 'a'
        'E' -> 'z'
        else -> this
    }.code

    private val getEdges: (Int) -> List<Int> = { pos ->
        val posHeight = area[pos].height() - 1
        area
            .getNeighborsIndexedValue(pos)
            .mapNotNull { (index, c) -> if (c.height() >= posHeight) index else null }
    }

    private val vertices = Graph.bfsSequence(startId = area.indexOf('E'), defaultEdges = getEdges)

    fun solve(targets: String) = vertices
        .first { area[it.id] in targets }
        .weight
        .toInt()

    override fun part1() = solve("S")
    override fun part2() = solve("Sa")

//    Alternate solution uses A* and is slightly faster, works for all known puzzle inputs but part 2 relies on quirk of
//    inputs that has all possible end points on the far left column of the grid. So the BFS solution above is more
//    general, though less fun.
//
//        fun solve(heuristic: (Coord) -> Double): Int {
//            val getEdges = { pos: Coord ->
//                area
//                    .getNeighborsIndexedValue(pos)
//                    .filter { (_, c) -> c.height() >= area[pos].height() - 1 }
//                    .map { (index, _) -> area.coordOf(index) }
//            }
//            return Graph
//                .aStar(
//                    area.coordOf(startId),
//                    heuristic = heuristic,
//                    defaultEdges = { pos -> getEdges(pos).toEdges() }
//                ).steps()
//        }
//
//        override fun part1(): Int {
//            val target = area.coordOf(area.indexOf('S'))
//            return solve(heuristic = { pos -> pos.manhattanDistance(target).toDouble() })
//        }
//
//        override fun part2() = solve(heuristic = { pos -> pos.x.toDouble() })

}

fun main() = Day.runDay(Y22D12::class)

//    Class creation: 24ms
//    Part 1: 361 (19ms)
//    Part 2: 354 (6ms)
//    Total time: 50ms
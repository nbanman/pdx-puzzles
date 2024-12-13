package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.toCoord
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import java.util.BitSet

class Y24D12(input: String) : Day {
    private val arrangement = input.toGrid()

    data class Region(val area: Int, val perimeter: Int, val sides: Int, val surveyed: BitSet) {
        companion object {
            /**
             * Finds the region for any given plant in the arrangement. Uses DFS to explore the region,
             * with each plot looking at adjacent plots and loading plots of the same plant into the queue.
             * Area and perimeter are calculated as each plot is evaluated. Area is just a count of plots
             * in the region. Perimeter is the sides that don't have a matching plant adjacent, so
             * 4 - [# of adjacent matching plants].
             *
             * Side are a bit kludgy and are relegated to a separate corners function. I refactored to use
             * Todd Ginsberg's counting corners method, which performs about the same but is far more
             * explicable.
             */
            fun new(arrangement: Grid<Char>, intPos: Int, plant: Char): Region {
                // Setting up DFS
                val start = intPos.toCoord(arrangement.width)
                val q = mutableListOf(start)
                val surveyed = mutableSetOf(start)

                // area and perimeter are counted block by block in the DFS loop, so use mutable variables
                var area = 0
                var perimeter = 0
                var corners = 0

                // DFS loop.
                while (q.isNotEmpty()) {
                    val current = q.removeLast()

                    val neighbors = Nsew.entries.associateWith { dir ->
                        val nPos = current.move(dir)
                        val inRegion = plant == arrangement.getOrNull(nPos)
                        nPos to inRegion
                    }

                    area++
                    val inRegion = neighbors.values.filter { (_, inRegion) -> inRegion }
                    perimeter += 4 - inRegion.size

                    val localCorners = corners(neighbors, arrangement, current, plant)
                    if (plant == 'C') {
                        println("$current: $localCorners")
                    }
                    corners += localCorners

                    // add those plots that haven't already been examined to the queue
                    inRegion
                        .filter { (neighbor, _) -> surveyed.add(neighbor) }
                        .forEach { (neighbor, _) -> q.add(neighbor) }
                }
                val bitSet = BitSet(arrangement.size)
                for (pos in surveyed) {
                    bitSet[pos.asIndex(arrangement.width)] = true
                }

                return Region(area, perimeter, corners, bitSet)
            }

            private fun corners(
                neighbors: Map<Nsew, Pair<Coord, Boolean>>,
                arrangement: Grid<Char>,
                current: Coord,
                plant: Char
            ) = listOf(Nsew.NORTH, Nsew.EAST, Nsew.SOUTH, Nsew.WEST, Nsew.NORTH)
                .zipWithNext()
                .count { (a, b) ->
                    if (!neighbors.getValue(a).second && !neighbors.getValue(b).second) {
                        true
                    } else {
                        neighbors.getValue(a).second
                                && neighbors.getValue(b).second
                                && arrangement.getOrNull(current.move(a).move(b)) != plant
                    }
                }
        }
    }

    private fun solve(cost: Region.() -> Int): Int {
        val surveyed = BitSet(arrangement.size)
        val regions = arrangement
            .asSequence()
            .withIndex()
            .filterNot { (pos, _) -> surveyed[pos] }
            .map { (pos, plant) ->
                val region = Region.new(arrangement, pos, plant)
                surveyed.or(region.surveyed)
                region
            }.toList()
        return regions.sumOf { it.cost() }
    }
    override fun part1() = solve { area * perimeter }
    override fun part2() = solve { area * sides }
}
fun main() = Day.runDay(Y24D12::class, test[0])

//    Class creation: 10ms
//    Part 1: 1424472 (89ms)
//    Part 2: 870202 (39ms)
//    Total time: 139ms

//p1 0: 140, 1: 772, 2: 1930
//p2 0: 80, 1: 436, 2: 1206, 3: 236, 4: 368
@Suppress("unused")
private val test = listOf("""AAAA
BBCD
BBCC
EEEC""", """OOOOO
OXOXO
OOOOO
OXOXO
OOOOO""", """RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE""", """EEEEE
EXXXX
EEEEE
EXXXX
EEEEE""", """AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA""")
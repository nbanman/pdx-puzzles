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
             * Side are a bit kludgy and are relegated to a separate calculateSides function.
             */
            fun new(arrangement: Grid<Char>, intPos: Int, plant: Char): Region {
                // Setting up DFS
                val start = intPos.toCoord(arrangement.width)
                val q = mutableListOf(start)
                val surveyed = mutableSetOf(start)

                // area and perimeter are counted block by block in the DFS loop, so use mutable variables
                var area = 0
                var perimeter = 0

                // DFS loop.
                while (q.isNotEmpty()) {
                    val current = q.removeLast()
                    area++

                    // get adjacent plots and only look at the ones that have matching plants
                    val neighbors = arrangement
                        .getNeighborsIndexedValue(current)
                        .filter { (_, nPlant) -> nPlant == plant }
                        .map { (nPos, _) -> nPos.toCoord(arrangement.width) }

                    perimeter += 4 - neighbors.size

                    // add those plots that haven't already been examined to the queue
                    neighbors
                        .filter { neighbor ->
                            if (neighbor in surveyed) {
                                false
                            } else {
                                surveyed.add(neighbor)
                                true
                            }
                        }.forEach { neighbor -> q.add(neighbor) }
                }

                val sides = calculateSides(surveyed)

                val bitSet = BitSet(arrangement.size)
                for (pos in surveyed) {
                    bitSet[pos.asIndex(arrangement.width)] = true
                }

                return Region(area, perimeter, sides, bitSet)
            }

            /**
             * I can't calculate until the entire region is known because the placement
             * of non-adjacent plants in the region affects whether a border is a new side. So after the DFS
             * is finished, I go through each of the points in the region once more, marking each revisit
             * in a separate set (a "resurvey"). Again, I check to see if there is a direction that doesn't have an
             * adjacent matching plant. If so, it is part of a side.
             *
             * But if we just count all these side parts, that's just the perimeter. We need to count each
             * side part just once. So for a side part in any given direction, I check adjacent plots to the
             * right and left. If either one of them has already been "resurveyed", then the side has already
             * been counted, so I don't add to the count.
             *
             * Finally, there can be a situation where the "resurvey" overcounts the number of sides because
             * a contiguous side can be formed from both ends, neither side knowing that eventually they will be
             * joined. I resolve this by checking to see if the piece connects two already resurveyed blocks in a
             * straight line. If it does that, I subtract one.
             */
            private fun calculateSides(surveyed: MutableSet<Coord>): Int {
                val resurveyed = mutableSetOf<Coord>()
                val sides = surveyed.sumOf { pos ->
                    resurveyed.add(pos)
                    val posSides = Nsew.entries
                        .map { it to pos.move(it) }
                        .filter { (_, adjacent) -> adjacent !in surveyed }
                        .sumOf { (dir, _) ->
                            val checkRight = checkTurn(dir.right(), dir, pos, surveyed, resurveyed)
                            val checkLeft = checkTurn(dir.left(), dir, pos, surveyed, resurveyed)
                            when {
                                checkRight && checkLeft -> 1
                                !checkRight && !checkLeft -> -1
                                else -> 0
                            }.toInt()
                        }
                    posSides
                }
                return sides
            }

            private fun checkTurn(turn: Nsew, dir: Nsew, pos: Coord, surveyed: Set<Coord>, resurveyed: Set<Coord>): Boolean {
                val adjacentTurn = pos.move(turn)
                return adjacentTurn !in resurveyed || adjacentTurn.move(dir) in surveyed
            }
        }
    }

    private fun solve(cost: Region.() -> Int): Int {
        val surveyed = BitSet(arrangement.size)
        return arrangement
            .asSequence()
            .withIndex()
            .filterNot { (pos, _) -> surveyed[pos] }
            .sumOf { (pos, plant) ->
                val region = Region.new(arrangement, pos, plant)
                surveyed.or(region.surveyed)
                region.cost()
            }
    }
    override fun part1() = solve { area * perimeter }
    override fun part2() = solve { area * sides }
}
fun main() = Day.runDay(Y24D12::class)

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
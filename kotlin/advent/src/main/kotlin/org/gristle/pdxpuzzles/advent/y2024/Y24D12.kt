package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.toCoord
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import java.util.ArrayDeque
import java.util.BitSet

class Y24D12(input: String) : Day {
    private val arrangement = input.toGrid()

    data class Region(val area: Int, val perimeter: Int, val sides: Int, val surveyed: BitSet) {
        companion object {
            fun new(arrangement: Grid<Char>, intPos: Int, plant: Char): Region {
                val start = intPos.toCoord(arrangement.width)
                val q = ArrayDeque<Coord>()
                q.addLast(start)
                val surveyed = mutableSetOf(start)
                var area = 0
                var perimeter = 0
                while (q.isNotEmpty()) {
                    val current = q.removeFirst()
                    area++
                    val neighbors = arrangement
                        .getNeighborsIndexedValue(current)
                        .filter { (_, nPlant) -> nPlant == plant }
                        .map { (nPos, _) -> nPos.toCoord(arrangement.width) }

                    perimeter += 4 - neighbors.size

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
                val resurveyed = mutableSetOf<Coord>()
                val sides = surveyed.sumOf { pos ->
                    resurveyed.add(pos)
                    val posSides = Nsew.entries
                        .map { it to pos.move(it) }
                        .filter { (_, adjacent) -> adjacent !in surveyed  }
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

                val bitSet = BitSet(arrangement.size)
                for (pos in surveyed) {
                    bitSet[pos.asIndex(arrangement.width)] = true
                }

                return Region(area, perimeter, sides, bitSet)
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
fun main() = Day.runDay(Y24D12::class)//, test[4])

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
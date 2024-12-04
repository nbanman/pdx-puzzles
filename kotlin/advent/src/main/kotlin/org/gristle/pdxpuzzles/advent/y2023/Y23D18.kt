package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord
import kotlin.math.absoluteValue

class Y23D18(input: String) : Day {
    
    data class Plan(val dir: Nsew, val dist: Int) {
        companion object {
            fun of(dirStr: String, distStr: String) = Plan(Nsew.of(dirStr[0]), distStr.toInt())
            fun of(colorStr: String): Plan {
                val dir = when(colorStr.last()) {
                    '0' -> Nsew.EAST
                    '1' -> Nsew.SOUTH
                    '2' -> Nsew.WEST
                    else -> Nsew.NORTH
                }
                return Plan(dir, colorStr.dropLast(1).toInt(16))
            }
        }
    }
    
    private val plans = input.lines().map { it.split(" (#", " ", ")") }

    override fun part1() = plans.map { (dir, dist) -> Plan.of(dir, dist) }.solve()
    override fun part2() = plans.map { (_, _, color) -> Plan.of(color) }.solve()
    
    fun Iterable<Plan>.solve(): Long {
        
        // build out list of points making the loop
        val moat = runningFold(Coord.ORIGIN) { acc, (dir, dist) -> acc.move(dir, dist) }
        
        // add up the distances in each plan and you get the perimeter of the moat
        val moatPerimeter = sumOf { it.dist }
        
        // this gives the area according to the shoelace formula, but that "rounds down" the area due to the block
        // nature of the AoC coordinate system. We add this back by adding half the perimeter + 1 to the shoelace
        // area.
        val shoelaceArea = moat.zipWithNext { (x1, y1), (x2, y2) -> x1.toLong() * y2 - x2.toLong() * y1 }.sum() / 2
        return moatPerimeter / 2 + 1 + shoelaceArea.absoluteValue
    }
}

fun main() = Day.runDay(Y23D18::class)

//    Class creation: 17ms
//    Part 1: 50746 (3ms)
//    Part 2: 70086216556038 (3ms)
//    Total time: 24ms

@Suppress("unused")
private val sampleInput = listOf(
    """R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
""",
)
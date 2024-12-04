package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.combinatorics.getPairs
import org.gristle.pdxpuzzles.utilities.parsing.getLongs
import org.gristle.pdxpuzzles.utilities.parsing.getNumbers
import kotlin.math.abs

class Y23D24(private val input: String) : Day {
    
    data class Point3d(val x: Double, val y: Double, val z: Double)

    data class Stone(val pos: Point3d, val vel: Point3d) {
        fun intersects(other: Stone, testArea: ClosedFloatingPointRange<Double>): Boolean {
            // get time for original stone at intersection point
            val t = (other.pos.y + (pos.x - other.pos.x) * other.vel.y / other.vel.x - pos.y) / 
                    (vel.y - vel.x * other.vel.y / other.vel.x)
            if (t < 0.0) return false // intersected in past
            
            // get time for comparison stone at intersection point
            val s = (t * vel.x + pos.x - other.pos.x) / other.vel.x
            if (s < 0.0) return false // intersected in past
            
            // get x and y for intersection point and report whether they are both within the test area 
            val x = t * vel.x + pos.x
            val y = t * vel.y + pos.y
            
            return x in testArea && y in testArea
        }
    }

    private val testArea = 200000000000000.0..400000000000000.0
    
    // H/T Brian Norman for walking through the math. 
    // https://github.com/bnorm/advent-of-code/blob/main/year2023/src/aoc/day24/main.kt#L33-L101
    override fun part1(): Int {
        val stones = input
            .getNumbers { toDoubleOrNull() }
            .chunked(3) { (x, y, z) -> Point3d(x, y, z) }
            .chunked(2) { (pos, vel) -> Stone(pos, vel) }
            .toList()
        return stones.getPairs().count { (a, b) -> a.intersects(b, testArea) } 
    } 

    // This was well beyond my abilities, so I had to look at others' solutions. Roman Elizarov's was the easiest for 
    // me to understand, so I replicate it here in commented, non-golfed fashion.
    // His version relies on the fact that on at least one axis the rock has the same position and velocity as one of
    // the hailstones. This way the bruteforce aspect is kept to a minimum.
    // https://github.com/elizarov/AdventOfCode2023/blob/main/src/Day24_2_golf.kt
    override fun part2(): Long {
        val stones: List<List<List<Long>>> = input.getLongs()
            .chunked(3)
            .chunked(2)
            .toList()
        
        // Get the intersection times for all the hailstones once you find the hailstone that matches the pos/velocity
        // of the rock.
        val times: List<Long> = (0..2).firstNotNullOf { axis ->
            stones.firstNotNullOfOrNull { stone ->
                stones.map { other ->
                    // if the two stones have the same velocity on the tested axis, then further consider whether they
                    // have the same position on the tested axis. If so, they meet immediately at time = 0. 
                    // If not, they are parallel, and it's not a valid position.
                    if (stone[1][axis] == other[1][axis]) {
                        if (stone[0][axis] == other[0][axis]) 0L else return@firstNotNullOfOrNull null
                    } else { // if the stones have a different velocity...
                        // time is an integer (why?), so discard any that don't divide cleanly, which means it will
                        // try the next stone.
                        val diffPos = abs(other[0][axis] - stone[0][axis]) // compute difference in initial positions
                        val diffVel = abs(other[1][axis] - stone[1][axis]) // compute difference in velocities
                        if (diffPos % diffVel == 0L) diffPos / diffVel else return@firstNotNullOfOrNull null
                    }
                }
            }
        } 
        
        // get the indices of the first two hailstones that do not travel parallel  
        val (i, j) = times.asSequence().withIndex()
            .filter { (_, time) -> time > 0 }
            .map { it.index }
            .take(2)
            .toList()
        
        // given a stone, an axis, and a time, calculate the position of the stone on that axis at that time
        fun rockPos(i: Int, axis: Int, time: Long): Long = stones[i][0][axis] + stones[i][1][axis] * time
        
        // for each axis, calculate the position at time 0, and sum it.
        return (0..2).sumOf { axis ->
            val iPos = rockPos(i, axis, times[i])
            iPos - (iPos - rockPos(j, axis, times[j])) / (times[i] - times[j]) * times[i]
        }
    }
}

fun main() = Day.runDay(Y23D24::class)

//    Class creation: 3ms
//    Part 1: 14046 (47ms)
//    Part 2: 808107741406756 (6ms)
//    Total time: 57ms

@Suppress("unused")
private val sampleInput = listOf(
    """19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
""",
)
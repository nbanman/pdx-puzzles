package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.isEven
import org.gristle.pdxpuzzles.utilities.math.pow

class Y23D21(private val garden: String) : Day {

    private val width = garden.indexOf('\n')

    // Maps out all the steps that can be traveled in the individual square using BFS.
    private val gardenPath = let {
        val start = garden.indexOf('S')
        
        // BFS queue is an ArrayDeque used as a FIFO queue
        val q = ArrayDeque<Pair<Int, Int>>()
            .apply { add(start to 0) } // load the start value (position plus steps taken)
        
        // Track visited using a BooleanArray, since a very small space.
        val visited = BooleanArray(garden.length)
            .apply { this[start] = true } // load the start value into visited
        
        // BFS finds neighbors and runs until the queue is empty, meaning that no more neighbors are found
        // due to everything already being visited.
        generateSequence { q.removeFirstOrNull() }
            .onEach { (pos, steps) -> // BFS logic is one big side effect filling the queue back up.
                val neighbors = listOf(pos - (width + 1), pos + 1, pos - 1, pos + (width + 1))
                    .filter { neighbor -> 
                        garden.getOrNull(neighbor)?.let { it !in "#\n"} == true && !visited[neighbor] 
                    }.map { neighbor ->
                        visited[neighbor] = true // Side Effect: adds neighbor to visited
                        neighbor to steps + 1 
                    }
                q.addAll(neighbors)
            }.map { (_, steps) -> steps } // we only care about the number of steps taken for each position
            .toList()
    }

    // Elf can always backtrack so can reach any location where the minimum steps taken are 64 and the steps taken is
    // even.
    override fun part1(): Int = gardenPath.count { it <= 64 && it.isEven() }

    // A detailed explanation for how this works is at:
    // https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    // Thanks, Villuna!
    override fun part2(): Long {
        // These are sublists of the original garden path, partitioned into groups of even and odd steps.
        // They serve dual purposes. Their *size* is used for the final calculation. Their *contents* are an 
        // intermediate step for getting the size of the corner pieces.
        val (evenPath, oddPath) = gardenPath.partition(Int::isEven)

        // these are used to smooth out the grid, turning the square into a diamond.
        val evenCorners = evenPath.count { it > 65 }
        val oddCorners = oddPath.count { it > 65 }

        // number of extra squares in a given direction.
        val n = (26501365 - width / 2) / width 

        // Because n is even, we have more odd-parity squares than even-parity squares. See the above link for more.
        return (n + 1).pow(2) * oddPath.size + n.pow(2) * evenPath.size - (n + 1) * oddCorners + n * evenCorners
    }
}

fun main() = Day.runDay(Y23D21::class)

//    Class creation: 27ms
//    Part 1: 3782 (2ms)
//    Part 2: 630661863455116 (5ms)
//    Total time: 35ms


@Suppress("unused")
private val sampleInput = listOf(
    """...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
""", """...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##......#.
.......#...
.##.#.####.
.##..##.##.
...........
""",
)
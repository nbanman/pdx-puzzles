package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y23D17(input: String) : Day {

    private val city = input.toGrid { it.digitToInt() }

    @JvmInline
    private value class State(val value: Int) {
        fun decomposed() = (value shr 1) to (value and 1)

        companion object {
            fun from(pos: Int, dir: Int): State = State((pos shl 1) + dir)
        }
    }

    private fun aStar(l: Int, u: Int): Int {
        val bq = List(100) { ArrayList<State>(300) }
        val cost = List(city.size) { mutableListOf(0, 0) }
        val end = city.lastIndex
        bq[0].add(State(0))
        bq[0].add(State(1))
        var index = 0

        while (true) {
            while (bq[index % 100].isNotEmpty()) {
                val (pos, dir) = bq[index % 100].removeLast().decomposed()
                val steps = cost[pos][dir]

                if (pos == end) return steps

                val x = pos % city.width
                val y = pos / city.width
                
                fun heuristic(x: Int, y: Int, steps: Int) = (steps + city.width - x + city.height - y) % 100 

                var newPos = pos
                var newSteps = steps

                if (dir == 0) {
                    // left
                    for (i in 1..u) {
                        if (i > x) break
                        newPos -= 1
                        newSteps += city[newPos]

                        if (i >= l && (cost[newPos][1] == 0 || newSteps < cost[newPos][1])) {
                            bq[heuristic(x - i, y, newSteps)].add(State.from(newPos, 1))
                            cost[newPos][1] = newSteps
                        }
                    }
                    // right
                    newPos = pos
                    newSteps = steps
                    for (i in 1..u) {
                        if (x + i >= city.width) break
                        newPos += 1
                        newSteps += city[newPos]

                        if (i >= l && (cost[newPos][1] == 0 || newSteps < cost[newPos][1])) {
                            bq[heuristic(x + i, y, newSteps)].add(State.from(newPos, 1))
                            cost[newPos][1] = newSteps
                        }
                    }
                } else {
                    // up 
                    newPos = pos
                    newSteps = steps

                    for (i in 1..u) {
                        if (i > y) break
                        newPos -= city.width
                        newSteps += city[newPos]

                        if (i >= l && (cost[newPos][0] == 0 || newSteps < cost[newPos][0])) {
                            bq[heuristic(x, y - i, newSteps)].add(State.from(newPos, 0))
                            cost[newPos][0] = newSteps
                        }
                    }

                    // down 
                    newPos = pos
                    newSteps = steps

                    for (i in 1..u) {
                        if (y + i >= city.height) break
                        newPos += city.width
                        newSteps += city[newPos]

                        if (i >= l && (cost[newPos][0] == 0 || newSteps < cost[newPos][0])) {
                            bq[heuristic(x, y + i, newSteps)].add(State.from(newPos, 0))
                            cost[newPos][0] = newSteps
                        }
                    }
                }

            }
            index++
        }
    }

    override fun part1(): Int = aStar(1, 3)

    override fun part2(): Int = aStar(4, 10)
}

fun main() = Day.runDay(Y23D17::class)

//    Class creation: 11ms
//    Part 1: 635 (67ms)
//    Part 2: 734 (78ms)
//    Total time: 157ms

@Suppress("unused")
private val sampleInput = listOf(
    """2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
""", """111111111111
999999999991
999999999991
999999999991
999999999991
"""
)
package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import kotlin.math.abs
import kotlin.math.sign

class Y24D21(private val input: String) : Day {

    private val numPad = "789456123#0A".toGrid(3)
    private val dirPad = "#^A<v>".toGrid(3)
    private val actions: Map<Char, Map<Char, List<Char>>> = buildMap<Char, MutableMap<Char, List<Char>>> {
        numPad.withIndex()
            .filter { it.value != '#' }
            .flatMap { a ->
                numPad.withIndex()
                    .filter { it.value != '#' }
                    .map { b -> a to b }
            }.forEach { (a, b) ->
                val (aP, aC) = a
                val (bP, bC) = b
                val aPos = numPad.coordOf(aP)
                val bPos = numPad.coordOf(bP)
                val yDelta = bPos.y - aPos.y
                val ySign = yDelta.sign
                val yLine = let {
                    val yDir: Char? = if (ySign < 0) {
                       '^'
                    } else if (ySign > 0) {
                        'v'
                    } else null

                    if (yDir == null) {
                        emptyList()
                    } else {
                        List(abs(yDelta)) { yDir }
                    }
                }
                val xDelta = bPos.x - aPos.x
                val xSign = xDelta.sign
                val xLine = let {
                    val xDir: Char? = if (xSign < 0) {
                        '<'
                    } else if (xSign > 0) {
                        '>'
                    } else null

                    if (xDir == null) {
                        emptyList()
                    } else {
                        List(abs(xDelta)) { xDir }
                    }
                }

                val actions = if (xSign < 0) {
                    if (aPos.y == 3 && bPos.x == 0) {
                        yLine + xLine
                    } else {
                        xLine + yLine
                    }
                }  else if (aPos.x == 0 && bPos.y == 3) {
                    xLine + yLine
                } else {
                    yLine + xLine
                }

                this.getOrPut(aC) { mutableMapOf() }[bC] = actions + 'A'
            }

        dirPad.withIndex()
            .filter { it.value != '#' }
            .flatMap { a ->
                dirPad.withIndex()
                    .filter { it.value != '#' }
                    .map { b -> a to b }
            }.forEach { (a, b) ->
                val (aP, aC) = a
                val (bP, bC) = b
                val aPos = numPad.coordOf(aP)
                val bPos = numPad.coordOf(bP)
                val yDelta = bPos.y - aPos.y
                val ySign = yDelta.sign
                val yLine = let {
                    val yDir: Char? = if (ySign < 0) {
                        '^'
                    } else if (ySign > 0) {
                        'v'
                    } else null

                    if (yDir == null) {
                        emptyList()
                    } else {
                        List(abs(yDelta)) { yDir }
                    }
                }
                val xDelta = bPos.x - aPos.x
                val xSign = xDelta.sign
                val xLine = let {
                    val xDir: Char? = if (xSign < 0) {
                        '<'
                    } else if (xSign > 0) {
                        '>'
                    } else null

                    if (xDir == null) {
                        emptyList()
                    } else {
                        List(abs(xDelta)) { xDir }
                    }
                }

                val actions = if (xSign < 0) {
                    if (aPos.y == 0 && bPos.x == 0) {
                        yLine + xLine
                    } else {
                        xLine + yLine
                    }
                } else if (aPos.x == 0 && bPos.y == 0) {
                    xLine + yLine
                } else {
                    yLine + xLine
                }

                this.getOrPut(aC) { mutableMapOf() }[bC] = actions + 'A'
            }
    }

    private fun solve(robots: Int): Long = input
        .lines()
        .sumOf { code ->
            val n = code.dropLast(1).toInt()
            val presses = keyPresses(code, robots)
            n * presses
        }

    private data class State(val from: Char, val to: Char, val level: Int)

    private val cache: MutableMap<State, Long> = mutableMapOf<State, Long>().apply {
        for ((a, bActions) in actions) {
            for ((b, actions) in bActions) {
                put(State(a, b, 0), actions.size.toLong())
            }
        }
    }

    private fun keyPresses(code: String, robots: Int): Long = "A$code"
        .zipWithNext()
        .sumOf { (a, b) -> search(State(a, b, robots)) }

    private fun search(state: State): Long = cache.getOrPut(state) {
        val (a, b, level) = state
        val next = (listOf('A') + actions.getValue(a).getValue(b)).zipWithNext()
        next.sumOf { (aa, bb) -> search(State(aa, bb, level - 1)) }
    }

    override fun part1() = solve(2)

    override fun part2() = solve(25)
}

// 379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

fun main() = Day.runDay(Y24D21::class)

@Suppress("unused")
private val test = listOf("""029A
980A
179A
456A
379A
""")

//    [24 Day 21]
//    Class creation: 6ms
//    Part 1: 169390 (2ms)
//    Part 2: 210686850124870 (3ms)
//    Total time: 12ms
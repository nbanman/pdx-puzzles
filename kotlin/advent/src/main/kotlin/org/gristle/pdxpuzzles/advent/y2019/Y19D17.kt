package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.y2019.Intcode.IntCode
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.toMutableGrid
import java.util.*

class Y19D17(input: String) : Day {
    private val initialState = input.split(',').map { it.toLong() }

    fun solve(): Pair<Int, Long> {

        val toDroid: Deque<Long> = LinkedList()
        val toComp: Deque<Long> = LinkedList()
        val intCode = IntCode("A", initialState, null, toComp, toDroid)
        intCode.run()
        val width = toDroid.indexOfFirst { it == 10L }
        val grid = toDroid.filter { it != 10L }.toMutableGrid(width)
        val intersections = grid.mapIndexedNotNull { index, l ->
            if (l != 35L || grid.getNeighbors(index).any { it != 35L }) {
                null
            } else {
                val pos = grid.coordOf(index)
                pos.x * pos.y
            }
        }
        val p1 = intersections.sum()

        // Part 2
        println(grid.representation { it.toInt().toChar() })
        var coord = grid.coordOfElement(94L)
        var dir = Nsew.NORTH
        var counter = 0
        val pathBuilder = StringBuilder()
        while (true) {
            if (cromulent(grid, coord, dir)) {
                counter++
                coord = dir.forward(coord)
            } else if (cromulent(grid, coord, dir.left())) {
                dir = dir.left()
                if (counter != 0) {
                    pathBuilder.append(counter)
                    pathBuilder.append(',')
                    counter = 0
                }
                pathBuilder.append("L,")
            } else if (cromulent(grid, coord, dir.right())) {
                dir = dir.right()
                if (counter != 0) {
                    pathBuilder.append(counter)
                    pathBuilder.append(',')
                    counter = 0
                }
                pathBuilder.append("R,")
            } else {
                pathBuilder.append(counter)
                pathBuilder.append(',')
                break
            }
        }
        val path = pathBuilder.toString()
        val matches = Regex("""[LR],\d+,""").findAll(path).toList()
        val (formSeq, forms) = getCommands(path, matches)
        val video = listOf('n', '\n')
        val commands = (formSeq + forms[0] + forms[1] + forms[2] + video).map { it.code.toLong() }
        toComp.addAll(commands)
        val intCodeB = IntCode("B", listOf(2L) + initialState.drop(1), null, toComp, toDroid)
        toDroid.clear()
        intCodeB.run()
        return p1 to toDroid.last()
    }

    private fun getCommands(path: String, matches: List<MatchResult>): Pair<String, List<String>> {
        val commands = mutableListOf<StringBuilder>()
        splitCommands(path, matches, 0, commands, 3)
        val formSequence = StringBuilder()
        var cursor = 0
        while (cursor < path.length) {
            val position = commands.indexOfFirst { s -> path.drop(cursor).startsWith(s) }
            val c = when (position) {
                0 -> 'A'
                1 -> 'B'
                2 -> 'C'
                else -> throw IllegalStateException()
            }
            formSequence.append(c)
            formSequence.append(',')
            cursor += commands[position].length
        }
        for (command in commands) {
            command.setLength(command.length - 1)
            command.append('\n')
        }
        formSequence.setLength(formSequence.length - 1)
        formSequence.append('\n')
        return formSequence.toString() to commands.map { it.toString() }
    }

    private fun splitCommands(
        path: String,
        matches: List<MatchResult>,
        matchIndex: Int,
        commands: MutableList<StringBuilder>,
        level: Int
    ): Boolean {
        if (level == 0 && matches.size == 0) {
            return true
        }
        var start = matches[matchIndex].range.first
        outer@while (true) {
            for (command in commands) {
                if (path.drop(start).startsWith(command.toString())) {
                    start += command.length
                    continue@outer
                }
            }
            break
        }
        if (level == 0) {
            return start >= path.length
        }
        for ((index, match) in matches
            .drop(matchIndex)
            .withIndex()
            .filter { (_, m) -> m.range.last <= start + 20 }
            .asReversed())
        {
            val sb = StringBuilder()
            sb.append(path.substring(start .. match.range.last))
            commands.add(sb)
            if (splitCommands(path, matches, matchIndex + index + 1, commands, level - 1)) {
                return true
            } else {
                commands.removeLast()
            }
        }
        return false
    }

    private fun cromulent(grid: Grid<Long>, coord: Coord, dir: Nsew): Boolean {
        val prospect = dir.forward(coord)
        return (prospect.x in 0 until grid.width && prospect.y in 0 until grid.height) && grid[prospect] == 35L
    }

    private val solution = solve()

    override fun part1() = solution.first

    override fun part2() = solution.second
}

fun main() = Day.runDay(Y19D17::class)

//    Class creation: 56ms
//    Part 1: 10632 (1ms)
//    Part 2: 1356191 (1ms)
//    Total time: 60ms
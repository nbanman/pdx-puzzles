package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.MutableGrid
import org.gristle.pdxpuzzles.utilities.objects.toMutableGrid

object Y25D10 : Day {
    private val moves = listOf(
        Coord(-2, -1),
        Coord(-1, -2),
        Coord(1, -2),
        Coord(2, -1),
        Coord(2, 1),
        Coord(1, 2),
        Coord(-1, 2),
        Coord(-2, 1),
    )

    private fun <T> dragonMoves(dragon: Int, board: Grid<T>): Sequence<Coord> {
        val dPos = Coord.fromIndex(dragon, board.width)
        return moves.asSequence().mapNotNull { move ->
            val moved = dPos + move
            if (board.validCoord(moved)) {
                moved
            } else {
                null
            }
        }
    }

    override fun part1(input: String): Int = countSheep(input, 4, false)
    override fun part2(input: String): Int = countSheep(input, 20, true)

    enum class Space { SHEEP, HIDEOUT, SHEEPHIDE, EMPTY }

    private fun getDragonAndBoard(input: String): Pair<Coord, MutableGrid<Space>> {
        val board = input.toMutableGrid { c ->
            when (c) {
                'S' -> Space.SHEEP
                '#' -> Space.HIDEOUT
                else -> Space.EMPTY
            }
        }

        val dragonAbsPos = input.indexOfFirst { c -> c == 'D' }
        val lineBreaks = input.take(dragonAbsPos).count { c -> c == '\n' }
        val dragon = Coord.fromIndex(dragonAbsPos - lineBreaks, board.width)

        return dragon to board
    }

    private fun countSheep(input: String, movesAllowed: Int, movingSheep: Boolean): Int {
        val (dragon, board) = getDragonAndBoard(input)
        var eaten = 0
        var moves = 0
        var todo = mutableListOf(dragon)
        var next = mutableListOf<Coord>()
        val visited = mutableSetOf(dragon)

        while (todo.isNotEmpty()) {
            if (movingSheep) {
                visited.clear()
            }
            for (cur in todo) {
                if (board[cur] == Space.SHEEP) {
                    eaten++
                    if (movingSheep) {
                        board[cur] = Space.EMPTY
                    }
                }
                if (moves < movesAllowed) {
                    for (dragonMove in dragonMoves(cur.asIndex(board.width), board)) {
                        if (visited.add(dragonMove)) {
                            next.add(dragonMove)
                        }
                    }
                }
            }
            if (movingSheep && moves != 0) {
                eaten += moveSheep(todo, board)
            }
            moves++
            todo.clear()
            todo = next.also { next = todo }
        }
        return eaten
    }

    private fun moveSheep(dragons: List<Coord>, board: MutableGrid<Space>): Int {
        var eaten = 0
        val lastRow = board.size - board.width

        // move all sheep down one
        for (idx in board.lastIndex downTo 0) {
            val cur = board[idx]
            board[idx] = when (cur) {
                Space.SHEEP -> Space.EMPTY
                Space.HIDEOUT -> Space.HIDEOUT
                Space.SHEEPHIDE -> Space.HIDEOUT
                Space.EMPTY -> Space.EMPTY
            }
            if (idx < lastRow && (cur == Space.SHEEP || cur == Space.SHEEPHIDE)) {
                val belowIdx = idx + board.width
                board[belowIdx] = when (board[belowIdx]) {
                    Space.SHEEP -> throw IllegalStateException("All sheep should have been cleared out!")
                    Space.HIDEOUT -> Space.SHEEPHIDE
                    Space.SHEEPHIDE -> throw IllegalStateException("All sheep should have been cleared out!")
                    Space.EMPTY -> {
                        if (Coord.fromIndex(belowIdx, board.width) in dragons) {
                            eaten++
                            Space.EMPTY
                        } else {
                            Space.SHEEP
                        }
                    }
                }
            }
        }
        return eaten
    }

    override fun part3(input: String): Long {
        val (board, initialState) = getBoardAndState(input)
        val cache = mutableMapOf<Pair<Turn, GameState>, Long>()
        return countVariants(initialState, Turn.Sheep, board, cache)
    }

    data class GameState(val dragon: Int, val sheep: Sheep)
    enum class Hedge { EMPTY, HEDGE, HOME_FREE }
    enum class Turn { Sheep, Dragon }

    @JvmInline
    value class Sheep(val value: UInt = UInt.MAX_VALUE) {
        fun get(x: Int): Int = ((value shr ((6 - x) * 4)) and 0xfu).toInt()
        fun set(pos: Coord): Sheep {
            // clear out area to set
            var r = value and (0xfu shl ((6 - pos.x) * 4)).inv()
            r = r or ((pos.y.toUInt() and 0xfu) shl ((6 - pos.x) * 4))
            return Sheep(r)
        }
        fun remove(x: Int): Sheep = Sheep(value or (0xfu shl ((6 - x) * 4)))
        fun isEmpty(): Boolean = value == UInt.MAX_VALUE
        fun iter(): Sequence<Coord> = generateSequence(7) { it - 1 }
            .takeWhile { it >= 0 }
            .mapNotNull { x ->
                val y = get(x)
                if (y == 0xf) {
                    null
                } else {
                    Coord(x, y)
                }
            }
        override fun toString() = buildString {
            append('[')
            for (x in 6 downTo 0) {
                val y = this@Sheep.get(x)
                if (y == 0xf) {
                    append('X')
                } else {
                    append(y)
                }
            }
            append(']')
        }
    }

    private fun getBoardAndState(input: String): Pair<Grid<Hedge>, GameState> {
        val board = input.toMutableGrid { c ->
            if (c == '#') {
                Hedge.HEDGE
            } else {
                Hedge.EMPTY
            }
        }
        for (x in 0 until board.width) {
            for (y in board.height - 1 downTo 0) {
                if (board[x, y] == Hedge.HEDGE) {
                    board[x, y] = Hedge.HOME_FREE
                } else {
                    break
                }
            }
        }

        val dragonAbsPos = input.indexOfFirst { c -> c == 'D' }
        val lineBreaks = input.take(dragonAbsPos).count { c -> c == '\n' }
        val dragon = dragonAbsPos - lineBreaks

        val sheepInnards = input.take(board.width).fold(Sheep().value) { sheep, c ->
            when (c) {
                'S' -> sheep shl 4
                '.' -> (sheep shl 4) or 0xfu
                else -> sheep
            }
        }

        return board to GameState(dragon, Sheep(sheepInnards))
    }

    private fun countVariants(
        state: GameState,
        turn: Turn,
        board: Grid<Hedge>,
        cache: MutableMap<Pair<Turn, GameState>, Long>
    ): Long = cache.getOrPut(turn to state) {
        // base case 1: all sheep eaten
        if (state.sheep.isEmpty()) {
            return@getOrPut 1L
        }

        if (turn == Turn.Sheep) {
            val sheepDown = state.sheep.iter()
                .map(Coord::south)
                .toList()

            if (sheepDown.size == 1) {
                // base case 2: only one sheep left, next to a home free hedge, so it must escape.
                if ((board.getOrNull(sheepDown[0]) ?: Hedge.HOME_FREE) == Hedge.HOME_FREE) {
                    return@getOrPut 0L
                }
            } else {
                // base case 3: all sheep are on verge of escaping or trapped by dragon and thus
                // one must escape
                val baseCase3 = sheepDown.all { oneDown ->
                    val space = board.getOrNull(oneDown) ?: Hedge.HOME_FREE
                    val matchesDragon = oneDown.asIndex(board.width) == state.dragon
                    space == Hedge.HOME_FREE || (space == Hedge.EMPTY && matchesDragon)
                }
                if (baseCase3) {
                    return@getOrPut 0L
                }
            }
        }


        val next = mutableListOf<GameState>()
        when (turn) {
            Turn.Sheep -> {
                for (oneSheep in state.sheep.iter()) {
                    val oneDown = oneSheep.south()
                    val spaceBelow = board.getOrNull(oneDown) ?: Hedge.HOME_FREE
                    if (spaceBelow != Hedge.HOME_FREE) {
                        if (spaceBelow == Hedge.HEDGE || oneDown.asIndex(board.width) != state.dragon) {
                            val newSheep = state.sheep.set(oneDown)
                            next.add(GameState(state.dragon, newSheep))
                        }
                    }
                }
                if (next.isEmpty()) {
                    next.add(state)
                }
            }
            Turn.Dragon -> {
                for (dragonMove in dragonMoves(state.dragon, board)) {
                    var newSheep = state.sheep

                    // if no hideaway
                    if (board[dragonMove] == Hedge.EMPTY) {
                        if (newSheep.get(dragonMove.x) == dragonMove.y) {
                            newSheep = newSheep.remove(dragonMove.x)
                        }
                    }
                    next.add(GameState(dragonMove.asIndex(board.width), newSheep))
                }
            }
        }

        val nextTurn = when (turn) {
            Turn.Sheep -> Turn.Dragon
            Turn.Dragon -> Turn.Sheep
        }

        next.sumOf { nextState ->
            cache.getOrPut(nextTurn to nextState) { countVariants(nextState, nextTurn, board, cache) }
        }
    }
}

fun main() = Day.runDay(Y25D10::class)

//    Quest 1: 153 (5ms)
//    Quest 2: 1743 (57ms)
//    Quest 3: 3270764079035 (330ms)
//    Total time: 392ms
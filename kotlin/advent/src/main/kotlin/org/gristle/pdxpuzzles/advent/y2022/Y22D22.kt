package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.graph.Graph
import org.gristle.pdxpuzzles.utilities.objects.*
import kotlin.math.sqrt

private typealias DirectionChange = Pair<Nsew, Nsew>
private typealias CoordDirection = Pair<Coord, Nsew>

class Y22D22(input: String) : Day {

    companion object {
        private fun Nsew.facing() = when (this) {
            Nsew.NORTH -> 3
            Nsew.SOUTH -> 1
            Nsew.EAST -> 0
            Nsew.WEST -> 2
        }

        private fun List<List<Char>>.getBounds() = map { line ->
            line.indexOfFirst { it != ' ' }..line.indexOfLast { it != ' ' }
        }
    }

    enum class Command { LEFT, RIGHT, FORWARD }

    private val lines = input.lines()

    private val grove = lines.dropLast(2).let { groveLines ->
        val width = groveLines.maxOf(String::length)
        val height = groveLines.size
        val rows = groveLines.map { it.padEnd(width) } // it will all come to a padEnd
        Grid(width, height) { i ->
            val y = i / width
            val x = i % width
            rows[y][x]
        }
    }

    private val path = buildList {
        Regex("""\d+|[LR]""")
            .findAll(lines.last())
            .map(MatchResult::value)
            .forEach {
                when (it[0]) {
                    'L' -> add(Command.LEFT)
                    'R' -> add(Command.RIGHT)
                    else -> repeat(it.toInt()) { add(Command.FORWARD) }
                }
            }
    }

    private val start = Coord.fromIndex(grove.indexOfFirst { it == '.' }, grove.width)

    /**
     * Main loop is a fold that traverses the path. It delegates the translation of commands to a "move" function
     * which is supplied by the two parts of the puzzle. Return is the scoring for the final position.
     */
    fun solve(move: (CoordDirection) -> CoordDirection): Int {
        var dir = Nsew.EAST
        val end = path.fold(start) { pos, command ->
            if (command == Command.FORWARD) {
                val (prospect, prospectiveDir) = move(CoordDirection(pos, dir))
                if (grove[prospect] == '#') {
                    pos
                } else {
                    dir = prospectiveDir
                    prospect
                }
            } else {
                dir = when (command) {
                    Command.LEFT -> dir.left()
                    Command.RIGHT -> dir.right()
                    else -> throw IllegalArgumentException("Should only be turning.")
                }
                pos
            }
        }
        return 1000 * (end.y + 1) + 4 * (end.x + 1) + dir.facing()
    }

    /**
     * Part 1 traversal wraps the area. It does this by tracking the bounds of each row and column and makes any value
     * higher than the upper bound become the lower bound and vice-versa.
     */
    override fun part1(): Int {
        val rowBounds = grove.rows().getBounds()
        val colBounds = grove.columns().getBounds()

        val move = { (pos, dir): CoordDirection ->
            val prospect = pos.move(dir)
            if (!grove.validCoord(prospect) || grove[prospect] == ' ') {
                if (dir == Nsew.NORTH || dir == Nsew.SOUTH) { // north or south
                    val newY = (if (prospect.y < colBounds[prospect.x].first) Int.MAX_VALUE else Int.MIN_VALUE)
                        .coerceIn(colBounds[prospect.x])
                    prospect.copy(y = newY) to dir
                } else { // east or went
                    val newX = (if (prospect.x < rowBounds[prospect.y].first) Int.MAX_VALUE else Int.MIN_VALUE)
                        .coerceIn(rowBounds[prospect.y])
                    prospect.copy(x = newX) to dir
                }
            } else prospect to dir
        }
        return solve(move)
    }

    /**
     * Part 2 traversal uses cube geometry. It makes a "miniGrove" which just has one pixel per side. Then it runs
     * BFS from every point to every point on the miniGrove. The path of each resulting destination point is a shape.
     * Shapes get folded the same way every time. I keep a list of sets of points along with their corresponding
     * direction changes. To avoid duplication these shapes all start at 0,0 and continue to 0,1. So the shapes found
     * by BFS need to be rotated and flipped to match that orientation, with corresponding changes to the direction
     * changes.
     */
    override fun part2(): Int {
        // get the length of each side
        val sideLength = sqrt((grove.size - grove.count { it == ' ' }) / 6.0).toInt()

        // make mini-grid, one pixel per side
        val miniWidth = grove.width / sideLength
        val miniGrove = buildList {
            for (y in 0 until grove.height / sideLength) for (x in 0 until miniWidth) {
                add(grove[Coord(x * sideLength, y * sideLength)])
            }
        }.toGrid(miniWidth)

        // map of various shapes
        val shapes = listOf(
            listOf(0 to 0, 0 to 1, 1 to 1) to DirectionChange(Nsew.EAST, Nsew.SOUTH),
            listOf(0 to 0, 0 to 1, 0 to 2, 1 to 2) to DirectionChange(Nsew.EAST, Nsew.WEST),
            listOf(0 to 0, 0 to 1, 1 to 1, 2 to 1) to DirectionChange(Nsew.NORTH, Nsew.SOUTH),
            listOf(0 to 0, 0 to 1, 0 to 2, 0 to 3) to DirectionChange(Nsew.SOUTH, Nsew.SOUTH),
            listOf(0 to 0, 0 to 1, 0 to 2, 0 to 3, 1 to 3) to DirectionChange(Nsew.WEST, Nsew.SOUTH),
            listOf(0 to 0, 0 to 1, 1 to 1, 2 to 1, 3 to 1) to DirectionChange(Nsew.EAST, Nsew.EAST),
            listOf(0 to 0, 0 to 1, 0 to 2, 1 to 2, 1 to 3) to DirectionChange(Nsew.NORTH, Nsew.WEST),
            listOf(0 to 0, 0 to 1, 1 to 1, 1 to 2, 1 to 3) to DirectionChange(Nsew.WEST, Nsew.NORTH),
            listOf(0 to 0, 0 to 1, 1 to 1, 1 to 2, 2 to 2) to DirectionChange(Nsew.SOUTH, Nsew.EAST),
            listOf(0 to 0, 0 to 1, 1 to 1, 1 to 2, 2 to 2, 2 to 3) to DirectionChange(Nsew.EAST, Nsew.EAST),
            listOf(0 to 0, 0 to 1, 1 to 1, 1 to 2, 1 to 3, 2 to 3) to DirectionChange(Nsew.NORTH, Nsew.NORTH),
            listOf(0 to 0, 0 to 1, 1 to 1, 2 to 1, 2 to 2, 3 to 2) to DirectionChange(Nsew.WEST, Nsew.WEST),
            listOf(0 to 0, 0 to 1, 0 to 2, 1 to 2, 1 to 3, 1 to 4) to DirectionChange(Nsew.EAST, Nsew.EAST),
        ).associate { (shape, directions) -> shape.map { it.toCoord() }.toSet() to directions }

        // map representing the sides of the cube and how the sides of the cube line up with each other.
        // the key is the position in the miniGrove, the value is another map
        // this second map takes the current direction and provides the destination coordinate in the miniGrove
        // and the new direction.
        val sides: Map<Coord, Map<Nsew, CoordDirection>> = buildMap {
            miniGrove
                .indices
                .filter { miniGrove[it] != ' ' }
                .forEach { index ->
                    val start = Coord.fromIndex(index, miniWidth)
                    val vertices = Graph.bfs(start) { pos ->
                        miniGrove
                            .getNeighborsIndexedValue(pos)
                            .filter { it.value != ' ' }
                            .map { Coord.fromIndex(it.index, miniWidth) }
                    }.drop(1)

                    val end: Map<Nsew, CoordDirection> = buildMap {
                        for (vertex in vertices) {
                            val coords = vertex.path().map { it.id - start }
                            val rotate: (List<Coord>) -> List<Coord> = when (coords[1]) {
                                Coord(0, 1) -> { shape: List<Coord> -> shape }
                                Coord(0, -1) -> { shape: List<Coord> -> shape.rotate180() }
                                Coord(1, 0) -> { shape: List<Coord> -> shape.rotate90() }
                                else -> { shape: List<Coord> -> shape.rotate270() }
                            }
                            var rotated = rotate(coords).toSet()
                            val flipped = rotated.last().x < 0
                            if (flipped) rotated = rotated.flipY().toSet()
                            val directions = shapes[rotated] ?: continue
                            val reverse = when (coords[1]) {
                                Coord(0, 1) -> { dir: Nsew -> dir }
                                Coord(0, -1) -> { dir: Nsew -> dir.flip() }
                                Coord(1, 0) -> { dir: Nsew -> dir.left() }
                                else -> { dir: Nsew -> dir.right() }
                            }

                            fun Nsew.flipY() = when {
                                this == Nsew.NORTH || this == Nsew.SOUTH -> this
                                else -> this.flip()
                            }
                            if (flipped) {
                                put(reverse(directions.first.flipY()), vertex.id to reverse(directions.second.flipY()))
                            } else {
                                put(reverse(directions.first), vertex.id to reverse(directions.second))
                            }
                        }
                    }
                    put(start, end)
                }
        }

        // the function passed to the solve function.
        val move = { (pos, dir): CoordDirection ->
            val prospect = pos.move(dir)
            if (!grove.validCoord(prospect) || grove[prospect] == ' ') {
                val side = Coord(pos.x / sideLength, pos.y / sideLength)
                val (newSide, newDir) = sides.getValue(side).getValue(dir)
                val bigNewSide = Coord(newSide.x * sideLength, newSide.y * sideLength)
                val relativeCoord = Coord(pos.x % sideLength, pos.y % sideLength)
                when (newDir) {
                    Nsew.NORTH -> {
                        val y = bigNewSide.y - 1 + sideLength
                        val x = when (dir) {
                            Nsew.SOUTH -> bigNewSide.x + sideLength - relativeCoord.x - 1
                            Nsew.EAST -> bigNewSide.x + relativeCoord.y
                            Nsew.WEST -> bigNewSide.x + sideLength - relativeCoord.y - 1
                            else -> bigNewSide.x + relativeCoord.x
                        }
                        Coord(x, y) to newDir
                    }

                    Nsew.SOUTH -> {
                        val y = bigNewSide.y
                        val x = when (dir) {
                            Nsew.NORTH -> bigNewSide.x + sideLength - relativeCoord.x - 1
                            Nsew.WEST -> bigNewSide.x + relativeCoord.y
                            Nsew.EAST -> bigNewSide.x + sideLength - relativeCoord.y - 1
                            Nsew.SOUTH -> bigNewSide.x + relativeCoord.x
                        }
                        Coord(x, y) to newDir
                    }

                    Nsew.EAST -> {
                        val x = bigNewSide.x
                        val y = when (dir) {
                            Nsew.NORTH -> bigNewSide.y + relativeCoord.x
                            Nsew.SOUTH -> bigNewSide.y + sideLength - relativeCoord.x - 1
                            Nsew.WEST -> bigNewSide.y + sideLength - relativeCoord.y - 1
                            Nsew.EAST -> bigNewSide.y + relativeCoord.y
                        }

                        Coord(x, y) to newDir
                    }

                    Nsew.WEST -> {
                        val x = bigNewSide.x - 1 + sideLength
                        val y = when (dir) {
                            Nsew.NORTH -> bigNewSide.y + sideLength - relativeCoord.x - 1
                            Nsew.SOUTH -> bigNewSide.y + relativeCoord.x
                            Nsew.EAST -> bigNewSide.y + sideLength - relativeCoord.y - 1
                            Nsew.WEST -> bigNewSide.y + relativeCoord.y
                        }
                        Coord(x, y) to newDir
                    }
                }
            } else prospect to dir
        }
        return solve(move)
    }
}

fun main() = Day.runDay(Y22D22::class)

//    Class creation: 46ms
//    Part 1: 133174 (44ms)
//    Part 2: 15410 (36ms)
//    Total time: 127ms
@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.objects

import org.gristle.pdxpuzzles.utilities.enums.Nsew
import kotlin.math.*

data class Coord(val x: Int, val y: Int) : Comparable<Coord> {
    companion object {
        val ORIGIN = Coord(0, 0)

        val CROSS = listOf(
            Coord(0, -1), Coord(-1, 0), Coord(0, 0), Coord(1, 0), Coord(0, 1),
        )

        /**
         * Positions above, left, below, and right of origin.
         */
        val NSEW = listOf(
            Coord(0, -1), Coord(0, 1), Coord(1, 0), Coord(-1, 0),
        )

        /**
         * All positions adjacent to origin, including diagonals.
         */
        val ALL_ADJACENT = listOf(
            Coord(-1, -1), Coord(0, -1), Coord(1, -1),
            Coord(1, 0), Coord(-1, 0),
            Coord(-1, 1), Coord(0, 1), Coord(1, 1),
        )

        fun fromIndex(n: Int, width: Int) = Coord(n % width, n / width)

        inline fun forRectangle(tl: Coord, br: Coord, action: (coord: Coord) -> Unit) {
            for (y in tl.y..br.y) for (x in tl.x..br.x) action(Coord(x, y))
        }

        inline fun forRectangle(xRange: IntRange, yRange: IntRange, action: (coord: Coord) -> Unit) {
            for (y in yRange) for (x in xRange) action(Coord(x, y))
        }

        inline fun forRectangle(minMaxRange: Pair<IntRange, IntRange>, action: (coord: Coord) -> Unit) =
            forRectangle(minMaxRange.first, minMaxRange.second, action)

        fun rectangleFrom(tl: Coord, br: Coord): List<Coord> = buildList {
            for (y in tl.y..br.y) for (x in tl.x..br.x) add(Coord(x, y))
        }
    }

    fun asIndex(width: Int) = y * width + x
    operator fun plus(coord: Coord) = Coord(x + coord.x, y + coord.y)

    operator fun plus(n: Int) = Coord(x + n, y + n)

    operator fun minus(coord: Coord) = Coord(x - coord.x, y - coord.y)

    operator fun times(other: Coord) = Coord(x * other.x, y * other.y)

    operator fun div(other: Coord) = Coord(x / other.x, y / other.y)

    operator fun rem(other: Coord) = Coord(x % other.x, y % other.y)

    operator fun unaryMinus() = Coord(-x, -y)

    fun mod(other: Coord) = Coord(x.mod(other.x), y.mod(other.y))

    fun max(other: Coord) = Coord(max(x, other.x), max(y, other.y))

    fun min(other: Coord) = Coord(min(x, other.x), min(y, other.y))

    fun area() = x * y

    fun lineTo(other: Coord): List<Coord> {
        val xDelta = (other.x - x).sign
        val yDelta = (other.y - y).sign
        val steps = maxOf((other.x - x).absoluteValue, (other.y - y).absoluteValue)
        return (1..steps).scan(this) { last, _ -> Coord(last.x + xDelta, last.y + yDelta) }
    }

    override fun toString() = "($x, $y)"

    inline fun move(distance: Int, size: Coord, wrapAround: Boolean, operation: () -> Coord): Coord {
        return (this + (Coord(distance, distance) * operation()))
            .let {
                if (size == ORIGIN) it else {
                    if (wrapAround) it.mod(size) else Coord(
                        it.x.coerceIn(0 until size.x),
                        it.y.coerceIn(0 until size.y)
                    )
                }
            }
    }

    fun move(instruction: Char) = when (instruction) {
        'U', 'N', '^' -> north()
        'D', 'S', 'v' -> south()
        'L', 'W', '<' -> west()
        'R', 'E', '>' -> east()
        else -> throw IllegalArgumentException("\'$instruction\' not recognized. Use UDRL, NSEW, or ^<>v.")
    }

    fun north() = copy(y = y - 1)
    fun northwest() = Coord(x - 1, y - 1)
    fun northeast() = Coord(x + 1, y - 1)
    fun south() = copy(y = y + 1)
    fun southwest() = Coord(x - 1, y + 1)
    fun southeast() = Coord(x + 1, y + 1)
    fun west() = copy(x = x - 1)
    fun east() = copy(x = x + 1)

    fun north(distance: Int, size: Coord = Coord(0, 0), wrapAround: Boolean = false) =
        move(distance, size, wrapAround) { Coord(0, -1) }

    fun northwest(distance: Int, size: Coord = Coord(0, 0), wrapAround: Boolean = false) =
        move(distance, size, wrapAround) { Coord(-1, -1) }

    fun northeast(distance: Int, size: Coord = Coord(0, 0), wrapAround: Boolean = false) =
        move(distance, size, wrapAround) { Coord(1, -1) }

    fun south(distance: Int, size: Coord = Coord(0, 0), wrapAround: Boolean = false) =
        move(distance, size, wrapAround) { Coord(0, 1) }

    fun southwest(distance: Int, size: Coord = Coord(0, 0), wrapAround: Boolean = false) =
        move(distance, size, wrapAround) { Coord(-1, 1) }

    fun southeast(distance: Int, size: Coord = Coord(0, 0), wrapAround: Boolean = false) =
        move(distance, size, wrapAround) { Coord(1, 1) }

    fun west(distance: Int, size: Coord = Coord(0, 0), wrapAround: Boolean = false) =
        move(distance, size, wrapAround) { Coord(-1, 0) }

    fun east(distance: Int, size: Coord = Coord(0, 0), wrapAround: Boolean = false) =
        move(distance, size, wrapAround) { Coord(1, 0) }

    fun getNeighbors(includeDiagonals: Boolean = false): List<Coord> {
        return if (includeDiagonals) {
            listOf(northwest(), north(), northeast(), east(), southeast(), south(), southwest(), west())
        } else {
            listOf(north(), west(), east(), south())
        }
    }

    fun getNeighbors(distance: Int, includeSelf: Boolean = true): List<Coord> {
        val tl = Coord(x - distance, y - distance)
        val br = Coord(x + distance, y + distance)
        val neighbors = mutableListOf<Coord>()
        forRectangle(tl, br, neighbors::add)
        if (!includeSelf) neighbors.remove(this)
        return neighbors
    }

    fun manhattanDistance(coord: Coord = ORIGIN): Int = abs(x - coord.x) + abs(y - coord.y)

    fun chebyshevDistance(coord: Coord = ORIGIN): Int = max(abs(x - coord.x), abs(y - coord.y))

    fun move(dir: Nsew, distance: Int = 1) = when (dir) {
        Nsew.NORTH -> north(distance)
        Nsew.SOUTH -> south(distance)
        Nsew.EAST -> east(distance)
        Nsew.WEST -> west(distance)
    }

    fun move(dir: Nsew, distance: Int = 1, size: Coord) = when (dir) {
        Nsew.NORTH -> north(distance, size)
        Nsew.SOUTH -> south(distance, size)
        Nsew.EAST -> east(distance, size)
        Nsew.WEST -> west(distance, size)
    }

    infix fun <T> isWithin(grid: Grid<T>): Boolean = grid.validCoord(this)

    override fun compareTo(other: Coord) = manhattanDistance() - other.manhattanDistance()
}

fun Int.toCoord(width: Int) = Coord(this % width, this / width)

fun Iterable<Coord>.minMaxRanges(): Pair<IntRange, IntRange> {
    var minX = Integer.MAX_VALUE
    var minY = Integer.MAX_VALUE
    var maxX = Int.MIN_VALUE
    var maxY = Int.MIN_VALUE
    forEach { coord ->
        if (coord.x < minX) minX = coord.x
        if (coord.x > maxX) maxX = coord.x
        if (coord.y < minY) minY = coord.y
        if (coord.y > maxY) maxY = coord.y
    }
    return minX..maxX to minY..maxY
}

fun Iterable<Coord>.printToConsole(blankSpace: Char = '.') {
    val (xRange, yRange) = minMaxRanges()
    Coord.forRectangle(xRange, yRange) { coord ->
        if (coord.x == xRange.first && coord.y != yRange.first) print('\n')
        print(if (coord in this) '#' else blankSpace)
    }
    println("\n")
}

fun Iterable<Coord>.toGraphicString(blankSpace: Char = '.'): String {
    val set = toSet()
    val (xRange, yRange) = minMaxRanges()
    return buildString {
        Coord.forRectangle(xRange, yRange) { coord ->
            if (coord.x == xRange.first && coord.y != yRange.first) append('\n')
            append(if (coord in set) '#' else blankSpace)
        }
        append('\n')
    }
}

fun Iterable<Coord>.rotate90(): List<Coord> = map { Coord(-it.y, it.x) }
fun Iterable<Coord>.rotate180(): List<Coord> = map { Coord(-it.x, -it.y) }
fun Iterable<Coord>.rotate270(): List<Coord> = map { Coord(it.y, -it.x) }

// flip along the y-axis (ie, x changes)
fun Iterable<Coord>.flipY(): List<Coord> = map { Coord(-it.x, it.y) }

// flip along the x-axis (ie, y changes)
fun Iterable<Coord>.flipX(): List<Coord> = map { Coord(it.x, -it.y) }

fun Iterable<Coord>.getBounds(padding: Int = 0): Pair<IntRange, IntRange> {
    var xMin = Int.MAX_VALUE
    var xMax = Int.MIN_VALUE
    var yMin = Int.MAX_VALUE
    var yMax = Int.MIN_VALUE
    forEach { coord ->
        if (coord.x < xMin) {
            xMin = coord.x
        } else if (coord.x > xMax) {
            xMax = coord.x
        }
        if (coord.y < yMin) {
            yMin = coord.y
        } else if (coord.y > yMax) {
            yMax = coord.y
        }
    }
    return xMin - padding..xMax + padding to yMin - padding..yMax + padding
}

fun Pair<Int, Int>.toCoord() = Coord(first, second)

operator fun Pair<Coord, Coord>.contains(pos: Coord): Boolean {
    val (tl, br) = this
    return pos.x >= tl.x && pos.x <= br.x && pos.y >= tl.y && pos.y <= br.y
}
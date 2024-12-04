@file:Suppress("unused")
package org.gristle.pdxpuzzles.utilities.objects

import kotlin.math.abs

open class MCoord(coordinates: Iterable<Int>) {

    companion object {
        val ORIGIN = MCoord(listOf(0,0,0,0,0,0,0))
    }

    constructor(vararg coordinates: Int) : this(coordinates.toList())

    val coordinates = coordinates.toList()

    private val dimensions = this.coordinates.size

    private fun changeInOneDimension(dimension: Int, newValue: Int): MCoord {
        return MCoord(coordinates.toMutableList().also { it[dimension] = newValue })
    }

    fun manhattanDistance(other: MCoord = ORIGIN): Int {
        return (coordinates zip other.coordinates).sumOf { (a, b) -> abs(a - b) }
    }

    operator fun get(n: Int) = coordinates[n]

    operator fun unaryMinus() = MCoord(coordinates.map { -it  })

    private fun operate(other: MCoord, identity: Int, operation: (Int, Int) -> Int): MCoord {
        val (larger, smaller) = if (dimensions > other.dimensions) {
            this.coordinates to other.coordinates
        } else {
            other.coordinates to this.coordinates
        }
        val newCoordinates = (this.coordinates zip other.coordinates)
            .map { (a, b) -> operation(a, b) } +
                larger.drop(smaller.size).map { identity * it }
        return MCoord(newCoordinates)

    }

    operator fun plus(other: MCoord) = operate(other, 1) { a, b -> a + b }

    operator fun minus(other: MCoord) = operate(other, -1) { a, b -> a - b }

    operator fun times(other: MCoord) = operate(other, 0) { a, b -> a * b }

    operator fun div(other: MCoord) = operate(other, 0) { a, b -> a / b }

    operator fun rem(other: MCoord) = operate(other, 0) { a, b -> a % b }

    operator fun component1() = coordinates[0]
    operator fun component2() = coordinates[1]
    operator fun component3() = coordinates[2]
    operator fun component4() = coordinates[3]
    operator fun component5() = coordinates[4]

    fun getNeighbors(): List<MCoord> {
        return (0 until dimensions).fold(listOf(this)) { acc, dim ->
            val left = acc.map { it.changeInOneDimension(dim, it.coordinates[dim] - 1) }
            val right = acc.map { it.changeInOneDimension(dim, it.coordinates[dim] + 1) }
            acc + left + right
        }.drop(1)
    }

    override fun toString(): String {
        return buildString {
            append ("MCoord(")
            coordinates.forEachIndexed { index, i -> append("$index=$i, ") }
            dropLast(2)
            append(")")
        }
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as MCoord

        return coordinates == other.coordinates
    }

    override fun hashCode(): Int {
        return coordinates.hashCode()
    }

}

class Xyz(val x: Int, val y: Int, val z: Int) : MCoord(listOf(x, y, z)) {

    companion object {
        val CROSS = listOf(
            Xyz(-1, 0, 0),
            Xyz(1, 0, 0),
            Xyz(0, -1, 0),
            Xyz(0, 1, 0),
            Xyz(0, 0, -1),
            Xyz(0, 0, 1)
        )
    }

    constructor(mCoord: MCoord) : this(mCoord[0], mCoord[1], mCoord[2])

    operator fun plus(other: Xyz) = Xyz(super.plus(other))

    operator fun minus(other: Xyz) = Xyz(super.minus(other))

    operator fun times(other: Xyz) = Xyz(super.times(other))

    operator fun div(other: Xyz) = Xyz(super.div(other))

    operator fun rem(other: Xyz) = Xyz(super.rem(other))

    override fun toString(): String {
        return "Xyz(x=$x, y=$y, z=$z)"
    }
}

fun Iterable<MCoord>.getBounds(padding: Int = 0): List<IntRange> {
    val first = first().coordinates
    val bounds = MutableList(first.size * 2) { first[it / 2] }
    forEach { mCoord ->
        mCoord.coordinates.forEachIndexed { index, i ->
            val lowerIndex = index * 2
            val upperIndex = lowerIndex + 1
            if (i < bounds[lowerIndex]) bounds[lowerIndex] = i
            if (i > bounds[upperIndex]) bounds[upperIndex] = i
        }
    }
    return bounds.chunked(2) { (lower, upper) -> lower - padding..upper + padding }
}
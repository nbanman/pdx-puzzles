@file:Suppress("unused")
package org.gristle.pdxpuzzles.utilities.objects

import org.gristle.pdxpuzzles.utilities.enums.Nsew

class StringGrid(val string: String) {
    val width = string.indexOf('\n')
    val height = (string.length + if (string.last() == '\n') 0 else 1) / (width + 1)
    val xIndices = 0 until width
    val yIndices = 0 until height

    @SuppressWarnings("WeakerAccess")
    fun isValid(index: Int): Boolean = index in string.indices && (index + 1) % (width + 1) != 0

    operator fun get(index: Int): Char = if (isValid(index)) string[index] else
        throw IndexOutOfBoundsException("Index $index out of bounds.")

    operator fun get(pos: Coord): Char {
        val index = pos.y * (width + 1) + pos.x
        return this[index]
    }

    fun getOrNull(index: Int): Char? = if (isValid(index)) string[index] else null

    fun move(index: Int, direction: Nsew, distance: Int = 1): Int {
        val new = moveOrNull(index, direction, distance)
        if (new != null) return new
        throw IndexOutOfBoundsException("Index $index out of bounds.")
    }

    fun moveOrNull(index: Int, direction: Nsew, distance: Int = 1): Int? {
        val movement = when (direction) {
            Nsew.NORTH -> -(width + 1) * distance
            Nsew.SOUTH -> (width + 1) * distance
            Nsew.EAST -> distance
            Nsew.WEST -> -distance
        }
        return (index + movement).let { if (isValid(it)) it else null }
    }

    @Suppress("unused")
    fun getNeighborIndices(index: Int): List<Int> = Nsew.entries.mapNotNull { moveOrNull(index, it) }
}
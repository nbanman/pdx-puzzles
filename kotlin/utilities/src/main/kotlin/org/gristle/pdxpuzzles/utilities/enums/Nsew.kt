@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.enums

import org.gristle.pdxpuzzles.utilities.objects.Coord

enum class Nsew {
    NORTH {
        override fun left() = WEST
        override fun right() = EAST
        override fun flip() = SOUTH

        override fun forward(c: Coord, distance: Int) = Coord(c.x, c.y - distance)
    },

    SOUTH {
        override fun left() = EAST
        override fun right() = WEST
        override fun flip() = NORTH

        override fun forward(c: Coord, distance: Int) = Coord(c.x, c.y + distance)
    },

    EAST {
        override fun left() = NORTH
        override fun right() = SOUTH
        override fun flip() = WEST
        override fun forward(c: Coord, distance: Int) = Coord(c.x + distance, c.y)
    },

    WEST {
        override fun left() = SOUTH
        override fun right() = NORTH
        override fun flip() = EAST
        override fun forward(c: Coord, distance: Int) = Coord(c.x - distance, c.y)
    };

    abstract fun left(): Nsew
    abstract fun right(): Nsew
    abstract fun flip(): Nsew
    fun straight(): Nsew = this
    fun opposite() = left().left()
    abstract fun forward(c: Coord, distance: Int = 1): Coord
    fun forwardInclusive(c: Coord, distance: Int = 1): List<Coord> {
        return (1..distance).map {
            forward(c, it)
        }
    }

    fun multiLeft(times: Int): Nsew = (1..times).fold(this) { acc, _ -> acc.left() }
    fun multiRight(times: Int): Nsew = (1..times).fold(this) { acc, _ -> acc.right() }

    companion object {
        fun of(c: Char): Nsew = when (c) {
            'R', 'E' -> EAST
            'U', 'N' -> NORTH
            'L', 'W' -> WEST
            'D', 'S' -> SOUTH
            else -> throw IllegalArgumentException("Invalid input")
        }
    }
}

fun Nsew.turn(c: Char): Nsew = when (c) {
    'L' -> left()
    'R' -> right()
    'F' -> flip()
    else -> throw IllegalArgumentException("Invalid input")
}
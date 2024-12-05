package org.gristle.pdxpuzzles.utilities.objects

import kotlin.math.abs

data class Hexagon(val q: Int = 0, val r: Int = 0): Comparable<Hexagon> {
    companion object {
        val ORIGIN = Hexagon(0, 0)
    }

    val s = -q - r

    fun hexAt(step: String): Hexagon {
        return when (step) {
            "n" -> copy(r = r - 1)
            "s" -> copy(r = r + 1)
            "nw" -> copy(q = q - 1)
            "ne" -> copy(q = q + 1, r = r - 1)
            "sw" -> copy(q = q - 1, r = r + 1)
            "se" -> copy(q = q + 1)
            else -> throw IllegalArgumentException("$step not a valid movement")
        }
    }

    override fun toString() = "Hex(q=$q, r=$r, s=$s)"

    private fun cubeSubtract(h: Hexagon): Hexagon {
        return Hexagon(q - h.q, r - h.r)
    }

    fun distance(h: Hexagon = ORIGIN): Int {
        val vec = cubeSubtract(h)
        return maxOf(abs(vec.q), abs(vec.r), abs(vec.s))
    }

    fun neighbors() = listOf(
        hexAt("nw"),
        hexAt("n"),
        hexAt("ne"),
        hexAt("se"),
        hexAt("s"),
        hexAt("sw")
    )

    override fun compareTo(other: Hexagon): Int {
        return distance() - other.distance()
    }
}
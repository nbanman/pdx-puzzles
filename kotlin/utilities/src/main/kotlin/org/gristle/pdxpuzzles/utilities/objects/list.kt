@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.objects

fun List<String>.transpose(): List<String> {
    val width = first().length
    require(all { it.length == width }) { "The rows are not of equal size." }
    val height = size
    return List(width) { x ->
        buildString {
            for (y in 0 until height) {
                append(this@transpose[y][x])
            }
        }
    }
}
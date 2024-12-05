@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.objects

@JvmName("transposeWithString")
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

/**
 * Transposes a list of lists to become a <I>list</I> of <I>lists</I>. If the lists are of unequal length it uses the
 * length of the shortest list.
 */
fun <E> List<List<E>>.transpose(): List<List<E>> {
    val width = minOf { it.size }
    val height = size
    return List(width) { w ->
        List(height) { h ->
            this[h][w]
        }
    }
}

/**
 * Shifts the start index of the list by n. The skipped parts get wrapped to the end. Accepts
 * negative numbers to go in the reverse direction.
 */
fun <E> List<E>.shift(n: Int): List<E> {
    val newIndex = n.mod(size)
    return drop(newIndex) + subList(0, newIndex)
}
@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.iteration

/**
 * Returns the minimum of a pair of the same comparable object
 */
fun <E : Comparable<E>> Pair<E, E>.min() = if (first <= second) first else second

/**
 * Returns the maximum of a pair of the same comparable object
 */
fun <E : Comparable<E>> Pair<E, E>.max() = if (first >= second) first else second

/**
 * Returns the minimum and maximum of a pair of the same comparable object in that order
 */
fun <E : Comparable<E>> Pair<E, E>.minMax() = if (first <= second) first to second else second to first

/**
 * Efficiently provides the minimum and maximum of a group of comparable objects
 */
fun <E : Comparable<E>> Iterable<E>.minMax(): Pair<E, E> {
    var min = first()
    var max = min
    drop(1).forEach { if (it < min) min = it else if (it > max) max = it }
    return min to max
}

/**
 * Efficiently provides the minimum and maximum of a group of comparable objects
 */
fun <E : Comparable<E>> minMax(vararg items: E): Pair<E, E> {
    var min = items.first()
    var max = min
    items.drop(1).forEach { if (it < min) min = it else if (it > max) max = it }
    return min to max
}

inline fun <E, R : Comparable<R>> Iterable<E>.minMaxBy(selector: (E) -> R): Pair<E, E> {
    var min = first()
    var minValue = selector(min)
    var max = min
    var maxValue = minValue
    drop(1).forEach {
        val selected = selector(it)
        if (selected < minValue) {
            min = it
            minValue = selected
        } else if (selected > maxValue) {
            max = it
            maxValue = selected
        }
    }
    return min to max
}

inline fun <E, R : Comparable<R>> minMaxBy(vararg items: E, selector: (E) -> R): Pair<E, E> {
    var min = items.first()
    var minValue = selector(min)
    var max = min
    var maxValue = minValue
    items.drop(1).forEach {
        val selected = selector(it)
        if (selected < minValue) {
            min = it
            minValue = selected
        } else if (selected > maxValue) {
            max = it
            maxValue = selected
        }
    }
    return min to max
}
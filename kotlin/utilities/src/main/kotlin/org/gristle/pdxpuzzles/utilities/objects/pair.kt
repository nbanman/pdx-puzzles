@file:Suppress("unused")
package org.gristle.pdxpuzzles.utilities.objects

inline fun <T, R> Pair<T, T>.map(transform: (T) -> R): Pair<R, R> {
    return Pair(transform(first), transform(second))
}

fun <T> Pair<Iterable<T>, Iterable<T>>.zipped(): List<Pair<T, T>> = first zip second
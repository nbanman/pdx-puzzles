@file:Suppress("unused")

package org.gristle.pdxpuzzles.advent.utilities.iteration

import java.util.ArrayList

/**
 * Splits the original collection into n lists, interleaving elements among the lists.
 */
fun <T> Iterable<T>.collate(threads: Int): List<List<T>> {
    val partitions = List(threads) { ArrayList<T>() }
    forEachIndexed { index, element -> partitions[index % threads].add(element) }
    return partitions
}

/**
 * Splits the original String into n Strings, interleaving Chars among the lists.
 */
fun String.collate(threads: Int): List<String> {
    val partitions = List(threads) { StringBuilder() }
    forEachIndexed { index, c -> partitions[index % threads].append(c) }
    return partitions.map(StringBuilder::toString)
}

/**
 * Splits the original sequence into n lists, interleaving elements among the lists.
 */
fun <T> Sequence<T>.collate(threads: Int): Sequence<List<T>> {
    val partitions = List(threads) { ArrayList<T>() }
    forEachIndexed { index, element -> partitions[index % threads].add(element) }
    return partitions.asSequence()
}

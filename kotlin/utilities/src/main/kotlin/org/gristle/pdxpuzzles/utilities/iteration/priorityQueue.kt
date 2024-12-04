@file:Suppress("unused")
package org.gristle.pdxpuzzles.utilities.iteration

import java.util.*
import kotlin.math.min

/**
 * Used in algorithms that have multiple nodes with different weights representing the same location.
 * This way, nodes that have already been visited can be skipped.
 */
inline fun <E> PriorityQueue<E>.pollUntil(predicate: (E) -> Boolean): E? {
    var poll = poll()
    while (poll != null) {
        if (predicate(poll)) return poll
        poll = poll()
    }
    return null
}

/**
 * Calls poll specified number of times, returns results as a list.
 */
fun <E> PriorityQueue<E>.poll(n: Int): List<E> {
    require(n > 0) { "n must be a positive Integer" }
    return MutableList(min(n, size)) { poll() }
}

/**
 * Calls poll until the predicate is no longer met or the PriorityQueue is empty.
 */
inline fun <E> PriorityQueue<E>.pollWhile(predicate: (E) -> Boolean): List<E> {
    val list = mutableListOf<E>()
    while (isNotEmpty()) {
        poll().let { if (predicate(it)) list.add(it) }
    }
    return list
}

fun <E : Comparable<E>> Iterable<E>.toPriorityQueue(): PriorityQueue<E> {
    val pq = if (this is Collection<E>) {
        PriorityQueue<E>(size)
    } else {
        PriorityQueue<E>()
    }
    return pq.also { it.addAll(this) }
}

inline fun <E, R : Comparable<R>> Iterable<E>.toPriorityQueue(transform: (E) -> R): PriorityQueue<R> {
    val pq = if (this is Collection<E>) {
        PriorityQueue<R>(size)
    } else {
        PriorityQueue<R>()
    }
    forEach { e ->
        pq.add(transform(e))
    }
    return pq
}

fun <E : Comparable<E>> Sequence<E>.toPriorityQueue(): PriorityQueue<E> {
    val pq = PriorityQueue<E>()
    return pq.also { it.addAll(this) }
}

fun <E : Comparable<E>> Iterable<E>.toPriorityQueueDescending(): PriorityQueue<E> {
    val pq = if (this is Collection<E>) {
        PriorityQueue(size, compareByDescending { e: E -> e })
    } else {
        PriorityQueue(compareByDescending { e: E -> e })
    }
    return pq.also { it.addAll(this) }
}

inline fun <E, R : Comparable<R>> Iterable<E>.toPriorityQueueDescending(transform: (E) -> R): PriorityQueue<R> {
    val pq = if (this is Collection<E>) {
        PriorityQueue(size, compareByDescending { r: R -> r })
    } else {
        PriorityQueue(compareByDescending { r: R -> r })
    }
    forEach { e ->
        pq.add(transform(e))
    }
    return pq
}

fun <E : Comparable<E>> Sequence<E>.toPriorityQueueDescending(): PriorityQueue<E> {
    val pq = PriorityQueue(compareByDescending { e: E -> e })
    return pq.also { it.addAll(this) }
}

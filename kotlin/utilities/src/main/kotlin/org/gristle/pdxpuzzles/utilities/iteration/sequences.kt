@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.iteration

fun <T> Iterator<T>.nextOrNull(): T? = if (hasNext()) next() else null

fun <T> Sequence<T>.stabilized() = zipWithNext().first { (prev, next) -> prev == next }.first

fun <T> Sequence<IndexedValue<T>>.stabilized() = zipWithNext()
    .first { (prev, next) -> prev.value == next.value }.first

inline fun <T, U> Sequence<T>.stabilized(selector: (T) -> U) =
    zipWithNext().first { (prev, next) -> selector(prev) == selector(next) }.first

inline fun <T, U> Sequence<IndexedValue<T>>.stabilized(selector: (T) -> U) = zipWithNext()
    .first { (prev, next) -> selector(prev.value) == selector(next.value) }.first

inline fun <T> Sequence<T>.takeUntil(crossinline predicate: (T) -> Boolean): Sequence<T> = sequence {
    for (item in this@takeUntil) {
        yield(item)
        if (predicate(item)) break
    }
}

data class Cycle<T>(val element: T, val indexOfFirstInstance: Int, val elements: List<T>)

fun <T> Sequence<T>.findCycle(): Cycle<T>? {
    val cache = LinkedHashMap<T, Int>()
    forEachIndexed { index, element ->
        cache[element]
            ?.let { indexOfFirstInstance -> return Cycle(element, indexOfFirstInstance, cache.keys.toList()) }
            ?: let { cache[element] = index }
    }
    return null
}

fun <T, U> Sequence<T>.findCycle(differentiateBy: T.() -> U): Cycle<T>? {
    val cache = LinkedHashMap<U, Int>()
    val list = mutableListOf<T>()
    forEachIndexed { index, element ->
        val compare = element.differentiateBy()
        cache[compare]
            ?.let { indexOfFirstInstance -> return Cycle(element, indexOfFirstInstance, list) }
            ?: let {
                cache[compare] = index
                list.add(element)
            }
    }
    return null
}
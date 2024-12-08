package org.gristle.pdxpuzzles.utilities.iteration

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.withContext

suspend fun <T, R> Iterable<T>.parMap(transform: (T) -> R): List<R> =
    withContext(Dispatchers.Default) { async { map(transform) }.await() }

suspend fun <T, R> Sequence<T>.parMap(transform: (T) -> R): Sequence<R> =
    withContext(Dispatchers.Default) { async { map(transform) }.await() }

suspend inline fun <T> Iterable<T>.parFilter(crossinline predicate: (T) -> Boolean): List<T> =
    withContext(Dispatchers.Default) { async { filter(predicate) }.await() }

suspend fun <T> Sequence<T>.parFilter(predicate: (T) -> Boolean): Sequence<T> =
    withContext(Dispatchers.Default) { async { filter(predicate) }.await() }

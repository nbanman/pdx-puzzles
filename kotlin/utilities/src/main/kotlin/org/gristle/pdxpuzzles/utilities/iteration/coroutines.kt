@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.iteration

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.withContext

suspend fun <T, R> Iterable<T>.parMap(transform: (T) -> R): List<R> =
    withContext(Dispatchers.Default) { map { async { transform(it) } }.awaitAll() }

@file:Suppress("unused")
package org.gristle.pdxpuzzles.utilities.objects

/**
 * For maps using Int as a value, get value for key or 0 if no matching key found.
 */
fun <K> Map<K, Int>.get0(key: K) = get(key) ?: 0
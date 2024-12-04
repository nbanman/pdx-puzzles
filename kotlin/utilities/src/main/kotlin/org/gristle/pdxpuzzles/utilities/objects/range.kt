@file:Suppress("unused")
package org.gristle.pdxpuzzles.utilities.objects

/**
 * Returns true if an IntRange is a superset of another IntRange; otherwise false.
 */
fun IntRange.containsAll(other: IntRange): Boolean =
    first <= other.first && last >= other.last

/**
 * Returns true if two IntRanges overlap; otherwise false.
 */
fun IntRange.overlaps(other: IntRange): Boolean = if (first <= other.first) {
    last >= other.first
} else {
    other.last >= first
}
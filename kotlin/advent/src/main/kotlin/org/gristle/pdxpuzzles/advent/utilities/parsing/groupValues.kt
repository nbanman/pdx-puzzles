@file:Suppress("UNUSED")

package org.gristle.pdxpuzzles.advent.utilities.parsing

/**
 * Convenience method to obtain the group values of a findall regex search of a string,
 * with a way to map the strings to something else if they are all the same type.
 */
inline fun <R> String.groupValues(pattern: String, transform: (String) -> R): List<List<R>> =
    groupValues(pattern.toRegex(), transform)

/**
 * Convenience method to obtain the group values of a findall regex search of a string,
 * with a way to map the strings to something else if they are all the same type.
 */
inline fun <R> String.groupValues(pattern: Regex, transform: (String) -> R): List<List<R>> {
    return pattern
        .findAll(this)
        .toList()
        .map { it.groupValues.drop(1).map(transform) }
}

fun String.gvs(regex: Regex): Sequence<List<String>> = regex
    .findAll(this)
    .map { it.groupValues.drop(1) }

fun String.gvs(pattern: String): Sequence<List<String>> = Regex(pattern)
    .findAll(this)
    .map { it.groupValues.drop(1) }

fun <R> String.gvs(regex: Regex, transform: (String) -> R): Sequence<List<R>> = regex
    .findAll(this)
    .map { it.groupValues.drop(1).map(transform) }

fun <R> String.gvs(pattern: String, transform: (String) -> R): Sequence<List<R>> = Regex(pattern)
    .findAll(this)
    .map { it.groupValues.drop(1).map(transform) }
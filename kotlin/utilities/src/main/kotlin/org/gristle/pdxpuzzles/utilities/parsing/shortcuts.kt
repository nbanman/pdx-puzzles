@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.parsing

fun String.blankSplit(): List<String> = split("\n\n")

/**
 * Split on blank String
 */
fun Iterable<String>.splitOnBlank(): List<List<String>> {
    val d = mutableListOf<List<String>>()
    var u = mutableListOf<String>()
    forEach { s ->
        if (s == "") {
            d += u
            u = mutableListOf()
        } else {
            u.add(s)
        }
    }
    d += u
    return d
}

/**
 * Returns an Int from a BooleanArray
 */
fun BooleanArray.toInt() = foldIndexed(0) { idx, acc, b ->
    if (b) {
        acc + (1 shl (size - idx - 1))
    } else {
        acc
    }
}

/**
 * Returns a Long from a BooleanArray
 */
fun BooleanArray.toLong() = foldIndexed(0L) { idx, acc, b ->
    if (b) {
        acc + (1L shl (size - idx - 1))
    } else {
        acc
    }
}

/**
 * Returns an Int from a BooleanArray
 */
fun List<Boolean>.toInt() = foldIndexed(0) { idx, acc, b ->
    if (b) {
        acc + (1 shl (size - idx - 1))
    } else {
        acc
    }
}

/**
 * Returns a Long from a BooleanArray
 */
fun List<Boolean>.toLong() = foldIndexed(0L) { idx, acc, b ->
    if (b) {
        acc + (1L shl (size - idx - 1))
    } else {
        acc
    }
}
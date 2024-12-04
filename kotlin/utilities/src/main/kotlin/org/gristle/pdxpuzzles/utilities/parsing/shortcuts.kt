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
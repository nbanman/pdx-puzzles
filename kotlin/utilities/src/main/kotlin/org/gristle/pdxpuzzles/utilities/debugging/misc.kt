@file:Suppress("unused")
package org.gristle.pdxpuzzles.utilities.debugging

/**
 * Prints to console if the Boolean is true. Used for quick and dirty debugging.
 */
fun Boolean.print(s: String) {
    if (this) println(s)
}
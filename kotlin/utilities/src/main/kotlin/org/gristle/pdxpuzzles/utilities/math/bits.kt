@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.math

/**
 * Reverse the bits in an integer
 */
fun Int.reversed(bits: Int): Int {
    var input = this
    var output = 0
    var counter = 0
    while (input > 0) {
        val bit = 1.and(input)
        input = input.shr(1)
        output = output.shl(1) + bit
        counter++
    }
    output = output.shl(bits - counter)
    return output
}
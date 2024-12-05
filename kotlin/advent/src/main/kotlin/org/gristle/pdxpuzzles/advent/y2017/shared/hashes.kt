package org.gristle.pdxpuzzles.advent.y2017.shared

import org.gristle.pdxpuzzles.utilities.objects.shift

fun denseHash(lengths: List<Int>): String {
    val ring = List(256) { it }
    val shiftSum = lengths.sum() * 64
    val skipSum = (lengths.size) * 64
    val p2 = (0 until 64).fold(ring) { acc, i ->
        acc.knotHash(lengths, i * (lengths.size))
    }
    val totalSkips = (1 until skipSum).reduce { acc, i -> acc + i }
    val reshifted = p2.shift(0 - (shiftSum + totalSkips))

    val denseHash = reshifted
        .chunked(16)
        .joinToString("") { chunk ->
            val reduction = chunk.reduce { acc, i -> acc xor i }
            String.format("%02x", reduction)
        }
    return denseHash
}

fun <E> List<E>.knotHash(lengths: List<Int>, skip: Int = 0): List<E> {
    return lengths.foldIndexed(this) { index, accRing, length ->
        knot(accRing, length, skip + index)
    }
}

private fun <E> knot(ring: List<E>, length: Int, skip: Int): List<E> {
    val reversePart = ring.subList(0, length).reversed()
    return (reversePart + ring.drop(reversePart.size)).shift(length + skip)
}

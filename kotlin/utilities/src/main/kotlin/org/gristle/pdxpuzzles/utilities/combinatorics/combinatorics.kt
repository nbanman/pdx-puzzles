@file:Suppress("unused")
package org.gristle.pdxpuzzles.utilities.combinatorics

import org.gristle.pdxpuzzles.utilities.math.factorial

fun <T> Iterable<T>.getPermutations(seed: List<T> = emptyList()): List<List<T>> {
    tailrec fun permute(combos: List<List<T>>, drawPile: List<T>): List<List<T>> {
        return if (combos.first().size == drawPile.size + seed.size) {
            combos
        } else {
            val newCombos = buildList {
                combos.forEach { combo -> addAll((drawPile - combo.toSet()).map { combo + it }) }
            }
            permute(newCombos, drawPile)
        }
    }
    val asList = this.toList() - seed.toSet()
    return permute(asList.map { seed + it }, asList)
}

fun <T> Iterable<T>.getPermutations(seed: List<T> = emptyList(), pruning: (List<T>, T) -> Boolean): List<List<T>> {
    tailrec fun permute(combos: List<List<T>>, drawPile: List<T>): List<List<T>> {
        return if (combos.first().size == drawPile.size + seed.size) {
            combos
        } else {
            val newCombos = buildList {
                combos.forEach { combo ->
                    addAll((drawPile - combo.toSet()).mapNotNull { if (pruning(combo, it)) null else combo + it })
                }
            }
            permute(newCombos, drawPile)
        }
    }

    val asList = this.toList() - seed.toSet()
    return permute(asList.map { if (pruning(emptyList(), it)) seed else seed + it }, asList)
}

fun <E> List<E>.getPairs(): List<Pair<E, E>> {
    val combos = ArrayList<Pair<E, E>>((1 until size).reduce(Int::plus))
    for (i in 0 until lastIndex) for (j in i + 1..lastIndex) combos.add(this[i] to this[j])
    return combos
}

fun <E> List<E>.getPairSequence(): Sequence<Pair<E, E>> = sequence {
    for (i in 0 until lastIndex) for (j in i + 1..lastIndex) {
        yield(this@getPairSequence[i] to this@getPairSequence[j])
    }
}

fun <E> List<E>.getCombinations(r: Int): List<List<E>> {
    val combos = ArrayList<List<E>>((size.factorial() / (r.factorial() * (size - r).factorial())).toInt())
    val working = ArrayList<E>(r)

    fun gP(prev: Int = -1) {
        if (working.size < r) {
            for (i in (prev + 1..size - r + working.size)) {
                working.add(get(i))
                gP(i)
                working.removeLast()
            }
        } else {
            combos.add(working.toList())
        }
    }

    gP()
    return combos
}

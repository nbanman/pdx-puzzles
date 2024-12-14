@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.math

import org.gristle.pdxpuzzles.utilities.iteration.minMax
import kotlin.math.abs
import kotlin.math.pow
import kotlin.math.sqrt

fun List<Pair<Long, Long>>.crt(): Long {
    val n = fold(1L) { acc, (mod, _) -> acc * mod }
    val bigPhase = sumOf { (mod, remainder) ->
        val ni = (n / mod)
        remainder * ni * (ni.toBigInteger().modInverse(remainder.toBigInteger())).toLong()
    }
    return bigPhase % n
}

fun Int.isEven() = this and 1 == 0
fun Int.isOdd() = this and 1 == 1

fun Int.isPrime(): Boolean = when {
    this <= 1 -> false
    this <= 3 -> true
    this.isEven() || this % 3 == 0 -> false
    else -> {
        val limit = sqrt(this.toFloat()).toInt()
        generateSequence(5) { it + 6 }
            .takeWhile { it <= limit }
            .none { this % it == 0 || this % (it + 2) == 0 }
    }
}

/**
 * pow implementation for Int
 */
fun Int.pow(n: Int): Long = if (n >= 0) {
    (1..n).fold(1L) { acc, _ -> acc * this }
} else {
    this.toFloat().pow(n).toLong()
}

fun Int.summation(): Int = (1..this).sum()

fun Int.factorial(): Long = (1..this).fold(1L, Long::times)

fun quadraticFormula(a: Long, b: Long, c: Long): Pair<Double, Double> {
    val determinant = sqrt(b.toDouble().pow(2) - 4 * a * c)
    val root1 = (-b + determinant) / (2 * a)
    val root2 = (-b - determinant) / (2 * a)
    return root1 to root2
}

fun gcd(unsortedInts: List<Int>): Int {
    require(unsortedInts.size >= 2) { "There must be at least two numbers" }
    return gcd(unsortedInts[0], unsortedInts[1], *unsortedInts.drop(2).toIntArray())
}

fun gcd(unsortedLongs: List<Long>): Long {
    require(unsortedLongs.size >= 2) { "There must be at least two numbers" }
    return gcd(unsortedLongs[0], unsortedLongs[1], *unsortedLongs.drop(2).toLongArray())
}

fun gcd(a: Int, b: Int, vararg n: Int): Int {
    val numbers: List<Int> = ArrayList<Int>(n.size + 2).apply {
        add(abs(a))
        add(abs(b))
        addAll(n.map { abs(it) })
    }
    require(numbers.any { it != 0 }) { "At least one number must not be zero" }

    tailrec fun gcd(a: Int, b: Int): Int = if (a == 0) b else gcd((b % a), a)

    return numbers.reduce { acc, i ->
        val (smaller, larger) = minMax(acc, i)
        val nextAcc = gcd(smaller, larger)
        if (nextAcc == 1) return nextAcc
        nextAcc
    }
}

fun gcd(a: Long, b: Long, vararg n: Long): Long {
    val numbers: List<Long> = ArrayList<Long>(n.size + 2).apply {
        add(abs(a))
        add(abs(b))
        addAll(n.map { abs(it) })
    }
    require(numbers.any { it != 0L }) { "At least one number must not be zero" }

    tailrec fun gcd(a: Long, b: Long): Long = if (a == 0L) b else gcd((b % a), a)

    return numbers.reduce { acc, i ->
        val (smaller, larger) = minMax(acc, i)
        val nextAcc = gcd(smaller, larger)
        if (nextAcc == 1L) return nextAcc
        nextAcc
    }
}

fun lcm(a: Long, b: Long, vararg n: Long): Long {
    val numbers: List<Long> = ArrayList<Long>(n.size + 2).apply {
        add(abs(a))
        add(abs(b))
        addAll(n.map { abs(it) })
    }
    require(numbers.any { it != 0L }) { "At least one number must not be zero" }

    fun lcm(a: Long, b: Long): Long = abs(a * b) / gcd(a, b)

    return numbers.reduce { acc, i -> lcm(acc, i)
    }
}

fun lcm(longs: List<Long>): Long {
    require(longs.size >= 2) { "There must be at least two numbers" }
    return lcm(longs[0], longs[1], *longs.drop(2).toLongArray())
}

fun Iterable<Long>.lcm(): Long {
    val longList = this.toList()
    return lcm(longList)
}
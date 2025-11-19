package org.gristle.pdxpuzzles.everybodycodes.y2025

import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.runBlocking
import org.gristle.pdxpuzzles.everybodycodes.utilities.Day

object Y25D09 : Day {
    data class UnionFind(val parent: MutableList<Int>, val size: MutableList<Int>) {
        companion object {
            fun new(n: Int) = UnionFind(
                MutableList(n) { it },
                MutableList(n) { 1 }
            )
        }

        fun find(x: Int): Int {
            if (parent[x] != x) {
                parent[x] = find(parent[x])
            }
            return parent[x]
        }

        fun union(x: Int, y: Int): Boolean {
            val x = find(x)
            val y = find(y)

            if (x == y) return false

            if (size[x] >= size[y]) {
                parent[y] = x
                size[x] += size[y]
            } else {
                parent[x] = y
                size[y] += size[x]
            }

            return true
        }

        fun update() {
            for (i in 0 until parent.size) {
                find(i)
            }
        }
    }

    @OptIn(ExperimentalUnsignedTypes::class)
    @JvmInline
    value class Dna(private val inner: ULongArray) {
        companion object {
            fun from(s: String): Dna {
                val (_, chain) = s.split(':')
                val chainChunks = chain.chunked(16)
                val inner = ULongArray(8) { i ->
                    chainChunks[i].fold(0uL) { acc, c ->
                        val next = when (c) {
                            'A' -> 1uL
                            'T' -> 2uL
                            'C' -> 4uL
                            'G' -> 8uL
                            else -> throw IllegalStateException("$c not a valid symbol")
                        }
                        (acc shl 4) or next
                    }
                }
                return Dna(inner)
            }
        }

        private inline fun expand(other: Dna, op: (ULong, ULong) -> ULong): Dna {
            val other = other.inner
            val new = ULongArray(8) { i ->
                op(inner[i], other[i])
            }
            return Dna(new)
        }

        infix fun and(other: Dna): Dna = expand(other, ULong::and)
        infix fun or(other: Dna): Dna = expand(other, ULong::or)

        fun contentEquals(other: Dna): Boolean = inner.contentEquals(other.inner)

        fun similarity(other: Dna): Int {
            val meld = this and other
            return meld.inner.sumOf { it.countOneBits() }
        }

        override fun toString() = buildString {
            append('[')
            inner.joinTo(this, ", ", transform = ULong::toString)
            append(']')
        }
    }

    private suspend fun getDna(input: String): List<Dna> = coroutineScope {
        input
            .lines()
            .map { line -> async { Dna.from(line) } }
            .awaitAll()
            .toList()
    }

    private fun getFamilies(
        dna: List<Dna>,
        child: Int,
        childDna: Dna
    ): Pair<Int, Int>? = (0 until dna.size)
        .asSequence()
        .filter { it != child }
        .map { p1 ->
            val p1Dna = dna[p1]
            p1 to childDna.similarity(p1Dna)
        }.filter { (_, p1Sim) -> p1Sim > 60 }
        .flatMap { (p1) ->
            (0 until dna.size)
                .asSequence()
                .filter { p2 -> p2 != child && p2 != p1 }
                .map { p2 -> p1 to p2 }
        }.find { (p1, p2) ->
            val p1Dna = dna[p1]
            val p2Dna = dna[p2]
            val parentDna = p1Dna or p2Dna
            childDna.contentEquals(childDna and parentDna)
        }

    override fun part1(input: String): Int = runBlocking {
        val dna = getDna(input)
        (0 until dna.size)
            .asSequence()
            .mapNotNull { i ->
                val child = dna[i]
                val p1 = dna[(i + 1) % dna.size]
                val p2 = dna[(i + 2) % dna.size]
                if (child.contentEquals(child and (p1 or p2))) {
                    child.similarity(p1) * child.similarity(p2)
                } else {
                    null
                }
            }.first()
    }

    override fun part2(input: String): Int = runBlocking {
        val dna = getDna(input)
        coroutineScope {
            (0 until dna.size).toList()
                .map { child ->
                    async {
                        val childDna = dna[child]
                        getFamilies(dna, child, childDna)
                            ?.let { (p1, p2) ->
                                childDna.similarity(dna[p1]) * childDna.similarity(dna[p2])
                            } ?: 0
                    }
                }
                .awaitAll()
                .sum()
        }
    }
    override fun part3(input: String): Int = runBlocking {
        val dna = getDna(input)
        val nuclearFamilies = coroutineScope {
            (0 until dna.size).toList()
                .map { child ->
                    async {
                        val childDna = dna[child]
                        getFamilies(dna, child, childDna)
                            ?.let { (p1, p2) ->
                                Triple(child, p1, p2)
                            }
                    }
                }
                .awaitAll()
                .filterNotNull()
        }
        val tree = UnionFind.new(500)
        for ((a, b, c) in nuclearFamilies) {
            tree.union(a, b)
            tree.union(a, c)
        }
        val largestGroup = tree
            .size
            .withIndex()
            .maxBy { it.value }
            .index

        tree.update()
        tree.parent
            .withIndex()
            .filter { it.value == largestGroup }
            .sumOf { it.index + 1 }
    }
}

fun main() = Day.runDay(Y25D09::class)

//    Quest 1: 6478 (23ms)
//    Quest 2: 316671 (20ms)
//    Quest 3: 40905 (55ms)
//    Total time: 99ms
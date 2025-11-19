package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit

object Y25D07 : Day {
    private fun parse(input: String): Pair<List<String>, Map<Char, List<Char>>> {
        val (nameStr, pathStr) = input.blankSplit()
        val names = nameStr.split(',')
        val paths = pathStr
            .lineSequence()
            .associate { line ->
                val iter = line.filter(Char::isLetter).asSequence()
                val k = iter.first()
                val v = iter.drop(1).toList()
                k to v
            }
        return names to paths
    }

    override fun part1(input: String): String {
        val (names, paths) = parse(input)
        val initial = paths.keys.filter(Char::isUpperCase)
        outer@for(name in names) {
            var available = initial
            for (c in name) {
                if (c in available) {
                    val next = paths[c]
                    if (next != null) {
                        available = next
                    } else {
                        if (c == name.last()) {
                            return name
                        }
                        continue@outer
                    }
                } else {
                    continue@outer
                }
            }
        }
        throw IllegalStateException("unreachable")
    }

    override fun part2(input: String): Int {
        val (names, paths) = parse(input)
        val initial = paths.keys.filter(Char::isUpperCase)
        var sum = 0

        outer@for((idx, name) in names.withIndex()) {
            var available = initial
            for (c in name) {
                if (c in available) {
                    val next = paths[c]
                    if (next != null) {
                        available = next
                    } else {
                        if (c == name.last()) {
                            sum += idx + 1
                            continue
                        }
                        continue@outer
                    }
                } else {
                    continue@outer
                }
            }
            sum += idx + 1
        }
        return sum
    }

    override fun part3(input: String): Int {
        val (names_immut, paths) = parse(input)
        val names = names_immut.toMutableList()
        var i = 0
        while (true) {
            if (i == names.size) break
            val name = names[i]
            for (ii in names.lastIndex downTo 0) {
                if (i != ii && name in names[ii]) {
                    names.removeAt(ii)
                }
            }
            i++
        }
        val initial = paths.keys.filter(Char::isUpperCase)
        var sum = 0
        val cache = IntArray(512) { -1 }

        outer@for(name in names) {
            var available = initial
            for (c in name) {
                if (c in available) {
                    val next = paths[c]
                    if (next != null) {
                        available = next
                    } else {
                        continue@outer
                    }
                } else {
                    continue@outer
                }
            }
            val len = name.length
            val last = name.last()
            val hash = last.hash(len)
            sum += cache[hash].let {
                if (it >= 0) {
                    it
                } else {
                    countNames(last, len, hash, paths, cache)
                }
            }
        }
        return sum
    }

    fun countNames(
        c: Char,
        depth: Int,
        hash: Int,
        paths: Map<Char, List<Char>>,
        cache: IntArray
    ): Int {
        // base case 1: Max depth reached
        if (depth == 11) {
            cache[hash] = 1
            return 1
        }

        var nameCount = if (depth >= 7) 1 else 0

        // base case 2: no children remaining
        val next = paths[c]
        if (next == null) {
            cache[hash] = nameCount
            return nameCount
        }

        // otherwise
        for (nc in next) {
            val nHash = nc.hash(depth + 1)
            nameCount += cache[nHash].let {
                if (it >= 0) {
                    it
                } else {
                    countNames(nc, depth + 1, nHash, paths, cache)
                }
            }
        }
        cache[hash] = nameCount
        return nameCount
    }

    fun Char.hash(depth: Int): Int = ((this - 'a') shl 4) or depth
}

fun main() = Day.runDay(Y25D07::class)

//    [25 Day 7]
//    Quest 1: Ulendris (1ms)
//    Quest 2: 2529 (3ms)
//    Quest 3: 1945135 (0ms)
//    Total time: 5ms
package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import kotlin.collections.forEach

object Y24D06 : Day {
    private fun solve(input: String, truncate: Boolean): String {
        val branches = getBranches(input)
        val paths = getPaths(branches, truncate)
        return getStrongest(paths)
    }

    private fun getBranches(input: String): Map<String, List<String>> = input.lines()
        .filter { line -> !line.startsWith("ANT") && !line.startsWith("BUG") }.associate { line ->
            val tokens = line.split(':', ',')
            tokens[0] to tokens.drop(1)
        }

    private fun getPaths(branches: Map<String, List<String>>, truncate: Boolean) = buildList<String> {
        val q = mutableListOf(listOf("RR"))
        while (q.isNotEmpty()) {
            val path = q.removeLast()
            val current = path.last()
            if (current == "@") {
                val pathName = buildString {
                    if (truncate) {
                        for (s in path) append(s.first())
                    } else {
                        append(path.joinToString(""))
                    }
                }
                add(pathName)
            } else {
                branches[current]?.forEach { child ->
                    q.add(path + child)
                }
            }
        }
    }

    private fun getStrongest(paths: List<String>): String = paths
        .groupBy { it.length }
        .values
        .find { it.size == 1 }
        ?.first()
        ?: throw IllegalArgumentException("All values have a matching length value.")

    override fun part1(input: String) = solve(input, false)
    override fun part2(input: String) = solve(input, false)
    override fun part3(input: String) = solve(input, true)
}

fun main() = Day.runDay(Y24D06::class)

//    Quest 1: RRHKWPCNPBDF@ (4ms)
//    Quest 2: RRSJZVFDKRVJWHTWHJMRTTFTPBMRJHGXFSFHCB@ (9ms)
//    Quest 3: RFFSZQMJWLLN@ (19ms)
//    Total time: 32ms
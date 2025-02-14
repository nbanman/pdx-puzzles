package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import kotlin.math.min

object Y24D02 : Day {
    private fun parseWords(wordsStr: String): List<String> = wordsStr
        .dropWhile { it != ':' }
        .drop(1)
        .split(',')

    private fun Grid<Char>.east(pos: Coord, length: Int): Pair<List<Coord>, List<Char>> {
        val row = row(pos.y)
        val indices = List(length) { i ->
            (pos.x + i) % width
        }
        return indices.map { Coord(it, pos.y) } to indices.map { row[it] }
    }

    private fun Grid<Char>.south(pos: Coord, length: Int): List<Char> =
        (pos.y until min(height, pos.y + length))
            .map { y -> this[Coord(pos.x, y)] }

    override fun part1(input: String): Int {
        val (words, code) = input.split("\n\n")
        return parseWords(words).sumOf { word -> code.windowed(word.length).count { it == word } }
    }
    override fun part2(input: String): Int {
        val (wordsStr, code) = input.split("\n\n")
        val words = parseWords(wordsStr).let { half ->
            (half + half.map { it.reversed() })
                .toSet()
        }
        val wordLengths = words.map { it.length }.distinct().sortedDescending()
        val symbols = BooleanArray(code.length)

        for ((index, c) in code.withIndex()) {
            if (c in "\n ") continue
            var snippet = code
                .substring(index, min(code.length, index + wordLengths.first()))
                .takeWhile { it !in "\n " }
            for (length in wordLengths) {
                if (snippet.length < length) continue
                snippet = snippet.take(length)
                if (snippet in words) {
                    for (i in index until index + length) symbols[i] = true
                    break
                }
            }
        }
        return symbols.count { it }
    }

    override fun part3(input: String): Int {
        val (wordsStr, code) = input.split("\n\n")
        val words = parseWords(wordsStr)
            .let { half ->
                buildSet<List<Char>> {
                    val halfList = half.map { it.toList() }
                    addAll(halfList)
                    addAll(halfList.map { it.reversed() })
                }
            }
        val wordLengths = words.map { it.size }.distinct().sortedDescending()

        val armor = code.toGrid()
        val symbols = mutableSetOf<Coord>()
        for (pos in armor.coords()) {
            var (eastIndex, east) = armor.east(pos, wordLengths.first())
            for (length in wordLengths) {
                if (length != east.size) east = east.take(length)
                if (east in words) {
                    symbols.addAll(eastIndex.take(length))
                    break
                }
            }

            var south = armor.south(pos, wordLengths.first())
            for (length in wordLengths) {
                if (south.size < length) continue
                if (length < south.size) south = south.take(length)
                if (south in words) {
                    symbols.addAll(pos.lineTo(pos.copy(y = pos.y + length - 1)))
                    break
                }
            }
        }
        return symbols.size
    }
}

fun main() = Day.runDay(Y24D02::class)

//    Quest 1: 30 (2ms)
//    Quest 1: 4992 (15ms)
//    Quest 1: 11816 (57ms)
//    Total time: 75ms
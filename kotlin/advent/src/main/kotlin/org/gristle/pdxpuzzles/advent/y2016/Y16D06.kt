package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.transpose

class Y16D06(input: String) : Day {
    private val columns: List<Map<Char, Int>> = input
        .lines()
        .map(String::toList)
        .transpose()
        .map { value -> value.groupingBy { it }.eachCount() }
    override fun part1() = columns.map { column -> column.maxBy { it.value }.key }.joinToString("")
    override fun part2() = columns.map { column -> column.minBy { it.value }.key }.joinToString("")
}

fun main() = Day.runDay(Y16D06::class)

//    Class creation: 25ms
//    Part 1: asvcbhvg (0ms)
//    Part 2: odqnikqv (0ms)
//    Total time: 25ms

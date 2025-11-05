package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getLongList

object Y25D02 : Day {
    private data class Point(val x: Long, val y: Long) {
        operator fun plus(other: Point) = Point(x + other.x, y + other.y)
        operator fun div(other: Long) = Point(x / other, y / other)
    }

    private fun parse(input: String): Point {
        val (x, y) = input.getLongList()
        return Point(x, y)
    }

    private fun square(a: Point): Point {
        val x = a.x * a.x - a.y * a.y
        val y = a.x * a.y + a.x * a.y
        return Point(x, y)
    }

    private fun engravablePoint(cycles: Int, point: Point, divisor: Long): Point? {
        val range = -1_000_000..1_000_000
        var acc = Point(0, 0)
        repeat(cycles) {
            acc = point + square(acc) / divisor
            if (acc.x !in range || acc.y !in range) {
                return null
            }
        }
        return acc
    }

    private fun solve(input: String, step: Long): Int {
        val tl = parse(input)
        val br = tl + Point(1000, 1000)
        var engravedPoints = 0
        for (y in tl.y..br.y step step) {
            for (x in tl.x..br.x step step) {
                val point = Point(x, y)
                if (engravablePoint(100, point, 100_000) != null) engravedPoints++
            }
        }
        return engravedPoints
    }

    override fun part1(input: String): String {
        val point = parse(input)
        return engravablePoint(3, point, 10)
            ?.let { "[${it.x},${it.y}]" }
            ?: throw IllegalStateException()
    }
    override fun part2(input: String): Int = solve(input, 10)
    override fun part3(input: String): Int = solve(input, 1)
}

fun main() = Day.runDay(Y25D02::class)

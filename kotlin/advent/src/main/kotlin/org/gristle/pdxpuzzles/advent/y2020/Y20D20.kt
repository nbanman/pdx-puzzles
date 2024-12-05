package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.math.reversed
import org.gristle.pdxpuzzles.utilities.objects.*
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import kotlin.math.sqrt

class Y20D20(input: String) : Day {

    data class TileOrient(val tile: Tile, val nsew: Nsew = Nsew.NORTH, val flipped: Boolean = false) {

        val borderless = tile
            .image
            .subGrid(Coord(1, 1), tile.length - 2, tile.length - 2)
            .let { if (flipped) it.flipY() else it }
            .let { (0 until nsew.orientValue()).fold(it) { acc, _ -> acc.rotate90() } }

        private fun Nsew.orientValue() = when (this) {
            Nsew.NORTH -> 0
            Nsew.EAST -> 1
            Nsew.SOUTH -> 2
            Nsew.WEST -> 3
        }

        fun matchValue(dir: Nsew): Int {
            val valueIndex = Math.floorMod(dir.orientValue() - nsew.orientValue(), 4) + if (flipped) 4 else 0
            return tile.values[valueIndex]
        }

        fun reorient(dir: Nsew): TileOrient {
            val newOrient = (1..Math.floorMod(dir.orientValue() - nsew.orientValue(), 4))
                .fold(Nsew.NORTH) { acc, _ -> acc.right() }
            return this.copy(nsew = newOrient)
        }

        override fun toString(): String {
            return "TileOrient(nsew=$nsew, flipped=$flipped, tile=$tile, borderless=$borderless)"
        }

    }

    data class Tile(val id: Int, val image: Grid<Char>) {
        companion object {
            val lookup = mutableMapOf<Int, MutableList<TileOrient>>()
        }

        val length = image.width
        val values = listOf(
            List(length) { i -> image[i] },
            List(image.height) { i -> image[i * length + length - 1] },
            List(length) { i -> image[image.size - 1 - i] },
            List(image.height) { i -> image[(image.size - length) - (i * length)] }
        ).map {
            it.indices.fold(0) { acc, place ->
                acc + if (it[place] == '.') 0 else 1.shl(it.size - 1 - place)
            }
        }.let {
            it + listOf(
                it[0].reversed(10),
                it[3].reversed(10),
                it[2].reversed(10),
                it[1].reversed(10)
            )
        }

        val matchingTiles: List<TileOrient> by lazy {
            values.flatMap { lookup[it] ?: emptyList() }.distinct().filter { it.tile != this }
        }

        init {
            values.forEachIndexed { index, tileValue ->
                val pot = lookup.getOrPut(tileValue) { mutableListOf() }
                val to = TileOrient(
                    this,
                    (0 until index).fold(Nsew.NORTH) { acc, _ -> acc.right() },
                    index >= 4
                )
                pot.add(to)
            }
        }

        override fun toString(): String {
            return "Tile(id=$id, values=$values, image=$image)"
        }

    }

    private val tiles = input
        .split("\n\n")
        .map { s ->
            val id = s.getInts().first()
            val image = s
                .substring(s.indexOf('\n') + 1)
                .toMutableGrid()
            Tile(id, image)
        }

    override fun part1(): Long {
        return tiles
            .filter { it.matchingTiles.size == 4 }
            .fold(1L) { acc, tile -> acc * tile.id }
    }

    override fun part2(): Int {
        // form grid
        val width = sqrt(tiles.size.toFloat()).toInt()
        val matchFour = tiles
            .find { it.matchingTiles.size == 4 }
            ?: throw IllegalStateException("No tiles matching four others exist")
        var firstTile = TileOrient(matchFour, Nsew.NORTH, false)
        while (Tile.lookup[firstTile.matchValue(Nsew.SOUTH).reversed(10)]?.size != 2 ||
            Tile.lookup[firstTile.matchValue(Nsew.EAST).reversed(10)]?.size != 2
        ) {
            firstTile = firstTile.copy(nsew = firstTile.nsew.right())
        }
        val stitched = mutableListOf(firstTile)
        for (index in 1..tiles.lastIndex) {
            val nextTile = if (index % width == 0) {
                val referenceTile = stitched[index - width]
                Tile.lookup.getValue(referenceTile.matchValue(Nsew.SOUTH).reversed(10))
                    .first { it.tile != referenceTile.tile }
                    .reorient(Nsew.NORTH)
            } else {
                val referenceTile = stitched[index - 1]
                Tile.lookup.getValue(referenceTile.matchValue(Nsew.EAST).reversed(10))
                    .first { it.tile != referenceTile.tile }
                    .reorient(Nsew.WEST)
            }
            stitched.add(nextTile)
        }

        // stitch image together
        var wholeImage = stitched
            .map { it.borderless }
            .toGrid(width)
            .rows()
            .map { row ->
                row.reduce { acc, tileImage -> acc.addRight(tileImage) }
            }.reduce { acc, chars -> acc.addDown(chars) }

        val hashMarks = wholeImage.count { it == '#' }
        val spacing = wholeImage.width - 18

        @Suppress("RegExpSimplifiable")
        val pattern =
            Regex("""(?=(#[#.\n]{$spacing}#[#.]{4}##[#.]{4}##[#.]{4}###[#.\n]{$spacing}#[#.]{2}#[#.]{2}#[#.]{2}#[#.]{2}#[#.]{2}#))""")
        for (i in 1..8) {
            val matches = pattern.findAll(wholeImage.representation { it }).toList().size
            if (matches > 0) return hashMarks - matches * 15
            wholeImage = wholeImage.rotate90()
            if (i == 4) wholeImage = wholeImage.flipY()
        }
        return -1
    }
}

fun main() = Day.runDay(Y20D20::class)

//    Class creation: 51ms
//    Part 1: 19955159604613 (7ms)
//    Part 2: 1639 (64ms)
//    Total time: 123ms
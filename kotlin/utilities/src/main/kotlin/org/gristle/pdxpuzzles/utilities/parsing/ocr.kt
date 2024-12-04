@file:Suppress("unused")
package org.gristle.pdxpuzzles.utilities.parsing

import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.objects.rep
import kotlin.text.indexOf

/**
 * Converts an Advent of Code ANSI graphical representation of a string of letters and converts it into a string.
 * @emptySpace (AoC default: '.') The character used to designate empty space in the image.
 */
fun String.ocr(emptySpace: Char = '.'): String = trimEnd()
    .mapToId(emptySpace) // converts the string into a list of bitsets representing each letter
    .map { id -> letterMap[id] ?: '?' } // converts each bitset to a letter using a precomputed map
    .joinToString("")

/**
 * Creates a map of Long representations of the letters in two font sizes, mapped to the Char represented.
 */
private val letterMap: Map<Long, Char> = buildMap {
    val letters6 = "ABCEFGHIJKLOPRSUYZ"

    val letterForms6 = """
    .##..###...##..####.####..##..#..#.###...##.#..#.#.....##..###..###...###.#..#.#...#.####
    #..#.#..#.#..#.#....#....#..#.#..#..#.....#.#.#..#....#..#.#..#.#..#.#....#..#.#...#....#
    #..#.###..#....###..###..#....####..#.....#.##...#....#..#.#..#.#..#.#....#..#..#.#....#.
    ####.#..#.#....#....#....#.##.#..#..#.....#.#.#..#....#..#.###..###...##..#..#...#....#..
    #..#.#..#.#..#.#....#....#..#.#..#..#..#..#.#.#..#....#..#.#....#.#.....#.#..#...#...#...
    #..#.###...##..####.#.....###.#..#.###..##..#..#.####..##..#....#..#.###...##....#...####
""".trimIndent()

    val letters10 = "ABCEFGHJKLNPRXZ"

    val letterForms10 = """
    ..##...#####...####..######.######..####..#....#....###.#....#.#......#....#.#####..#####..#....#.######
    .#..#..#....#.#....#.#......#......#....#.#....#.....#..#...#..#......##...#.#....#.#....#.#....#......#
    #....#.#....#.#......#......#......#......#....#.....#..#..#...#......##...#.#....#.#....#..#..#.......#
    #....#.#....#.#......#......#......#......#....#.....#..#.#....#......#.#..#.#....#.#....#..#..#......#.
    #....#.#####..#......#####..#####..#......######.....#..##.....#......#.#..#.#####..#####....##......#..
    ######.#....#.#......#......#......#..###.#....#.....#..##.....#......#..#.#.#......#..#.....##.....#...
    #....#.#....#.#......#......#......#....#.#....#.....#..#.#....#......#..#.#.#......#...#...#..#...#....
    #....#.#....#.#......#......#......#....#.#....#.#...#..#..#...#......#...##.#......#...#...#..#..#.....
    #....#.#....#.#....#.#......#......#...##.#....#.#...#..#...#..#......#...##.#......#....#.#....#.#.....
    #....#.#####...####..######.#.......###.#.#....#..###...#....#.######.#....#.#......#....#.#....#.######
""".trimIndent()

    populateLetterMap(letterForms6, letters6)
    populateLetterMap(letterForms10, letters10)
}

/**
 * Helper function to populate the above letterMap.
 */
private fun MutableMap<Long, Char>.populateLetterMap(letterForms: String, letters: String) {
    letterForms
        .mapToId('.') // turns the letter forms into bitset ids
        .zip(letters.toList())   // zips them with the string showing the discovered letters
        .forEach { (id, c) -> put(id, c) } // populates the map
}

/**
 * Helper function for both ocr() and populateLetterMap(). Takes a string, chops it up by columns, separates
 * the letters, then converts them to a Long bitset for easy map access.
 */
private fun String.mapToId(emptySpace: Char): List<Long> = buildList {
    // height is needed to handle the "Y bug" where there is no space after a 'Y' in one input, as well as for
    // splitting the string up into columns.
    val width = this@mapToId.indexOf('\n')
    val height = length / width

    // id is the bitset we use to represent each letter. We convert each character into a list of BooleanArrays,
    // then represent all these values as a bitset. id starts at zero and gets built out column by column.
    var id = 0L

    // we need to track letter width to handle the "Y bug". Luckily, 'Y' is the only character in the size 6 font set
    // that has a width of 5, which makes it easy to identify.
    var letterWidth = 0

    // chop the string up into columns...
    for (x in 0 until width) {
        val col = BooleanArray(height) { y -> this@mapToId[x + y * (width + 1)] != emptySpace }
        if (col.none { it }) { // Handle the ordinary space case
            if (id != 0L) add(id) // Sometimes there are extra spaces so don't add if the bitset has not been built out
            id = 0 // Reset the bitset for the next column
            letterWidth = 0 // Reset the letter width counter
        } else { // Add the column values to the bitset case
            // Handle the "Y bug" case by immediately adding any 'Y' and resetting the id and letterwidth to 0
            if (height == 6 && letterWidth == 5) {
                add(id)
                id = 0
                letterWidth = 0
            }
            // Build out the bitset
            id = col.fold(id) { acc, b -> (acc shl 1) + if (b) 1 else 0 }
            letterWidth++
        }
    }
    // Add anything left in the hopper after all the columns have been processed
    if (id != 0L) add(id)
}

@JvmName("ocrBoolean")
fun Grid<Boolean>.ocr() = rep().ocr('⚫')

@JvmName("ocrChar")
fun Grid<Char>.ocr() = representation { it }.ocr()
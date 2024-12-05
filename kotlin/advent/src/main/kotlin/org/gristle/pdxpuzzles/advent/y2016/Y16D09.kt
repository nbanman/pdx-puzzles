package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getIntList

class Y16D09(private val input: String) : Day {

    // regex to find markers in the string
    private val markerRx = Regex("""\(\d+x\d+\)""")

    // returns the decompressed length of a string. "recursive" boolean flag used for part 2.
    private fun String.decompressedLength(recursive: Boolean): Long {
        var decompressedLength = 0L

        // index advances to parse the string
        var index = 0

        // keep going until there are no more chars to parse
        while (index < length) {

            // get the next marker. If none found, return early, adding the length of the remaining unparsed string
            val marker = markerRx
                .find(substring(index))
                ?: return decompressedLength + substring(index).length // return early if no marker found

            // add the length of any characters preceding the marker
            decompressedLength += substring(index, index + marker.range.first).length

            // get the length and number of repeats in the marker
            val (sequenceLength, repeats) = marker.value.getIntList()

            // get the length of the sequence affected by the marker
            val sequence = substring(index + marker.range.last + 1, index + marker.range.last + 1 + sequenceLength)
                .let {

                    // If part 2, feed the sequence into the function recursively, otherwise just use the length of
                    // the string.
                    @Suppress("KotlinConstantConditions")
                    if (recursive) it.decompressedLength(recursive) else it.length.toLong()
                }.times(repeats)

            // add this length to the overall decompressed length
            decompressedLength += sequence

            // advance the index to the character after the sequence
            index += marker.range.last + sequenceLength + 1
        }
        return decompressedLength
    }

    override fun part1() = input.decompressedLength(false)
    override fun part2() = input.decompressedLength(true)
}

fun main() = Day.runDay(Y16D09::class)

//    Class creation: 13ms
//    Part 1: 110346 (2ms)
//    Part 2: 10774309173 (15ms)
//    Total time: 31ms


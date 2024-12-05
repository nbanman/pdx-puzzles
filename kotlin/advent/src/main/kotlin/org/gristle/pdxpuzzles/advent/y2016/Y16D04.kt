package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.groupValues

class Y16D04(input: String) : Day {

    class Room private constructor(val name: String, val id: Int) {
        companion object {
            fun from(encryptedName: String, id: Int, checkSum: String): Room? {
                val isReal = checkSum == encryptedName
                    .replace("-", "") // don't count dashes
                    .groupingBy { it }
                    .eachCount() // deliver size of each group of characters
                    .entries // grab both key (the char) and the value (the size)
                    // sort the entries by size then alphabetical
                    .sortedWith(compareByDescending<Map.Entry<Char, Int>> { it.value }.thenBy { it.key })
                    .take(5) // take 1st 5
                    .map { it.key } // use only the char
                    .joinToString("") // join them to string
                return if (isReal) {
                    fun Char.shift() = if (this == '-') ' ' else ((code - 97 + id) % 26 + 97).toChar()
                    val name = encryptedName.map { it.shift() }.joinToString("") 
                    Room(name, id)
                } else {
                    null
                }
            }
        }
    }

    private val rooms = input
        .groupValues("""([a-z-]+)-(\d+)\[([a-z]+)\]""")
        .mapNotNull { (encryptedName, idString, checkSum) -> Room.from(encryptedName, idString.toInt(), checkSum) }

    override fun part1() = rooms.sumOf(Room::id)

    override fun part2() = rooms.find { it.name == "northpole object storage" }?.id?.toString() ?: "not found"
}

fun main() = Day.runDay(Y16D04::class)

//    Class creation: 42ms
//    Part 1: 158835 (0ms)
//    Part 2: 993 (0ms)
//    Total time: 42ms

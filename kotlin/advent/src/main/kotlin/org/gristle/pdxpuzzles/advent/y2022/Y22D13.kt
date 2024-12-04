package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import kotlin.math.sign

class Y22D13(input: String) : Day {

    /**
     * Packets consist of Values (an Int value) and Lists (a list of sub-packets). They are comparable using
     * rules spelled out in the puzzle.
     */
    sealed class Packet : Comparable<Packet> {
        class Value(val value: Int) : Packet() {

            // it's easiest to use the PacketList comparison logic when comparing values to PacketLists, so this 
            // utility function turns a Value into a PacketList with one sub-Value.
            fun toPacket() = PacketList(listOf(this))

            override fun compareTo(other: Packet): Int {
                return when (other) {
                    is Value -> (this.value - other.value).sign
                    else -> this.toPacket().compareTo(other)
                }
            }
        }

        class PacketList(val list: List<Packet>) : Packet() {
            override fun compareTo(other: Packet): Int {
                when (other) {
                    is PacketList -> {
                        (this.list zip other.list).forEach { (leftItem, rightItem) ->
                            val answer = leftItem.compareTo(rightItem)
                            if (answer != 0) return answer
                        }
                        return (this.list.size - other.list.size).sign
                    }

                    is Value -> {
                        return this.compareTo(other.toPacket())
                    }
                }
            }
        }

        companion object {
            private val regex = Regex("""\[|]|\d+""")

            /**
             * Factory method. Finds all relevant tokens, uses a consumable iterator so that makePacket() can go
             * recursive without losing its place.
             */
            fun of(line: String): PacketList {

                // Consumable iterator of '[', ']', and '#'.
                val iterator: Iterator<String> = regex.findAll(line).map(MatchResult::value).iterator()

                // Recursive function makes PacketLists, filling out any sub-packets as necessary.
                fun makePacket(): PacketList {
                    val list: List<Packet> = buildList {
                        while (iterator.hasNext()) { // while loop consumes entire iterator unless break encountered
                            when (val next = iterator.next()) {
                                "[" -> add(makePacket()) // '[' means a PacketList found, so go recursive
                                "]" -> break // ']' means packet finished, so break, finishing that list
                                else -> add(Value(next.toInt())) // a number means add a Value as a sub-packet
                            }
                        }
                    }
                    return PacketList(list)
                }

                return makePacket()
            }
        }
    }

    private val packets = input
        .lineSequence()
        .filter(String::isNotBlank)
        .map(Packet::of)
        .toList()

    override fun part1(): Int = packets
        .chunked(2) // pairs the packets
        .withIndex()
        .sumOf { (index, value) -> if (value.first() < value.last()) index + 1 else 0 }

    override fun part2(): Int {
        val dividerPackets = listOf(Packet.of("[[2]]"), Packet.of("[[6]]"))
        return dividerPackets
            .mapIndexed { index, packet -> packets.count { packet > it } + index + 1 }
            .reduce(Int::times)
    }
}

fun main() = Day.runDay(Y22D13::class)

//    Class creation: 34ms
//    Part 1: 5506 (2ms)
//    Part 2: 21756 (2ms)
//    Total time: 39ms
package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import org.gristle.pdxpuzzles.utilities.parsing.getLongList
import java.io.IOException
import java.util.*

private typealias Readout = Triple<String, List<String>, List<String>>

class Y19D25(private val input: String) : Day {

    /**
     * Unused function that plays the game.
     */
    private fun play() {
        // Set up Intcode with program
        val initialState = input.getLongList()
        val output: Deque<Long> = LinkedList()
        val toComp: Deque<Long> = LinkedList()
        val intCode = IntCode("A", initialState, null, toComp, output)
        intCode.run()
        // support saves
        val saves = mutableMapOf<String, ICSave>()

        // game loop
        while (true) {
            // read output and print it
            println(output.read())
            output.clear()

            // get input from terminal
            val input = readlnOrNull() ?: throw IOException("Failed to read from console")

            // parse and expand shorthand, then recombine input
            val firstWord = input.takeWhile { it != ' ' }
            val remainder = input.dropWhile { it != ' ' }

            val expandedInput = when (firstWord) {
                "q" -> "quit"
                "n" -> "north"
                "s" -> "south"
                "e" -> "east"
                "w" -> "west"
                "t" -> "take"
                "d" -> "drop"
                "i" -> "inv"
                else -> firstWord
            } + remainder

            // handle save, load, and quit
            if (expandedInput.contains("save ")) {
                saves[expandedInput.takeLastWhile { it != ' ' }] = intCode.save()
                continue
            }

            if (expandedInput.contains("load ")) {
                intCode.restore(saves.getValue(expandedInput.takeLastWhile { it != ' ' }))
                continue
            }

            if (expandedInput.contains("quit")) break

            // Otherwise, pass the command to
            execute(expandedInput, intCode, toComp)
        }
    }

    private val locationRx = Regex("""== ((?:\w+ ?)+) ==""")
    private val doorsRx = Regex("""Doors here lead:\n((?:- \w+\n)+)""")
    private val itemizedRx = Regex("""Items here:\n((?:- [a-z ]+\n)+)""")
    private val splitRx = Regex("""\w+(?: \w+)?""")

    private fun Deque<Long>.parse(clear: Boolean = true): Readout {
        val output = read()
        if (clear) clear()
        val location = locationRx.find(output)?.groupValues?.get(1)
            ?: throw IllegalArgumentException("output does not contain location")
        val doors = doorsRx
            .find(output)
            ?.groupValues
            ?.get(1)
            ?.let { s -> splitRx.findAll(s).map { it.value }.toList() }
            ?: throw IllegalArgumentException("output does not contain doors")
        val items = itemizedRx
            .find(output)
            ?.groupValues
            ?.get(1)
            ?.let { s -> splitRx.findAll(s).map { it.value }.toList() }
            ?: emptyList()
        return Readout(location, doors, items)
    }

    /**
     * Utility function for converting string input to Intcode input.
     */
    private fun String.toCode() = map { it.code.toLong() }

    /**
     * Utility function for reading output.
     */
    private fun Iterable<Long>.read() = map { it.toInt().toChar() }.joinToString("")

    // Map used to backtrack.
    private val reverse = mapOf(
        "north" to "south",
        "south" to "north",
        "east" to "west",
        "west" to "east",
    )

    /**
     * Utility function for sending commands to IntCode.
     */
    private fun execute(command: String, intCode: IntCode, toComp: Deque<Long>) {
        toComp.addAll(command.toCode())
        toComp.add(10L)
        intCode.run(100_000)
    }

    /**
     * Recursive DFS for traversing the ship.
     */
    private fun explore(
        stopAtSecurity: Boolean,
        command: String,
        previousLocation: String,
        intCode: IntCode,
        toComp: Deque<Long>,
        output: Deque<Long>
    ): String {
        execute(command, intCode, toComp)
        val (currentLocation, doors, items) = output.parse()

        // returns early if hits pressure plate
        if (previousLocation == currentLocation) {
            return if (stopAtSecurity) command else ""
        }

        // picks up items, undos action if fatal
        if (!stopAtSecurity) items.forEach { item ->
            val save = intCode.save()
            execute("take $item", intCode, toComp)
            output.clear()
            execute("", intCode, toComp)
            if (!output.read().contains("Unrecognized")) {
                intCode.restore(save)
            }
        }

        // moves to next spot
        doors.filter { it != reverse[command] }.forEach { door ->
            val endCommand = explore(stopAtSecurity, door, currentLocation, intCode, toComp, output)
            if (endCommand.isNotBlank()) return endCommand
        }

        // moves back out
        if (command.isNotEmpty()) {
            execute(reverse.getValue(command), intCode, toComp)
            output.clear()
        }

        return ""
    }

    /**
     * DFS that runs through all the item combinations and tests them on the pressure plate, ultimately returning
     * the passcode.
     */
    private fun getPasscode(
        inventory: List<String>,
        direction: String,
        index: Int,
        intCode: IntCode,
        toComp: Deque<Long>,
        output: Deque<Long>,
    ): String {
        // first step on plate and get report
        val step = step(direction, intCode, toComp, output)

        // if "heavy" or the answer, return it. otherwise continue
        if (step != "light") return step

        // go through all items in the list and pick them up, then call the function recursively. if the result 
        // of the recursive call is too heavy, drop that item. If the result of the recursive all is the answer,
        // return it.
        for (i in index until inventory.size) {
            execute("take ${inventory[i]}", intCode, toComp)
            output.clear()
            val innerStep = getPasscode(inventory, direction, i + 1, intCode, toComp, output)
            if (innerStep == "heavy") {
                execute("drop ${inventory[i]}", intCode, toComp)
                output.clear()
            } else if (innerStep.toIntOrNull() != null) {
                return innerStep
            }
        }
        // if all the item combinations don't work, then the current loadout does not work. Report "heavy" so that
        // the outer recursive function drops the last item.
        return "heavy"
    }

    /**
     * Steps on the plate, returning whether the droid should be lighter, heavier, or the passcode if just right.
     */
    private fun step(direction: String, intCode: IntCode, toComp: Deque<Long>, output: Deque<Long>): String {
        toComp.addAll(direction.toCode())
        toComp.add(10L)
        intCode.run()
        val report = output.read()
        output.clear()
        return if ("lighter" in report) {
            "heavy"
        } else if ("heavier" in report) {
            "light"
        } else report.getInts().first().toString()
    }

    override fun part1(): String {
        // Set up the IntCode program
        val initialState = input.getLongList()
        val output: Deque<Long> = ArrayDeque()
        val toComp: Deque<Long> = ArrayDeque()
        val intCode = IntCode("A", initialState, null, toComp, output)

        // Initial Hull breach information needed to start the second pathfinding, so run the program and save
        // a copy of the output.
        intCode.run()
        val outputCopy = (output as ArrayDeque).clone()

        // Run a DFS that traverses entire map, picking up all items, savescumming to avoid fatal items.
        explore(false, "", "Outer Space", intCode, toComp, output)

        // Reset the output for the second traversal, now that all items have been picked up.
        output.addAll(outputCopy)

        // Second DFS to get to Security
        val plateDirection = explore(
            true,
            "",
            "Outer Space",
            intCode, toComp, output
        )

        // Get list of all items...
        execute("inv", intCode, toComp)

        // ...and parse them.
        val inventory = output
            .read()
            .lineSequence()
            .filter { it.isNotBlank() && it[0] == '-' }
            .map { line -> line.drop(2) }
            .toList()

        // drop all items
        inventory.forEach { item ->
            execute("drop $item", intCode, toComp)
        }

        // DFS tries all combinations until the answer is provided.
        return getPasscode(inventory, plateDirection, 0, intCode, toComp, output)
    }

    override fun part2() = play()
}

fun main() {
//    Y19D25(readRawInput("y2019/d25")).part2()
    Day.runDay(Y19D25::class)
}

//    Class creation: 15ms
//    Part 1: 16810049 (304ms)
//    Total time: 319ms

package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.StringGrid

class Y17D19(input: String) : Day {
    // wrap the input string into StringGrid with methods for navigating the string like a 2-D grid.
    private val maze = StringGrid(input)
    
    // a sequence emitting the positions the mouse runs as it navigates the maze
    private val runMaze: Sequence<Int> 
    init {
        // the mouse starts at the first non-blank position at the top 
        val start = maze.string.indexOfFirst { it != ' ' } to Nsew.SOUTH
        
        // function that moves the mouse one step along the maze, returning the new position and direction. If no
        // valid move exists, return null. Along with the generateSequence function that invokes it, a null will
        // stop the sequence.
        val move: (Pair<Int, Nsew>) -> Pair<Int, Nsew>? = { (pos, dir) ->
            // inspect the ground below the mouse. If '+', the mouse moves either left or right. Otherwise it 
            // goes straight.
            val ground = maze[pos]
            if (ground == '+') {
                // Calculate turning left. If it's a valid move, return that. Otherwise go right and return that.
                val leftDir = dir.left() 
                val leftPos = maze.moveOrNull(pos, leftDir) 
                if (leftPos != null && maze[leftPos] != ' ') {
                    leftPos to leftDir
                } else {
                    val rightDir = dir.right()
                    maze.move(pos, rightDir) to rightDir
                }
            } else {
                // Calculate going straight. If it's a valid move, return that. Otherwise, the mouse has run its course
                // and return null.
                val newPos = maze.moveOrNull(pos, dir)
                if (newPos == null || maze[newPos] == ' ') null else newPos to dir
            }
        }
        runMaze = generateSequence(start, move).map { it.first }
    }
    
    // helper function that returns a letter from a maze position, or returns null if no letter found at that index
    private val getLetter: (Int) -> Char? = { pos ->
        val c = maze[pos]
        if (c.isLetter()) c else null 
    }

    override fun part1() = runMaze.mapNotNull(getLetter).joinToString("")
    override fun part2() = runMaze.count()
}

fun main() = Day.runDay(Y17D19::class)

//    Class creation: 7ms
//    Part 1: EOCZQMURF (11ms)
//    Part 2: 16312 (2ms)
//    Total time: 21ms


package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y22D07(input: String) : Day {

    // Directory node tracks child Directories in a map, and tracks the total file size of any files assigned to it 
    // or any child directory.
    private class Directory {
        val directories: MutableMap<String, Directory> = mutableMapOf()
        var fileSize: Int = 0 // not just size of files in this directory, but files in child directories as well

        // Provides a list of all directories in and under this directory.
        fun inclusiveDirectories(): List<Directory> =
            listOf(this) + directories.values.flatMap(Directory::inclusiveDirectories)
    }

    // Parsing input to create file structure
    private fun createFileStructure(input: String): Directory {

        // tracks the full path to current dir, starting with root.
        val path: MutableList<Directory> = mutableListOf(Directory())

        input.lines().forEach { line ->
            when {
                line.startsWith("\$ cd /") -> repeat(path.size - 1) { path.removeLast() } // $ cd / 
                line.startsWith("\$ cd ..") -> if (path.size > 1) path.removeLast() // $ cd ..
                line.startsWith("\$ cd") -> { // cd [directory]
                    val dir = line.takeLastWhile { it != ' ' } // grabs last word in the String
                    path.add(path.last().directories.getOrPut(dir) { Directory() })
                }

                line[0].isDigit() -> { // increase fileSize of all Directories in the path
                    val fileSize = line.takeWhile { it != ' ' }.toInt()
                    path.forEach { dir -> dir.fileSize += fileSize }
                }
            }
        }
        return path.first()
    }

    private val root = createFileStructure(input)
    private val allDirectories = root.inclusiveDirectories()

    override fun part1() = allDirectories
        .filter { it.fileSize <= 100_000 }
        .sumOf(Directory::fileSize)

    override fun part2(): Int {
        val spaceAvailable = 70_000_000 - root.fileSize
        val minDirSize = 30_000_000 - spaceAvailable

        return allDirectories
            .filter { it.fileSize >= minDirSize }
            .minOf(Directory::fileSize)
    }
}

fun main() = Day.runDay(Y22D07::class)

//    Class creation: 14ms
//    Part 1: 1477771 (0ms)
//    Part 2: 3579501 (0ms)
//    Total time: 14ms
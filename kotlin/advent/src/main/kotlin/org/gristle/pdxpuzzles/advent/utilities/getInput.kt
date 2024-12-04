package org.gristle.pdxpuzzles.advent.utilities

import java.io.File
import java.net.HttpURLConnection
import java.net.URI
import java.nio.file.Path
import java.nio.file.Paths

fun getInput(year: Int, day: Int): String {
    // get inputs directory
    val path: Path = Paths
        .get(System.getProperty("user.dir"))
        .parent
        .resolve("inputs/advent/20$year/y${year}d${String.format("%02d", day)}.txt")

    val inputFile = File(path.toUri())

    if (!inputFile.exists()) {
        inputFile.parentFile.mkdirs()
        val url = URI("https://adventofcode.com/20$year/day/$day/input").toURL()
        val connection = url.openConnection() as HttpURLConnection
        connection.setRequestProperty("Cookie", "session=${System.getenv("ADVENT_SESSION")}")
        connection.setRequestProperty(
            "User-Agent",
            "github.com/nbanman/pdx-puzzles/tree/main/kotlin/advent/src/main/kotlin/utilities/getInput.kt by neil.banman@gmail.com"
        )
        try {
            connection.connect()
            check(connection.responseCode == 200) { "${connection.responseCode} ${connection.responseMessage}" }
            connection.inputStream.use { input ->
                inputFile.outputStream().use { output ->
                    input.copyTo(output)
                }
                inputFile.setReadOnly()
            }
        } finally {
            connection.disconnect()
        }
    }
    return inputFile.readText().replace("\r", "").trimEnd { it == '\n' }
}

fun main() {
    println(getInput(24, 1))
}
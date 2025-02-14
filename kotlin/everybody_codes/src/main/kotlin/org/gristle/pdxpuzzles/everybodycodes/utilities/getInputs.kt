package org.gristle.pdxpuzzles.everybodycodes.utilities

import java.io.File
import java.nio.file.Paths

fun getInputs(year: Int, day: Int): List<String?> {
    // get inputs directory
    val template = Paths
        .get(System.getProperty("user.dir"))
        .parent
        .toString() +
        "/inputs/everybody_codes/20$year/y${year}d${String.format("%02d", day)}q"

    return List(3) { quest ->
        File("$template${quest + 1}.txt").takeIf { it.exists() }?.readText()?.trimEnd()
    }
}

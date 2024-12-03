package org.gristle.utils

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlinx.serialization.json.Json
import kotlinx.serialization.encodeToString
import org.gristle.utils.*

internal class PrinterTest {

    @Test
    fun testMessage() {
        val message = "message 2"
        val testPrinter = Printer(message)
        assertEquals(testPrinter.message, message)
    }

    @Test
    fun testSerialization() {
        val message = "message"
        val json1 = Json.encodeToString(Printer(message))
        val json2 = Json.encodeToString(Printer(message))
        assertEquals(json1, json2)
    }
}
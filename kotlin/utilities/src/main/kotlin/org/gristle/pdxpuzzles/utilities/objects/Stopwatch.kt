@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.objects

enum class TimeUnits(val divisor: Long) {
    S(1_000_000_000),
    MS(1_000_000),
    US(1_000),
    NS(1)
}

class Stopwatch(start: Boolean = false, private val units: TimeUnits = TimeUnits.MS) {
    private var elapsed = 0L
    private var lastStart = 0L
    private var isRunning = false

    init {
        if (start) start()
    }

    fun start(): Boolean =
        if (isRunning) false else {
            lastStart = System.nanoTime()
            isRunning = true
            true
        }

    fun stop(units: TimeUnits? = null): Long {
        val now = System.nanoTime()
        if (isRunning) {
            isRunning = false
            elapsed += now - lastStart
        }
        return elapsed / (units?.divisor ?: this.units.divisor)
    }

    fun lap(units: TimeUnits? = null): Long =
        if (isRunning) {
            val now = System.nanoTime()
            val lap = now - lastStart
            elapsed += lap
            lastStart = now
            lap / (units?.divisor ?: this.units.divisor)
        } else 0


    fun reset() {
        elapsed = 0
        lastStart = 0
        isRunning = false
    }

    fun elapsed(units: TimeUnits? = null) = elapsed / (units?.divisor ?: this.units.divisor)
}
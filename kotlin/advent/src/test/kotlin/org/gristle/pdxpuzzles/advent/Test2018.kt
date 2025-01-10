package org.gristle.pdxpuzzles.advent

import org.gristle.pdxpuzzles.advent.utilities.Day
import kotlin.test.Test
import kotlin.test.assertEquals

import org.gristle.pdxpuzzles.advent.y2018.Y18D01
import org.gristle.pdxpuzzles.advent.y2018.Y18D02
import org.gristle.pdxpuzzles.advent.y2018.Y18D03
import org.gristle.pdxpuzzles.advent.y2018.Y18D04
import org.gristle.pdxpuzzles.advent.y2018.Y18D05
import org.gristle.pdxpuzzles.advent.y2018.Y18D06
import org.gristle.pdxpuzzles.advent.y2018.Y18D07
import org.gristle.pdxpuzzles.advent.y2018.Y18D08
import org.gristle.pdxpuzzles.advent.y2018.Y18D09
import org.gristle.pdxpuzzles.advent.y2018.Y18D10
import org.gristle.pdxpuzzles.advent.y2018.Y18D11
import org.gristle.pdxpuzzles.advent.y2018.Y18D12
import org.gristle.pdxpuzzles.advent.y2018.Y18D13
import org.gristle.pdxpuzzles.advent.y2018.Y18D14
import org.gristle.pdxpuzzles.advent.y2018.Y18D15
import org.gristle.pdxpuzzles.advent.y2018.Y18D16
import org.gristle.pdxpuzzles.advent.y2018.Y18D17
import org.gristle.pdxpuzzles.advent.y2018.Y18D18
import org.gristle.pdxpuzzles.advent.y2018.Y18D19
import org.gristle.pdxpuzzles.advent.y2018.Y18D20
import org.gristle.pdxpuzzles.advent.y2018.Y18D21
import org.gristle.pdxpuzzles.advent.y2018.Y18D22
import org.gristle.pdxpuzzles.advent.y2018.Y18D23
import org.gristle.pdxpuzzles.advent.y2018.Y18D24
import org.gristle.pdxpuzzles.advent.y2018.Y18D25

class Test2018 {
    @Test
    internal fun y2018d1() {
        assertEquals("433" to "256", Day.testDay(Y18D01::class))
    }

    @Test
    internal fun y2018d2() {
        assertEquals("7688" to "lsrivmotzbdxpkxnaqmuwcchj", Day.testDay(Y18D02::class))
    }

    @Test
    internal fun y2018d3() {
        assertEquals("110891" to "297", Day.testDay(Y18D03::class))
    }

    @Test
    internal fun y2018d4() {
        assertEquals("19025" to "23776", Day.testDay(Y18D04::class))
    }

    @Test
    internal fun y2018d5() {
        assertEquals("10972" to "5278", Day.testDay(Y18D05::class))
    }

    @Test
    internal fun y2018d6() {
        assertEquals("5365" to "42513", Day.testDay(Y18D06::class))
    }

    @Test
    internal fun y2018d7() {
        assertEquals("ABGKCMVWYDEHFOPQUILSTNZRJX" to "898", Day.testDay(Y18D07::class))
    }

    @Test
    internal fun y2018d8() {
        assertEquals("36027" to "23960", Day.testDay(Y18D08::class))
    }

    @Test
    internal fun y2018d9() {
        assertEquals("422980" to "3552041936", Day.testDay(Y18D09::class))
    }

    @Test
    internal fun y2018d10() {
        assertEquals("LRCXFXRP" to "10630", Day.testDay(Y18D10::class))
    }

    @Test
    internal fun y2018d11() {
        assertEquals("235,48" to "285,113,11", Day.testDay(Y18D11::class))
    }

    @Test
    internal fun y2018d12() {
        assertEquals("4110" to "2650000000466", Day.testDay(Y18D12::class))
    }

    @Test
    internal fun y2018d13() {
        assertEquals("86,118" to "2,81", Day.testDay(Y18D13::class))
    }

    @Test
    internal fun y2018d14() {
        assertEquals("4910101614" to "20253137", Day.testDay(Y18D14::class))
    }

    @Test
    internal fun y2018d15() {
        assertEquals("224370" to "45539", Day.testDay(Y18D15::class))
    }

    @Test
    internal fun y2018d16() {
        assertEquals("529" to "573", Day.testDay(Y18D16::class))
    }

    @Test
    internal fun y2018d17() {
        assertEquals("40879" to "34693", Day.testDay(Y18D17::class))
    }

    @Test
    internal fun y2018d18() {
        assertEquals("605154" to "200364", Day.testDay(Y18D18::class))
    }

    @Test
    internal fun y2018d19() {
        assertEquals("1764" to "18992484", Day.testDay(Y18D19::class))
    }

    @Test
    internal fun y2018d20() {
        assertEquals("3930" to "8240", Day.testDay(Y18D20::class))
    }

    @Test
    internal fun y2018d21() {
        assertEquals("3345459" to "skipped", Day.testDay(Y18D21::class, skipPartTwo = true))
    }

    @Test
    internal fun y2018d22() {
        assertEquals("5637" to "969", Day.testDay(Y18D22::class))
    }

    @Test
    internal fun y2018d23() {
        assertEquals("481" to "47141479", Day.testDay(Y18D23::class))
    }

    @Test
    internal fun y2018d24() {
        assertEquals("15165" to "4037", Day.testDay(Y18D24::class))
    }

    @Test
    internal fun y2018d25() {
        assertEquals("394" to "skipped", Day.testDay(Y18D25::class))
    }
}

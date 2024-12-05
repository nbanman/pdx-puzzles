package org.gristle.pdxpuzzles.advent.y2020

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit
import org.gristle.pdxpuzzles.utilities.parsing.groupValues

class Y20D04(input: String) : Day {

    sealed class PassportField {
        abstract fun isValid(): Boolean

        companion object {
            fun of(gv: List<String>): PassportField {
                val (field, info) = gv
                return when (field) {
                    "byr" -> Byr(info)
                    "iyr" -> Iyr(info)
                    "eyr" -> Eyr(info)
                    "hgt" -> Hgt(info)
                    "hcl" -> Hcl(info)
                    "ecl" -> Ecl(info)
                    "pid" -> Pid(info)
                    else -> Cid
                }
            }
        }

        class Byr(private val info: String) : PassportField() {
            override fun isValid() = (1920..2002).contains(info.toIntOrNull())
        }

        class Iyr(private val info: String) : PassportField() {
            override fun isValid() = (2010..2020).contains(info.toIntOrNull())
        }

        class Eyr(private val info: String) : PassportField() {
            override fun isValid() = (2020..2030).contains(info.toIntOrNull())
        }

        class Hgt(info: String) : PassportField() {
            private val amt = info.dropLast(2).toIntOrNull() ?: -1
            private val isCm = info.takeLast(2) == "cm"
            override fun isValid(): Boolean {
                return (isCm && amt in 150..193) || (!isCm && amt in 59..76)
            }
        }

        class Hcl(private val info: String) : PassportField() {
            override fun isValid() = """#[a-f\d]{6}""".toRegex().matches(info)
        }

        class Ecl(private val info: String) : PassportField() {
            override fun isValid() = """amb|blu|brn|gry|grn|hzl|oth""".toRegex().matches(info)
        }

        class Pid(private val info: String) : PassportField() {
            override fun isValid() = info.length == 9 && info.all(Char::isDigit)
        }

        data object Cid : PassportField() {
            override fun isValid() = true
        }
    }

    private val passports = input
        .blankSplit()
        .map { rawPassportData ->
            rawPassportData
                .groupValues("""([a-z]{3}):([^ \r\n]+)""")
                .map(PassportField::of)
                .filter { it !is PassportField.Cid }
        }.filter { passportFields -> passportFields.size == 7 }

    override fun part1() = passports.size

    override fun part2() = passports.count { it.all(PassportField::isValid) }
}

fun main() = Day.runDay(Y20D04::class)

//    Class creation: 38ms
//    Part 1: 242 (0ms)
//    Part 2: 186 (8ms)
//    Total time: 47ms
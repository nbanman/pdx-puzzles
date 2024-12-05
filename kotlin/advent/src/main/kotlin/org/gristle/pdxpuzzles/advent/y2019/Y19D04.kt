package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y19D04(input: String) : Day {

    // passwords for part 1 are used for both parts
    private val part1Passwords: List<String> by lazy {
        input // take input
            .split('-') // split String into two Strings representing numbers
            .let { it[0].toInt()..it[1].toInt() } // create range between the two numbers
            .map(Int::toString) // convert each number in range to String for rule analysis
            .filter { password -> // apply part 1 rules b/c they apply to both parts
                val zippedPassString = password.zipWithNext() // compare each digit with the one after it
                zippedPassString.none { (prev, next) -> prev > next } // digits never decrease 
                        && zippedPassString.any { (prev, next) -> prev == next } // at least one is the same
            }
    }

    override fun part1() = part1Passwords.size // return number of passwords

    override fun part2(): Int {
        val pattern = Regex("""(\d)\1+""") // Regex looks for repeated digits of any length
        return part1Passwords
            // For each password, find all sequences of repeated digits and keep those that have at least one
            // sequence that only has two digits.
            .filter { password -> pattern.findAll(password).any { it.value.length == 2 } }
            .size // return number of passwords matching this criteria
    }
}

fun main() = Day.runDay(Y19D04::class)

//    Class creation: 159ms
//    Part 1: 466 (0ms)
//    Part 2: 292 (4ms)
//    Total time: 164ms
package org.gristle.pdxpuzzles.advent.y2016

import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*
import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.Md5.toHex
import java.security.MessageDigest

class Y16D14(private val salt: String) : Day {

    // utility function for accessing the indexes of 5-strings in an IntArray
    private fun Char.hexIndex(): Int = if (isDigit()) digitToInt() else this - 'W'

    // Regexes used to find 3- and 5-strings in hashes
    private val threeRx = Regex("""([0-9a-f])\1{2,}""")
    private val fiveRx = Regex("""([0-9a-f])\1{4,}""")

    @OptIn(FlowPreview::class)
    private fun solve(routines: Int = 1, hashing: (digest: MessageDigest, String) -> String): Int {

        // for each hex value 0-f, store index of last time 5-string appeared
        val fives = IntArray(16) { -1 }

        // mutable list of validated keys
        val keys = ArrayList<Int>(70)

        // rolling list of 1,000 of the 3-string value of hashes. returns 'X' if no 3-string in hash
        val threes = ArrayDeque<Char>(1001)

        // adds a 3-string value to the rolling list, returning the index of the value that rolled off if that
        // index has been validated
        fun ArrayDeque<Char>.addRolling(three: Char, index: Int): Int? {

            // add value
            addLast(three)

            // do nothing if list is not full
            if (size <= 1000) return null

            // if full, start rolling off
            val evaluate = removeFirst()
            if (evaluate == 'X') return null
            val evaluateIndex = index - 1000

            // check fives to see if any of the next 1,000 hashes has a 5-string matching the rolling off 3-string
            if (fives[evaluate.hexIndex()] !in evaluateIndex + 1..index) return null
            return evaluateIndex
        }

        // Flow starting with increasing index, generating a hash based on that and the salt. For each hash,
        // record any 5-string in the fives with the current index. Add the 3-string value to the rolling list.
        // When 3-string values start rolling off, check fives to see if that value has shown up as a five. If
        // so, add it to the list of keys. Keep going until the 64th key is found.
        val flowGenerator: Flow<Int> = generateSequence(0) { it + 1 } // Sequence starting with increasing index...
            .asFlow() // turn into Flow

        // Hashes can be run sequentially or in parallel. Sequential is faster for part 1 because this allows us to 
        // reuse the MessageDigest instance, which is expensive to create. Parallel is faster for part2 because while
        // each coroutine creates its own MessageDigest instance, it then uses it 2017 times so the init cost is 
        // amortized.
        val hashGenerator: Flow<String> = if (routines == 1) {
            val digest = MessageDigest.getInstance("MD05")
            flowGenerator.map { seed -> hashing(digest, (salt + seed)) }
        } else {
            flowGenerator
                .map { n ->
                    withContext(Dispatchers.Default) {
                        (n * routines until n * routines + routines)
                            .map { seed ->
                                async {
                                    val digest = MessageDigest.getInstance("MD05")
                                    hashing(digest, (salt + seed))
                                }
                            }.awaitAll()
                            .asFlow()
                    }
                }.flattenConcat()
        }

        val keyFlow = hashGenerator
            .withIndex()
            .onEach { (n, md5) ->
                // For each hash, record any 5-string in the fives with the current index.
                fiveRx
                    .findAll(md5)
                    .forEach { fives[it.groupValues[1][0].hexIndex()] = n }

                // Add the 3-string value to the rolling list.
                val three = threeRx.find(md5)?.value?.get(0) ?: 'X'
                threes
                    .addRolling(three, n) // When 3-string values start rolling off, check fives... 
                    ?.let { key -> keys.add(key) } // ...If so, add it to the list of keys.
            }.takeWhile { keys.size < 64 } // Keep going until the 64th key is found.

        keyFlow.collectBlocking()

        return keys.last()
    }

    private fun Flow<IndexedValue<String>>.collectBlocking() = runBlocking { collect() }

    override fun part1() = solve { digest, s ->
        digest.update(s.toByteArray())
        digest.digest().toHex()
    }

    override fun part2() = solve(512) { digest, s ->
        // needs to convert back and forth from ByteArray and String because Java MessageDigest returns uppercase
        // A-F, and we need to feed back in lowercase a-f values.
        (1..2017).fold(s) { acc, _ ->
            digest.update(acc.toByteArray())
            digest.digest().toHex()
        }
    }
}

fun main() = Day.runDay(Y16D14::class)

//    Class creation: 2ms
//    Part 1: 18626 (152ms)
//    Part 2: 20092 (1875ms)
//    Total time: 2030ms

//    Y16D14 Part 1
//    
//    Warm-up 1: 157599 us/op
//    Iteration 1: 62071 us/op
//    Iteration 2: 49656 us/op
//    Iteration 3: 58339 us/op
//    Iteration 4: 49319 us/op
//    Iteration 5: 49527 us/op
//    
//    53782.4 us/op [Average]
//    
//    Y16D14 Part 2
//    
//    Warm-up 1: 1847431 us/op
//    Iteration 1: 1514573 us/op
//    Iteration 2: 1538634 us/op
//    Iteration 3: 1517352 us/op
//    Iteration 4: 1521000 us/op
//    Iteration 5: 1506612 us/op
//    
//    1519634.2 us/op [Average]
//    
//    Parts 1 and 2: 1573416 us/op [Average]
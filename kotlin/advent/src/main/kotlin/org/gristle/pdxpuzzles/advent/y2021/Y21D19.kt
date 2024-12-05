package org.gristle.pdxpuzzles.advent.y2021

import kotlinx.coroutines.*
import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.MCoord
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import kotlin.math.abs

class Y21D19(input: String) : Day {
    companion object {
        private fun <T> List<T>.elementPairs(): Sequence<Pair<T, T>> = sequence {
            for (i in 0 until size - 1)
                for (j in i + 1 until size)
                    yield(get(i) to get(j))
        }
    }

    data class Scanner(val id: String, val beacons: List<MCoord>, val scannerLocations: List<MCoord> = emptyList()) {

        companion object {
            fun of(s: String): Scanner {
                val id = s.getInts().first().toString()
                val beacons = s
                    .getInts()
                    .drop(1)
                    .chunked(3, ::MCoord)
                    .toList()
                return Scanner(id, beacons)
            }
        }

        fun MCoord.toSet(): Set<Int> = coordinates.map { abs(it) }.toSet()

        val morphisms = beacons
            .map { b ->
                listOf(
                    MCoord(b[0], b[1], b[2]),
                    MCoord(b[1], -b[0], b[2]),
                    MCoord(-b[0], -b[1], b[2]),
                    MCoord(-b[1], b[0], b[2]),
                    MCoord(b[2], b[1], -b[0]),
                    MCoord(b[2], -b[0], -b[1]),
                    MCoord(b[2], -b[1], b[0]),
                    MCoord(b[2], b[0], b[1]),
                    MCoord(-b[0], b[1], -b[2]),
                    MCoord(-b[1], -b[0], -b[2]),
                    MCoord(b[0], -b[1], -b[2]),
                    MCoord(b[1], b[0], -b[2]),
                    MCoord(-b[2], b[1], b[0]),
                    MCoord(-b[2], -b[0], b[1]),
                    MCoord(-b[2], -b[1], -b[0]),
                    MCoord(-b[2], b[0], -b[1]),
                    MCoord(b[0], b[2], -b[1]),
                    MCoord(b[1], b[2], b[0]),
                    MCoord(-b[0], b[2], b[1]),
                    MCoord(-b[1], b[2], -b[0]),
                    MCoord(b[0], -b[2], b[1]),
                    MCoord(b[1], -b[2], -b[0]),
                    MCoord(-b[0], -b[2], -b[1]),
                    MCoord(-b[1], -b[2], b[0]),
                )
            }

        var mightyMorphisms = List(morphisms.first().size) { i ->
            List(morphisms.size) { j ->
                morphisms[j][i]
            }
        }

        val coordPairs = beacons
            .indices
            .toList()
            .elementPairs()
            .toList()
            .associateBy { (a, b) -> (beacons[a] - beacons[b]).toSet() }

        override fun toString(): String {
            return "Scanner(id=$id, beacons=${beacons.size})"
        }
    }

    private val scanners = input
        .splitToSequence("\n\n")
        .map(Scanner::of)
        .toList()

    data class SharedSets(val master: Scanner, val b: Scanner, val matches: Set<Set<Int>>) {
        override fun toString(): String {
            return "SharedSets(master=$master, b=$b, sets=${matches.size})"
        }

        fun merge(): Scanner {
            // Pick the first matches and use them to align and find offset
            val matchSet = matches.first()
            val masterIndexPair = master.coordPairs.getValue(matchSet)
            val bIndexPair = b.coordPairs.getValue(matchSet)
            val m1 = master.beacons[masterIndexPair.first]
            val m2 = master.beacons[masterIndexPair.second]
            val mDiff = m1 - m2
            val bMorph = b.morphisms[bIndexPair.first] zip b.morphisms[bIndexPair.second]
            var match = bMorph
                .withIndex()
                .find { it.value.first - it.value.second == mDiff }
            if (match != null) {
                val offset = m1 - match.value.first
                val rotatedOffsetBeacons = b.mightyMorphisms[match.index].map { it + offset }
                return Scanner(
                    master.id + b.id,
                    (master.beacons + rotatedOffsetBeacons).distinct(),
                    master.scannerLocations + offset
                )
            } else {
                match = bMorph
                    .withIndex()
                    .find { it.value.second - it.value.first == mDiff }
                    ?: throw Exception("mDiff not found among morphisms")
                val offset = m1 - match.value.second
                val rotatedOffsetBeacons = b.mightyMorphisms[match.index].map { it + offset }
                return Scanner(
                    master.id + b.id,
                    (master.beacons + rotatedOffsetBeacons).distinct(),
                    master.scannerLocations + offset
                )
            }

        }
    }

    fun solve(): Pair<Int, Int> = runBlocking {

        var master = scanners.first()
        val mScan = scanners.drop(1).toMutableList()

        while (mScan.isNotEmpty()) {

            val sharedSet = sharedSet(mScan, master)

            master = sharedSet.merge()
            mScan.remove(sharedSet.b)
        }

        val furthestDistance =
            master.scannerLocations.elementPairs().toList().maxOf { (a, b) -> a.manhattanDistance(b) }

        master.beacons.size to furthestDistance
    }

    private suspend fun sharedSet(
        mScan: MutableList<Scanner>,
        master: Scanner
    ): SharedSets = withContext(Dispatchers.Default) {
        mScan
            .map {
                async {
                    SharedSets(master, it, master.coordPairs.keys.intersect(it.coordPairs.keys))
                }
            }.awaitAll()
            .first { it.matches.size >= 66 }
    }

    private val solution = solve()

    override fun part1() = solution.first

    override fun part2() = solution.second
}

fun main() = Day.runDay(Y21D19::class)

//    Class creation: 2275ms
//    Part 1: 378 (0ms)
//    Part 2: 13148 (0ms)
//    Total time: 2275ms
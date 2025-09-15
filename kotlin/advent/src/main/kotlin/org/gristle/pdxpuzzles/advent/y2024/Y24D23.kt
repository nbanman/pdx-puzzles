package org.gristle.pdxpuzzles.advent.y2024

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.combinatorics.getPairs
import java.util.concurrent.atomic.AtomicInteger
import kotlin.math.max

class Y24D23(input: String) : Day {
    private val lan: Map<String, Set<String>> = buildMap<String, MutableSet<String>> {
        for (line in input.lines()) {
            val (a, b) = line.split('-')
            getOrPut(a) { mutableSetOf() }.add(b)
            getOrPut(b) { mutableSetOf() }.add(a)
        }
    }

    override fun part1(): Int {
        return lan
            .filter { (a, _) -> a.startsWith('t') }
            .flatMap { (a, bs) ->
                bs.toList()
                    .getPairs()
                    .filter { (b, c) -> lan.getValue(b).contains(c) }
                    .map { (b, c) -> setOf(a, b, c) }
            }.toSet()
            .size
    }

    override fun part2(): String {
        return lan.keys
            .map { pc ->
                val connections = lan.getValue(pc) + pc
                connections
                    .map { nextPc ->
                        val intersect = lan.getValue(nextPc).intersect(connections) + nextPc
                        nextPc to intersect
                    }.fold(connections) { acc, (nextPc, intersect) ->
                        val trial = acc.intersect(intersect)
                        if (trial.size >= 13) trial else acc - nextPc
                    }.sorted()
            }.maxBy { it.size }
            .joinToString(",")
    }
}

fun main() = Day.runDay(Y24D23::class)

//    Class creation: 7ms
//    Part 1: 1253 (6ms)
//    Part 2: ag,bt,cq,da,hp,hs,mi,pa,qd,qe,qi,ri,uq (22ms)
//    Total time: 35ms

@Suppress("unused")
private val test = listOf("""kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
""")
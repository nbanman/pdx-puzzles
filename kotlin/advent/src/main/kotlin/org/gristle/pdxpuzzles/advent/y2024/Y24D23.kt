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

    override fun part2(): String = runBlocking {
        val toBeat = AtomicInteger(0)
        val previous = mutableListOf<String>()
         withContext(Dispatchers.Default) {
             lan.keys
                 .map { pc ->
                     previous.add(pc)
//                     searchParty(pc, toBeat, previous.toSet())
                    async { searchParty(pc, toBeat, previous.toSet()) }
                 }
                .awaitAll()
                 .maxBy { it.size }
                 .sorted()
                 .joinToString(",")
         }
    }

    private fun searchParty(pc: String, toBeat: AtomicInteger, ignore: Set<String>): Set<String> {
        var localCandidate = emptySet<String>()
        val current = setOf(pc)
        val permitted = (lan.getValue(pc) - ignore - pc).toMutableSet()
        for (nextPc in permitted.toList()) {
            val nextCurrent = current + nextPc
            permitted.remove(nextPc)
            val next = dfs(nextPc, toBeat, nextCurrent, permitted)
            if (next.size > localCandidate.size) {
                localCandidate = next
            }
        }
        return localCandidate
    }

    private fun dfs(
        pc: String,
        toBeat: AtomicInteger,
        current: Set<String>,
        permitted: Set<String>
    ): Set<String> {
        val nextPermitted = (permitted.intersect(lan.getValue(pc)) as HashSet)
        var localCandidate = current.toSet()
        if (current.size + permitted.size <= toBeat.get()) return emptySet()
        for (nextPc in nextPermitted.toList()) {
            if (lan.getValue(nextPc).intersect(current) != current) continue
            val nextCurrent = current + nextPc
            nextPermitted.remove(nextPc)
            if (nextPermitted.isEmpty()) {
                val oldMax = toBeat.getAndUpdate { max(it, nextCurrent.size) }
                if (oldMax < nextCurrent.size) {
                    return nextCurrent
                }
            }
            val next = dfs(nextPc, toBeat, nextCurrent, permitted)
            if (next.size > localCandidate.size) localCandidate = next
        }
        return localCandidate
    }
}

// todo
// the issue here is that when

fun main() = Day.runDay(Y24D23::class)

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
package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.math.lcm
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.toGrid

object Y24D07 : Day {
    private const val TRACK1 = "=========="

    private val TRACK2 = """
    S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--
    -                                                                     -
    =                                                                     =
    +                                                                     +
    =                                                                     +
    +                                                                     =
    =                                                                     =
    -                                                                     -
    --==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-
""".trimIndent()

    private val TRACK3 = """
    S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
    - + +   + =   =     =      =   == = - -     - =  =         =-=        -
    = + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
    + + + =     +         =  + + == == ++ =     = =  ==   =   = =++=
    = = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =
    + ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
    =     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
    -               = = = =   +  +  ==+ = = +   =        ++    =          -
    -               = + + =   +  -  = + = = +   =        +     =          -
    --==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-
""".trimIndent()

    override fun part1(input: String): String {
        val knights = parseKnights(input)
        val race = TRACK1
            .map { it.toPower() }
            .asSequence()
        return knights.map { (knight, plan) -> knight to getPower(plan, race) }
            .sortedByDescending { (_, power) -> power }
            .joinToString("") { (knight) -> knight}
    }

    override fun part2(input: String): String {
        val knights = parseKnights(input)
        val track: List<Int> = parseTrack(TRACK2)
        val race: Sequence<Int> = getRace(track, 10)
        return knights.map { (knight, plan) -> knight to getPower(plan, race) }
            .sortedByDescending { (_, power) -> power }
            .joinToString("") { (knight) -> knight}
    }

    override fun part3(input: String): Int {
        val (_, opponentPlan) = parseKnights(input).first()
        val track = parseTrack(TRACK3)
        val lcm = lcm(11, track.size.toLong()).toInt()
        val adjustedLoops = lcm / track.size
        val race: Sequence<Int> = getRace(track, adjustedLoops)
        val opponentPower = getPower(opponentPlan, race)
        val plans = getPlans()

        return plans.count { plan -> getPower(plan, race) > opponentPower }
    }

    private fun parseKnights(input: String): List<Pair<String, Sequence<Int>>> = input.lines().map { line ->
        val (label, powerStr) = line.split(':')
        val power: List<Int> = powerStr.split(',').map { c -> c[0].toPower() }
        label to generateSequence(0, Int::inc).map { power[it % power.size] }
    }

    private fun getPower(plan: Sequence<Int>, race: Sequence<Int>): Long {
        val powerSeq: Sequence<Pair<Int, Int>> = race zip plan
        return powerSeq
            .runningFold(10L) { acc, (track, device) ->
                val adjust = if (track == 0) device else track
                val newAcc = (acc + adjust).coerceAtLeast(0)
                newAcc
            }.drop(1)
            .sum()
    }

    private fun parseTrack(rawLoop: String): List<Int> = buildList {
        val width = rawLoop.indexOf('\n')
        val paddedLoop = rawLoop.lines().joinToString("\n") { line -> line.padEnd(width) }
        val loop = paddedLoop.toGrid()
        val turns = listOf(Nsew::straight, Nsew::left, Nsew::right).asSequence()
        val move: (Pair<Coord, Nsew>) -> Pair<Coord, Nsew> = { (pos, dir) ->
            turns
                .map { turn ->
                    val newDir = turn(dir)
                    val newPos = pos.move(newDir)
                    newPos to newDir
                }.first { (pos) -> loop.validCoord(pos) && loop[pos] != ' ' }
        }
        generateSequence(Coord(1, 0) to Nsew.EAST, move)
            .first { (pos) ->
                val action = loop[pos]
                add(action.toPower())
                action == 'S'
            }
    }

    private fun Char.toPower() = when (this) {
        '+' -> 1
        '-' -> -1
        else -> 0
    }

    private fun getRace(loop: List<Int>, adjustedLoops: Int): Sequence<Int> = generateSequence(loop) { loop }
        .take(adjustedLoops)
        .flatMap { it }

    private fun getPlans(): List<Sequence<Int>> {
        val permutations = ArrayList<List<Int>>(9240)
        val working = ArrayList<Int>(11)
        val store = intArrayOf(3, 3, 5)

        fun traverse() {
            for (value in 0..2) {
                if (store[value] > 0) {
                    working.add(value - 1)
                    store[value]--
                    if (working.size == 11) {
                        permutations.add(working.toList())
                    } else {
                        traverse()
                    }
                    working.removeLast()
                    store[value]++
                }
            }
        }
        traverse()
        return permutations.map { permutation ->
            generateSequence(0, Int::inc).map { permutation[it % permutation.size] }
        }
    }
}

fun main() = Day.runDay(Y24D07::class)

//    Quest 1: CIEBDGAKH (3ms)
//    Quest 2: DECJFAKIG (22ms)
//    Quest 3: 5876 (866ms)
//    Total time: 892ms
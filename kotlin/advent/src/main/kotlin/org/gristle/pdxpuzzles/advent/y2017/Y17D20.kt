package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Xyz
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import kotlin.math.abs

class Y17D20(input: String) : Day {

    data class Particle(val number: Int, val p: Xyz, val v: Xyz, val a: Xyz) {
        fun stableTime(): Int {
            return maxOf(
                stableAxis(v.x, a.x),
                stableAxis(v.y, a.y),
                stableAxis(v.z, a.z),
            )
        }

        fun particleAt(time: Int): Particle {
            if (time == 0) return this
            val newV = v + a
            val newP = p + newV
            val newParticle = Particle(number, newP, newV, a)
            return newParticle.particleAt(time - 1)
        }

        private fun stableAxis(v: Int, a: Int): Int {
            return if (v * a >= 0) {
                0
            } else {
                abs(v / a) + 1
            }
        }
    }

    private val particles = input
        .getInts()
        .chunked(9)
        .mapIndexed { index, ints ->
            Particle(
                index,
                Xyz(ints[0], ints[1], ints[2]),
                Xyz(ints[3], ints[4], ints[5]),
                Xyz(ints[6], ints[7], ints[8]),
            )
        }.sortedBy { it.a.manhattanDistance() }
        .toList()

    override fun part1(): Int {
        val selectParticles = particles.filter {
            it.a.manhattanDistance() == particles.first().a.manhattanDistance()
        }
        val offset = selectParticles.maxOf { it.stableTime() }
        return selectParticles
            .maxByOrNull { it.particleAt(offset).p.manhattanDistance() }
            ?.number
            ?: throw Exception("selectParticles has no elements")
    }

    override fun part2(): Int {
        val collisionSequence = generateSequence(particles) { last ->
            val collisionsRemoved = last
                .groupBy { it.p }
                .filter { it.value.size == 1 }
                .map { it.value.first() }
            collisionsRemoved.map { it.particleAt(1) }
        }

        return collisionSequence
            .take(1000)
            .last()
            .size
    }
}

fun main() = Day.runDay(Y17D20::class)

//    Class creation: 55ms
//    Part 1: 308 (4ms)
//    Part 2: 504 (580ms)
//    Total time: 640ms
package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.lcm
import org.gristle.pdxpuzzles.utilities.objects.MCoord
import org.gristle.pdxpuzzles.utilities.objects.Xyz
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y19D12(input: String) : Day {

    data class Moon(val pos: MCoord, val vel: MCoord = MCoord(0, 0, 0)) {
        private val potentialEnergy = pos.manhattanDistance(MCoord.ORIGIN)
        private val kineticEnergy = vel.manhattanDistance(MCoord.ORIGIN)
        val totalEnergy = potentialEnergy * kineticEnergy

        override fun toString(): String {
            return "Moon(pos=$pos, vel=$vel, totalEnergy=$totalEnergy)"
        }
    }

    private val moons = input
        .getInts()
        .chunked(3) { (x, y, z) -> Moon(Xyz(x, y, z)) }
        .toList()

    private fun applyForce(a: Int, b: Int) = (a - b).let { if (it < 0) 1 else if (it > 0) -1 else 0 }

    override fun part1(): Int {
        // Part 1
        val steps = 1000
        return (1..steps).fold(moons) { acc, _ ->
            val newMoons = acc.map { moon ->
                val newVel = (acc - moon).fold(MCoord(0, 0, 0)) { acc, other ->
                    val velDelta = MCoord(
                        applyForce(moon.pos[0], other.pos[0]),
                        applyForce(moon.pos[1], other.pos[1]),
                        applyForce(moon.pos[2], other.pos[2])
                    )
                    acc + velDelta
                }
                Moon(moon.pos + moon.vel + newVel, moon.vel + newVel)
            }
            newMoons
        }.sumOf(Moon::totalEnergy)
    }

    override fun part2(): Long {
        // Part 2
        val xMap = mutableMapOf<String, Boolean>()
        val yMap = mutableMapOf<String, Boolean>()
        val zMap = mutableMapOf<String, Boolean>()
        tailrec fun periodOf(
            id: String,
            positions: List<Pair<Int, Int>>,
            register: MutableMap<String, Boolean>,
            counter: Int = 0
        ): Int {
            return if (register[id] != null) {
                counter - 1
            } else {
                register[id] = true
                val newPositions = positions.map { moon ->
                    val newVel = (positions - moon).fold(0) { acc, other ->
                        val velDelta = applyForce(moon.first, other.first)
                        acc + velDelta
                    }
                    moon.first + moon.second + newVel to moon.second + newVel
                }
                periodOf(
                    "${newPositions[0].first}/${newPositions[0].second}:${newPositions[1].first}/${newPositions[1].second}:${newPositions[2].first}/${newPositions[2].second}",
                    newPositions,
                    register,
                    counter + 1
                )
            }
        }

        val periods = listOf(
            periodOf(
                "${moons[0].pos[0]}/${moons[0].vel[0]}:${moons[1].pos[0]}/${moons[1].vel[0]}:${moons[2].pos[0]}/${moons[2].vel[0]}:${moons[3].pos[0]}/${moons[3].vel[0]}",
                moons.map { it.pos[0] to it.vel[0] },
                xMap
            ),
            periodOf(
                "${moons[0].pos[1]}/${moons[0].vel[1]}:${moons[1].pos[1]}/${moons[1].vel[1]}:${moons[2].pos[1]}/${moons[2].vel[1]}:${moons[3].pos[1]}/${moons[3].vel[1]}",
                moons.map { it.pos[1] to it.vel[1] },
                yMap
            ),
            periodOf(
                "${moons[0].pos[2]}/${moons[0].vel[2]}:${moons[1].pos[2]}/${moons[1].vel[2]}:${moons[2].pos[2]}/${moons[2].vel[2]}:${moons[3].pos[2]}/${moons[3].vel[2]}",
                moons.map { it.pos[2] to it.vel[2] },
                zMap
            )
        ).map { it.toLong() }

        return lcm(periods)
    }
}

fun main() = Day.runDay(Y19D12::class)

//    Class creation: 20ms
//    Part 1: 10028 (38ms)
//    Part 2: 314610635824376 (701ms)
//    Total time: 759ms
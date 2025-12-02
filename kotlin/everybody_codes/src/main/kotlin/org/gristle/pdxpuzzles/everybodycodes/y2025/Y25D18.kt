package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit
import org.gristle.pdxpuzzles.utilities.parsing.getLongs

object Y25D18 : Day {
    data class Plant(val thickness: Long, val branches: Branches) {
        fun getEnergy(plants: List<Plant>, plantEnergy: MutableList<Long?>): Long = when (branches) {
            Free -> 1L
            is ToPlants -> {
                var energy = 0L
                for (branch in branches.value) {
                    val sourceEnergy = plantEnergy[branch.toPlant]
                        ?: let {
                            val branchEnergy = plants[branch.toPlant].getEnergy(plants, plantEnergy)
                            plantEnergy[branch.toPlant] = branchEnergy
                            branchEnergy
                        }
                    energy += sourceEnergy * branch.thickness
                }
                if (energy >= thickness) {
                    energy
                } else {
                    0
                }
            }
        }



        companion object {
            fun from(notes: String): Plant {
                val thickness = notes.getLongs().drop(1).first()
                val branches = Branches.from(notes.dropWhile { it != '\n' })
                return Plant(thickness, branches)
            }
        }
    }

    sealed interface Branches {
        companion object {
            fun from(value: String): Branches = if (value.startsWith("\n- f")) {
                Free
            } else {
                val branches = value.getLongs().chunked(2)
                    .map { (plant, thickness) ->
                        Branch(thickness, plant.toInt() - 1)
                    }.toList()
                ToPlants(branches)
            }
        }
    }

    data object Free : Branches

    @JvmInline
    value class ToPlants(val value: List<Branch>) : Branches

    data class Branch(val thickness: Long, val toPlant: Int)

    private fun getPlants(notes: String): List<Plant> = notes.blankSplit().map(Plant::from)

    override fun part1(input: String): Long {
        val plants = getPlants(input)
        val plantEnergy: MutableList<Long?> = MutableList(plants.size) { null }
        return plants.last().getEnergy(plants, plantEnergy)
    }

    override fun part2(input: String): Long {
        val (plantsStr, testCases) = input.split("\n\n\n")
        val plants = getPlants(plantsStr)

        return testCases.lineSequence().sumOf { line ->
            val iter = line.getLongs().iterator()
            val plantEnergy: MutableList<Long?> = MutableList(plants.size) {
                if (iter.hasNext()) iter.next() else null
            }
            plants.last().getEnergy(plants, plantEnergy)
        }
    }

    override fun part3(input: String): Long {
        val (plantsStr, testCases) = input.split("\n\n\n")
        val plants = getPlants(plantsStr)

        val plantEnergy: MutableList<Long?> = MutableList(plants.size) { null }

        val phase2Start = plants.indexOfFirst { (_, branches) -> branches is ToPlants }

        for (i in 0 until phase2Start) plantEnergy[i] = 0L

        val positiveBranchPlants = plants.asSequence()
            .mapNotNull { (_, branches) ->
                when (branches) {
                    Free -> null
                    is ToPlants -> branches.value
                }
            }.takeWhile { branches -> branches.size > 2 }
            .flatMap { branches ->
                branches
                    .filter { branch -> branch.thickness > 0 }
                    .map { branch -> branch.toPlant }
            }.distinct()
            .sorted()

        for (plantIdx in positiveBranchPlants) {
            plantEnergy[plantIdx] = 1L
        }

        val optimum = plants.last().getEnergy(plants, plantEnergy)

        return testCases.lineSequence().sumOf { line ->
            val iter = line.getLongs().iterator()
            val plantEnergy: MutableList<Long?> = MutableList(plants.size) {
                if (iter.hasNext()) iter.next() else null
            }
            val energy = plants.last().getEnergy(plants, plantEnergy)
            if (energy == 0L) {
                0L
            } else {
                optimum - energy
            }
        }
    }
}

fun main() = Day.runDay(Y25D18::class)

//    Quest 1: 2067316 (3ms)
//    Quest 2: 15481956620 (7ms)
//    Quest 3: 485271 (6ms)
//    Total time: 17ms
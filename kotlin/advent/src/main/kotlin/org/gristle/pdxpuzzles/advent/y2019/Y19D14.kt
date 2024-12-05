package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.advent.y2019.Y19D14.Chemical.Companion.lookup
import org.gristle.pdxpuzzles.utilities.parsing.groupValues
import kotlin.math.max

class Y19D14(private val input: String) : Day {
    private val pattern = """(\d+) ([A-Z]+)""".toRegex()

    data class Chemical(val name: String, val quantity: Long) {
        companion object {
            val lookup = mutableMapOf<String, MutableSet<Chemical>>()
        }

        val upstream: Set<Chemical> by lazy {
            val upSet = mutableSetOf<Chemical>()
            val immediateUp = lookup[name] ?: return@lazy emptySet<Chemical>()
            upSet.addAll(immediateUp)
            for (chem in immediateUp) {
                upSet.addAll(chem.upstream)
            }
            upSet
        }
    }
    data class Reaction(val result: Chemical, val ingredients: List<Chemical>)

    fun solve(): Pair<Long, Long> {
        val rules = buildMap {
            input
                .lines()
                .map { line ->
                    line
                        .groupValues(pattern)
                        .let { gvs ->
                            val result = Chemical(gvs.last()[1], gvs.last()[0].toLong())
                            val ingredients = gvs.dropLast(1).map { gv ->
                                Chemical(gv[1], gv[0].toLong())
                            }
                            val reaction = Reaction(result, ingredients)
                            for (ingredient in reaction.ingredients) {
                                if (lookup[ingredient.name] == null) {
                                    lookup[ingredient.name] = mutableSetOf()
                                }
                                lookup.getValue(ingredient.name).add(result)
                            }
                            put(result.name, reaction)
                        }
                }
        }

        fun calculateOre(fuel: Long): Long {
            val repository = mutableMapOf("FUEL" to fuel)
            var potentials = repository.entries.filter { it.value != 0L }
            var ore = 0L

            while (potentials.isNotEmpty()) {

                val potential = potentials.find { pot ->
                    rules.getValue(pot.key).result.upstream.all { chem ->
                        chem.name !in potentials.map { it.key }
                    }
                } ?: break

                val rule = rules.getValue(potential.key)
                val timesApplied = (potential.value / rule.result.quantity) +
                        if (potential.value % rule.result.quantity > 0) 1 else 0
                repository[potential.key] = max(
                    0,
                    repository.getValue(potential.key) - rule.result.quantity * timesApplied
                )
                for (ingredient in rule.ingredients) {
                    repository[ingredient.name] =
                        (repository[ingredient.name] ?: 0) + ingredient.quantity * timesApplied
                }

                ore += repository["ORE"] ?: 0L
                repository["ORE"] = 0L
                potentials = repository.entries.filter { it.value != 0L }
            }
            return ore
        }

        val oneFuel = calculateOre(1)

        // Part 2
        val totalOre = 1_000_000_000_000L
        val lowerBound = totalOre / oneFuel
        val test = calculateOre(lowerBound)
        val guess = (lowerBound * totalOre) / test
        return oneFuel to guess
    }

    private val solution = solve()

    override fun part1() = solution.first

    override fun part2() = solution.second
}

fun main() = Day.runDay(Y19D14::class)

//    Class creation: 40ms
//    Part 1: 751038 (0ms)
//    Part 2: 2074843 (0ms)
//    Total time: 41ms

package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.groupValues

class Y18D24(input: String) : Day {
    private val data = input.split("Infection:\n")
    private val pattern =
        """(\d+) units each with (\d+) hit points (?:\(([^)]+)\) )?with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)""".toRegex()
    private val patternWeak = """weak to ((?:[a-z]+(?:, )?)+)""".toRegex()
    private val patternImmune = """immune to ((?:[a-z]+(?:, )?)+)""".toRegex()

    private fun selectionOrder(boost: Int) =
        compareByDescending<ArmyUnit> { it.effectivePower(boost) }.thenByDescending { it.initiative }

    data class ArmyUnit(
        val team: String,
        val group: Int,
        var units: Int,
        val hp: Int,
        val immunities: List<String>,
        val weaknesses: List<String>,
        val damage: Int,
        val damageType: String,
        val initiative: Int
    ) {
        fun effectivePower(boost: Int = 0): Int = units * (damage + if (team == "Immune System") boost else 0)

        fun modifiedDamage(other: ArmyUnit, boost: Int = 0): Int = when (other.damageType) {
            in weaknesses -> other.effectivePower(boost) * 2
            in immunities -> 0
            else -> other.effectivePower(boost)
        }

        fun takeDamage(damage: Int): Boolean {
            val reduction = damage / hp
            return if (reduction == 0) {
                false
            } else {
                units -= damage / hp
                true
            }
        }
    }

    private fun selectTargets(
        attackers: List<ArmyUnit>,
        defenders: List<ArmyUnit>,
        boost: Int = 0
    ): List<Pair<ArmyUnit, ArmyUnit>> {
        val attackerSelections = mutableListOf<Pair<ArmyUnit, ArmyUnit>>()

        // take all defenders and rank them by their effective power, with initiative as a tiebreaker
        val defenders = defenders
            .toMutableList()
            .apply {
                sortWith(
                    compareByDescending<ArmyUnit> { it.effectivePower(boost) }
                        .thenByDescending { it.initiative }
                )
            }

        // assign a defender to each attacker as follows:
        // all the tiebreakers are already sorted, so start at the top and grab the first defender who is
        // weak to attacker's damage type. If none such defender exists, grab the first defender who is
        // not immune to attacker's damage type.
        // Assuming such a defender is found, add the attacker/defender pair and remove the defender for
        // future consideration by other attackers.
        for (attacker in attackers) {
            defenders
                .withIndex()
                .let {
                    it.find { (_, defender) -> attacker.damageType in defender.weaknesses }
                        ?: it.find { (_, defender) -> attacker.damageType !in defender.immunities }
                }
                ?.let { (idx, defender) ->
                    attackerSelections.add(attacker to defender)
                    defenders.removeAt(idx)
                }
        }
        return attackerSelections
    }

    private fun makeUnits(team: String, s: String, groupStart: Int): List<ArmyUnit> = s
        .groupValues(pattern)
        .mapIndexed { index, gv ->
            val units = gv[0].toInt()
            val hp = gv[1].toInt()
            val immunities = patternImmune.find(gv[2])?.groupValues?.get(1)?.split(", ") ?: emptyList()
            val weaknesses = patternWeak.find(gv[2])?.groupValues?.get(1)?.split(", ") ?: emptyList()
            val damage = gv[3].toInt()
            val damageType = gv[4]
            val initiative = gv[5].toInt()
            ArmyUnit(team, index + groupStart, units, hp, immunities, weaknesses, damage, damageType, initiative)
        }

    fun solve(startingBoost: Int, predicate: (List<ArmyUnit>) -> Boolean): Int {
        // play rounds
        var boost = startingBoost
        loop@while (true) {
            var immuneSystem = makeUnits("Immune System", data.first(), 0)
            var infection = makeUnits("Infection", data.last(), immuneSystem.size)

            while (immuneSystem.isNotEmpty() && infection.isNotEmpty()) {
                // target selection phase
                val immuneSelections = selectTargets(immuneSystem.sortedWith(selectionOrder(boost)), infection, boost)
                val infectionSelections = selectTargets(infection.sortedWith(selectionOrder(boost)), immuneSystem, boost)

                // attack phase
                val attackers = (immuneSelections + infectionSelections)
                    .sortedByDescending { (attacker) -> attacker.initiative }

                // changed tracks whether a defender actually loses units in a turn. if this stays false,
                // we know it's an unchanging loop and we can exit early.
                var changed = false
                for ((attacker, defender) in attackers) {
                    changed = changed or defender.takeDamage(defender.modifiedDamage(attacker, boost))
                }

                // cleanup phase
                immuneSystem = immuneSystem.filter { it.units > 0 }
                infection = infection.filter { it.units > 0 }

                // exit early if no units were killed
                if (!changed) {
                    boost++
                    continue@loop
                }
            }
            if (predicate(infection)) {
                return (immuneSystem + infection).sumOf { it.units }
            }
            boost++
        }
    }

    override fun part1() = solve(0) { true }
    override fun part2() = solve(1) { it.isEmpty() }
}

fun main() = Day.runDay(Y18D24::class)

//    Class creation: 2ms
//    Part 1: 15165 (16ms)
//    Part 2: 4037 (110ms)
//    Total time: 129ms
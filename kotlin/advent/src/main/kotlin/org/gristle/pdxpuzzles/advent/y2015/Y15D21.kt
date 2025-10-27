package org.gristle.pdxpuzzles.advent.y2015

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.combinatorics.getPairSequence
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit
import org.gristle.pdxpuzzles.utilities.parsing.getIntList
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y15D21(input: String) : Day {

    companion object {

        const val ITEM_LIST = """
            Weapons:    Cost  Damage  Armor
            Dagger        8     4       0
            Shortsword   10     5       0
            Warhammer    25     6       0
            Longsword    40     7       0
            Greataxe     74     8       0

            Armor:      Cost  Damage  Armor
            Leather      13     0       1
            Chainmail    31     0       2
            Splintmail   53     0       3
            Bandedmail   75     0       4
            Platemail   102     0       5

            Rings:      Cost  Damage  Armor
            Damage +1    25     1       0
            Damage +2    50     2       0
            Damage +3   100     3       0
            Defense +1   20     0       1
            Defense +2   40     0       2
            Defense +3   80     0       3
        """
    }

    data class Character(val hp: Int, val damage: Int, val armor: Int) {
        fun roundsToKill(other: Character): Int {
            val adjustedDamage = maxOf(1, damage - other.armor)
            return other.hp / adjustedDamage + if (other.hp % adjustedDamage == 0) 0 else 1
        }
    }

    data class Stats(val cost: Int, val damage: Int, val armor: Int) {
        fun toCharacter() = Character(100, damage, armor)
    }

    operator fun Stats.plus(other: Stats): Stats =
        Stats(cost + other.cost, damage + other.damage, armor + other.armor)

    val loadouts: List<Stats> = let {
        val (weaponShop, armorShop, ringShop) = ITEM_LIST.blankSplit()
        val armor = armorShop
            .getInts()
            .chunked(3) { (cost, damage, armor) -> Stats(cost, damage, armor) }
            .plus(Stats(0, 0, 0))
            .toList()
        val rings = ringShop
            .getInts()
            .chunked(4) { (_, cost, damage, armor) -> Stats(cost, damage, armor) }
            .plus(Stats(0, 0, 0))
            .toList()
        weaponShop
            .getInts()
            .chunked(3) { (cost, damage, armor) -> Stats(cost, damage, armor) }
            .flatMap { weaponized -> armor.map { weaponized + it } }
            .flatMap { armored ->
                val doubleRings = rings.dropLast(1)
                    .getPairSequence()
                    .map { (ring, middle) -> ring + middle }
                rings
                    .asSequence()
                    .plus(doubleRings)
                    .map { beRinged -> beRinged + armored }
            }.sortedBy { (cost) -> cost }
            .toList()
    }

    // Generate boss from input stats
    private val boss = input
        .getIntList()
        .let { (hp, damage, armor) -> Character(hp, damage, armor) }

    override fun part1() = loadouts
        .first { stats ->
            val player = stats.toCharacter()
            player.roundsToKill(boss) <= boss.roundsToKill(player)
        }.cost

    override fun part2() = loadouts
        .last { stats ->
            val player = stats.toCharacter()
            player.roundsToKill(boss) > boss.roundsToKill(player)
        }.cost
}

fun main() = Day.runDay(Y15D21::class)

//    Class creation: 7ms
//    Part 1: 91 (1ms)
//    Part 2: 158 (2ms)
//    Total time: 11ms
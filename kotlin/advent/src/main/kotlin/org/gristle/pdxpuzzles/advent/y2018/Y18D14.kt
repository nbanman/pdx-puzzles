package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y18D14(input: String) : Day {

    // elves' plan as an Int for pt1
    private val plan = input.toInt()

    // elves' plan as a List<Int> for pt2
    private val planList = input.map { it.digitToInt() }

    // function to advance the state in the below sequence
    private fun advance(state: Triple<MutableList<Int>, Int, Int>): Triple<MutableList<Int>, Int, Int> {
        // deconstructs state
        val (recipes, elf1, elf2) = state

        // gets ratings from recipes list at each elf's location
        val elf1Rating = recipes[elf1]
        val elf2Rating = recipes[elf2]

        // adds new recipes to recipes per rules
        val sum = elf1Rating + elf2Rating
        if (sum > 9) {
            recipes.add(1)
            recipes.add(sum - 10)
        } else {
            recipes.add(sum)
        }

        // returns new state
        return Triple(
            recipes,
            (elf1 + 1 + elf1Rating) % recipes.size,
            (elf2 + 1 + elf2Rating) % recipes.size
        )
    }

    // starts with initial recipes and positions and advances until told to stop
    private fun recipeSequence() = generateSequence(Triple(mutableListOf(3, 7), 0, 1), ::advance)

    override fun part1() = recipeSequence()
        .first { (recipes) -> recipes.size == plan + 10 } // stop when enough recipes have been generated
        .let { (recipes) ->
            recipes.takeLast(10).joinToString("").toLong() // return last 10 recipe ratings
        }

    override fun part2() = recipeSequence()
        .first { (recipes) -> // stop when plan found at end, or end - 1, of recipes

            // need to make sure that there are at least as many recipes as the plan calls for
            // also, need to check both the end, and the end - 1, because each step can add 0, 1, or 2 new recipes
            recipes.size > planList.size + 1 &&
                    (planList == recipes.subList(recipes.size - planList.size - 1, recipes.size - 1) ||
                            planList == recipes.subList(recipes.size - planList.size, recipes.size))
        }.let { (recipes) -> // return the number of recipes to the left

            // check if the plan is found in ultimate or penultimate position, subtract one if penultimate
            if (planList == recipes.subList(recipes.size - planList.size, recipes.size)) {
                recipes.size - planList.size
            } else {
                recipes.size - planList.size - 1
            }
        }
}

fun main() = Day.runDay(Y18D14::class)

//    Class creation: 2ms
//    Part 1: 4910101614 (61ms)
//    Part 2: 20253137 (1120ms)
//    Total time: 1185ms
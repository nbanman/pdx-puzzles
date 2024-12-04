package org.gristle.pdxpuzzles.advent.y2022

import kotlinx.coroutines.*
import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.get0
import org.gristle.pdxpuzzles.utilities.parsing.getInts
import org.gristle.pdxpuzzles.utilities.parsing.gvs
import kotlin.math.ceil

// I refactored and now it's 3x slower!
// I "borrowed" (i.e., stole) heavily from ephemient's solution: 
// https://github.com/ephemient/aoc2022/blob/main/kt/src/commonMain/kotlin/com/github/ephemient/aoc2022/Day19.kt
// See previous commit for my (3x faster!) solution, though TBH that version has ephemient written all over it as
// well, particularly in the coroutines.

// This version relies heavily on Maps to store all the robot, resource, and relationship information. It is slow
// because the State has to reconstruct two maps for each branch, rather than simply update a couple fields as my
// previous version did.

// This version is a more general solution. My original version relied on the fact that the resources are always the 
// same and that for each blueprint every robot requires the same kinds of resources, just in different amounts. 
// This version is agnostic as to the names and different types of resources.

// This version is also significantly shorter because I do not need to handle special cases for my hard-coded types. 

@Suppress("SameParameterValue")
class Y22D19(input: String) : Day {

    data class Blueprint(
        val id: Int,
        val robotCosts: Map<String, Map<String, Int>>,
    ) {
        // maxMaterial is used to determine whether enough robots of a particular kind have been created that 
        // any robot requiring that material can make a new robot each turn, such that no more robots of that
        // kind need to ever be made.
        val maxMaterial: Map<String, Int> = robotCosts.keys.mapNotNull { resource -> // for each resource...
            // This takes the resource, goes back to each entry in robotCosts, and finds the cost for that resource
            // Then it returns the max value. If the particular robot does not require that resource, then the
            // .mapNotNull function excludes it because the map get function returns null. If no robot requires the
            // resource, then the .maxOrNull function returns null instead of throwing an Exception. 
            val maxMaterial = robotCosts.values
                .mapNotNull { resourceMap -> resourceMap[resource] }
                .maxOrNull()
            // if maxMaterial is null then no robot requires that material and no entry need be created
            // otherwise, map to Pair<String, Int> for conversion to Map.
            if (maxMaterial == null) null else resource to maxMaterial
        }.toMap()
    }

    private val blueprints: List<Blueprint>

    init { // parses input into a list of blueprint objects

        // Regex to that finds relationships
        val pattern = Regex("""Each ([a-z]+) robot costs (\d+) ([a-z]+)(?: and (\d+) ([a-z]+))?. ?""")

        blueprints = input.lines().map { spec ->
            val id = spec.getInts().first() // id is the first number in the spec
            val robotCosts: Map<String, Map<String, Int>> = buildMap {
                spec.gvs(pattern).forEach { (robotType, cost1, resource1, cost2, resource2) ->
                    val resourceMap: Map<String, Int> = buildMap {
                        put(resource1, cost1.toInt())
                        if (resource2.isNotBlank()) put(resource2, cost2.toInt())
                    }
                    put(robotType, resourceMap)
                }
            }
            Blueprint(id, robotCosts)
        }
    }

    data class State(
        val minute: Int = 0,
        val resources: Map<String, Int>,
        val robots: Map<String, Int>,
    ) {
        // Delivers list of next possible states.
        fun nextStates(blueprint: Blueprint, minutes: Int, cutoff: Map<String, Int>): List<State> =
            blueprint.robotCosts.keys
                .mapNotNull { robotType ->
                    if ((robots.get0(robotType)) == (blueprint.maxMaterial[robotType] ?: Int.MAX_VALUE)) {
                        null
                    } else {
                        if (minutes - minute - cutoff.getValue(robotType) < 0) {
                            null
                        } else {
                            val buildTime = buildTime(blueprint, robotType)
                            if (minutes - minute - buildTime < 0) {
                                null
                            } else {
                                val newRobots = robots + (robotType to robots.get0(robotType) + 1)
                                val newResources: Map<String, Int> = buildMap {
                                    putAll(resources)
                                    val costs = blueprint.robotCosts.getValue(robotType)
                                    forEach { (component, cost) ->
                                        this[component] = cost -
                                                costs.get0(component) +
                                                robots.get0(component) * buildTime
                                    }
                                }
                                State(
                                    minute = minute + buildTime,
                                    resources = newResources,
                                    robots = newRobots,
                                )
                            }
                        }
                    }
            }

        // Calculates the time needed to build a particular robot. It cycles through each component resource required
        // to make the robot. For each component, it finds out how many resources are already available. If enough
        // are available, assuming unlimited resources in the other components, the build time is 1. 
        // If not enough are available, it finds out how many robots building that component there are and then 
        // calculates how many turns are needed to harvest enough to build the robot. 
        private fun buildTime(blueprint: Blueprint, robotType: String): Int =
            blueprint.robotCosts.getValue(robotType).maxOf { (component, cost) ->
                val resourcesAvailable = resources[component] ?: 0
                if (cost <= resourcesAvailable) {
                    1
                } else {
                    val robotsAvailable = robots.get0(component)
                    if (robotsAvailable == 0) {
                        Int.MAX_VALUE
                    } else {
                        ceil((cost - resourcesAvailable) / robotsAvailable.toFloat()).toInt() + 1
                    }
                }
            }

        // This is a crude but fast calculation to see if the current State can possibly overtake the current leader.
        // It assumes that until time runs out, one robot of the resource we want can be produced each turn.
        fun maxBound(minutes: Int, resource: String): Int {
            val currentAmount = resources[resource] ?: 0
            val currentRobotNum = robots[resource] ?: 0
            return currentAmount + (0 until (minutes - minute)).sumOf { it + currentRobotNum }
        }

        // This is a crude but fast calculation of the minimum number of a resource the State will produce before 
        // time runs out. 
        // It assumes that until time runs out, *no* robots of the resource we want can be produced.
        fun minBound(minutes: Int, resource: String): Int {
            val currentAmount = resources[resource] ?: 0
            val currentRobotNum = robots[resource] ?: 0
            return currentAmount + (minutes - minute) * currentRobotNum
        }
    }

    // Main loop uses a stack. Starting with initial state, it starts branching. For each state, it calculates
    // the lower and upper bounds, storing the highest lower bound. If the upper bound is lower than the highest
    // lower bound, it is discarded.
    private fun findResource(blueprint: Blueprint, resource: String, initialState: State, minutes: Int): Int {
        // create cutoff map for optimization
        val cutoff = blueprint.robotCosts.keys.associateWith { robotType ->
            when {
                robotType == resource -> 1
                blueprint.robotCosts.getValue(resource).containsKey(robotType) -> 3
                else -> 5
            }
        }

        var maxGeodes = 0
        val queue = ArrayDeque<State>()
        queue.add(initialState)
        while (queue.isNotEmpty()) {
            val state = queue.removeLast()
            if (state.maxBound(minutes, resource) < maxGeodes) continue
            val minGeodes = state.minBound(minutes, resource)
            if (minGeodes > maxGeodes) maxGeodes = minGeodes
            queue.addAll(state.nextStates(blueprint, minutes, cutoff))
        }

        return maxGeodes
    }

    private val initialState = State(
        robots = mapOf("ore" to 1),
        resources = mapOf(
            "ore" to 0,
            "clay" to 0,
            "obsidian" to 0,
            "geode" to 0,
        )
    )

    override fun part1() = runBlocking {
        withContext(Dispatchers.Default) {
            blueprints.map { blueprint ->
                async {
                    blueprint.id * findResource(blueprint, "geode", initialState, 24)
                }
            }.sumOf { it.await() }
        }
    }

    override fun part2() = runBlocking {
        withContext(Dispatchers.Default) {
            blueprints
                .take(3)
                .map { blueprint ->
                    async { findResource(blueprint, "geode", initialState, 32) }
                }.awaitAll()
                .reduce(Int::times)
        }
    }
}

// Pt 1: 1427 (500ms) (629ms) (146ms hardcoded)
// Pt 2: 4400 (614ms) (807ms) (238ms hardcoded)
// Total: (1173ms) (1490ms) (408ms hardcoded)
fun main() = Day.runDay(Y22D19::class)

//    Class creation: 32ms
//    Part 1: 1427 (565ms)
//    Part 2: 4400 (777ms)
//    Total time: 1376ms

package org.gristle.pdxpuzzles.advent.y2016

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.algorithms.Graph
import org.gristle.pdxpuzzles.utilities.algorithms.Graph.steps
import org.gristle.pdxpuzzles.utilities.parsing.groupValues

class Y16D11(private val input: String) : Day {

    private val pattern = """(\w+)(?: |-compatible )(generator|microchip)""".toRegex()

    /*
    * The state, showing what's on each floor and where the elevator is.
    */
    private data class FloorState(val elevator: Int, val floors: List<Floor>) {

        fun isValid() = floors.all(Floor::isValid) // Checks that all floors are valid
        fun isSolved() = toHash().drop(1).dropLast(2).all { it == '0' } // Checks if all items on top floor

        // Gives "hash" of the state, which is just numbers corresponding to the elevator position + number of items
        // on each floor. Since items are interchangeable and all microchips must be matched where there are
        // generators, all similarly hashed states are functionally identical.
        fun toHash() = "$elevator${floors.joinToString("") { it.toHash() }}"

        // Generates the list of possible floor states after a valid move.
        fun validStates(): List<FloorState> {
            val vm = mutableListOf<FloorState>() // Holder for states to return

            // List of floor numbers the elevator can move to. Can go up or down, but can't go below 0 or above the top.
            val potentialFloorNumbers = listOf(elevator - 1, elevator + 1).filter { it in 0..floors.lastIndex }

            // Gets all potential loads the elevator can carry by getting combinations of all the items on the floor
            // then checking to see if the remaining items on the floor cause any chips to fry.
            val potentialLoads = getPotentialMoves(floors[elevator])
                .filter { Floor(floors[elevator].items - it.items.toSet()).isValid() } // check for frying

            // Nested loop that tries all valid loads on all potential floors. If the floor with the new load is valid,
            // add it to the list of potential states.
            for (pfn in potentialFloorNumbers) {
                for (pm in potentialLoads) {
                    // This generates the floor layout of the state by taking the original state, then making
                    // modifications to the floor that the load is being moved to as well as the floor that the load
                    // is taken from.
                    val newFloors = floors.mapIndexed { index, floor ->
                        when (index) {
                            elevator -> Floor(floors[index].items - pm.items.toSet()) // remove items from floor
                            pfn -> Floor(floors[index].items + pm.items) // add items to floor
                            else -> floor // keep floor unchanged
                        }
                    }
                    // take the floor layout and new elevator position and create a new state
                    val newState = FloorState(pfn, newFloors)
                    // if the new state is valid on all floors, add it.
                    if (newState.isValid()) vm.add(newState)
                }
            }
            return vm
        }

        // Gets combinations of potential items to move, with at least one item and at most two items.
        fun getPotentialMoves(floor: Floor): List<Floor> {
            return floor.items.foldIndexed(listOf()) { index, acc, radioItem ->
                acc + Floor(listOf(radioItem)) + floor.items.drop(index + 1).map { Floor(listOf(radioItem, it)) }
            }
        }
    }

    private enum class ItemType { MICROCHIP, GENERATOR }

    private data class RadioItem(val name: String, val type: ItemType)

    private data class Floor(val items: List<RadioItem>) {
        val microchips = items.filter { it.type == ItemType.MICROCHIP }.map { it.name }
        val generators = items.filter { it.type == ItemType.GENERATOR }.map { it.name }
        fun isValid() = microchips.isEmpty() || generators.isEmpty() || (generators.containsAll(microchips))
        fun toHash() = "${microchips.size}${generators.size}"
    }

    // Runs a regex and creates the initial floor state from it. Each line in the input is a 'floor.' Each
    private fun parseFloors(part2: Boolean): FloorState {
        val floors = input
            .split('\n')
            .mapIndexed { index, line ->
                line
                    .groupValues(pattern)
                    .map { (name, typeStr) ->
                        val type = if (typeStr == "microchip") ItemType.MICROCHIP else ItemType.GENERATOR
                        RadioItem(name, type)
                    }.let {
                        if (index == 0 && part2) it + listOf(
                            RadioItem("elerium", ItemType.GENERATOR),
                            RadioItem("elerium", ItemType.MICROCHIP),
                            RadioItem("dilithium", ItemType.GENERATOR),
                            RadioItem("dilithium", ItemType.MICROCHIP),
                        ) else it
                    }.let { Floor(it) }
            }
        return FloorState(0, floors)
    }

    /*
    * Solver
    */
    private fun solveFloors(initialState: FloorState): Int {

        // cache used to prune previously visited states. The standard visit check done by the library function is
        // insufficient because there are many states that involve different arrangement of specific items but are
        // functionally equivalent.
        val stateHashes = mutableSetOf(initialState.toHash())
        val getNeighbors = { state: FloorState ->
            state
                .validStates()
                .filter {
                    val unvisited =
                        !stateHashes.contains(it.toHash()) // checks if functionally equivalent state unvisited
                    if (unvisited) stateHashes.add(it.toHash()) // if unvisited add it to the cache
                    unvisited
                }
        }

        /*
        * breadth-first search through all possible states, pruning states that are functionally identical to previous 
        * states.
        */
        return Graph
            .bfs(startId = initialState, endCondition = FloorState::isSolved, defaultEdges = getNeighbors)
            .steps()
    }

    override fun part1() = solveFloors(parseFloors(false))

    override fun part2() = solveFloors(parseFloors(true))
}

fun main() = Day.runDay(Y16D11::class)

// (680ms custom BFS) (210ms library BFS)
// (9776ms custom BFS) (409ms library BFS)

//    Class creation: 10ms
//    Part 1: 47 (152ms)
//    Part 2: 71 (421ms)
//    Total time: 583ms
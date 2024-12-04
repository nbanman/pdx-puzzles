package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.graph.Graph
import org.gristle.pdxpuzzles.utilities.iteration.pollUntil
import org.gristle.pdxpuzzles.utilities.parsing.groupValues
import java.util.*
import kotlin.math.max
import kotlin.math.min

class Y22D16(input: String) : Day {
    private val flowMap: Map<String, Int>
    private val edgeMap: Map<String, Map<String, Int>>

    init {
        val edgeMapNoValves = mutableMapOf<String, List<Graph.Edge<String>>>()

        // parse to edgeMapNoValves and flowRate maps
        flowMap = buildMap {
            val pattern = """Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)""".toRegex()
            input.groupValues(pattern).forEach { (name, flowRate, tunnelString) ->
                val tunnels = tunnelString.split(", ").map { Graph.Edge(it, 1.0) }
                this[name] = flowRate.toInt()
                edgeMapNoValves[name] = tunnels
            }
        }

        // get point to point info on all the points, getting rid of 0-flow valve locations
        edgeMapNoValves
            .filter { (valve, _) -> valve == "AA" || flowMap.getValue(valve) > 0 }
            .forEach { (valve, _) ->
                edgeMapNoValves[valve] = Graph.dijkstra(valve, edges = edgeMapNoValves)
                    .filter { it.id != "AA" && flowMap.getValue(it.id) > 0 && it.weight != 0.0 }
                    .map { Graph.Edge(it.id, it.weight) }
            }

        // clean up the map
        edgeMap = edgeMapNoValves
            .filter { it.key == "AA" || flowMap.getValue(it.key) > 0 }
            .map { it.key to it.value.associate { edge -> edge.vertexId to edge.weight.toInt() + 1 } }
            .associate { it }
    }

    /**
     * Tracks everything. Note that total can be negative to avoid double-counting things that do not happen until
     * the other agent moves.
     */
    data class State(
        val pos: List<Pair<String, Int>>,
        val valves: Set<String>,
        val flow: Int,
        val total: Int,
        val minute: Int,
    )

    fun solve(agents: Int, minutes: Int): Int {
        var max = 0

        val startId = State(
            pos = List(agents) { "AA" to 0 },
            valves = edgeMap.getValue("AA").keys,
            flow = 0,
            total = 0,
            minute = minutes + 1,
        )

        val open = PriorityQueue<Pair<State, Int>>(compareByDescending { it.second })
        open.add(startId to 0)

        val closed = mutableSetOf<State>()

        while (open.isNotEmpty()) {
            val (current, heuristic) = open.pollUntil { (state, _) -> !closed.contains(state) } ?: break
            val firstRoom = current.pos.first()
            val secondRoom = current.pos.last()
            closed.add(current)

            val potentialFuture = heuristic + current.valves.sumOf { valve ->
                val flowRate = flowMap.getValue(valve)
                current.pos.maxOf { (room, timeOffset) ->
                    flowRate * max(0, (current.minute + timeOffset - edgeMap.getValue(room).getValue(valve)))
                }
            }

            if (heuristic > max) max = heuristic

            if (potentialFuture < max) continue

            val distanceMap = current.valves.associateWith { valve ->
                current
                    .pos
                    .mapIndexed { roomNo, (room, timeOffset) ->
                        roomNo to edgeMap.getValue(room).getValue(valve) - timeOffset
                    }.minBy { it.second }
            }

            val newStates = distanceMap.map { (valve, roomInfo) ->
                val (roomNo, distance) = roomInfo
                val newPos = run {
                    if (current.pos.size == 1) {
                        listOf(valve to 0)
                    } else {
                        if (roomNo == 0) { // first room moving
                            val newFirstOffset = min(0, distance)
                            val newSecondOffset = max(0, distance)
                            listOf(
                                valve to newFirstOffset,
                                secondRoom.first to newSecondOffset
                            )
                        } else {
                            val newFirstOffset = max(0, distance)
                            val newSecondOffset = -min(0, distance)
                            listOf(
                                firstRoom.first to newFirstOffset,
                                valve to newSecondOffset
                            )
                        }
                    }
                }
                val newFlow = current.flow + flowMap.getValue(valve)
                val newMinute = min(current.minute - distance, current.minute)
                val newState = State(
                    pos = newPos,
                    valves = current.valves - valve,
                    flow = newFlow,
                    total = current.total + current.flow * distance,
                    minute = newMinute
                )
                val newHeuristic = heuristic + flowMap.getValue(valve) * (newMinute - 1)

                newState to newHeuristic
            }
            open.addAll(newStates)
        }

        return max
    }

    override fun part1() = solve(1, 30)

    override fun part2() = solve(2, 26)
}

fun main() = Day.runDay(Y22D16::class)

//    Class creation: 29ms
//    Part 1: 2059 (178ms)
//    Part 2: 2790 (6318ms)
//    Total time: 6526ms
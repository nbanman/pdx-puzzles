package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.toGrid
import java.util.*

class Y19D20(input: String) : Day {
    private enum class Side { INNER, OUTER }
    private data class Portal(val name: String, val side: Side)
    private data class State(val pos: Int, val level: Int)
    private data class EdgeInfo(val state: State, val dist: Int)
    
    private val maze = input.toGrid()

    private val portals: Map<Int, Portal> = buildMap {
        // horizontal
        maze.rows().forEachIndexed { row, chars ->
            val s = chars.joinToString("")
            portalRx.findAll(s).forEach { mr ->
                val name = mr.value.replace(".", "")
                val pos = row * maze.width + if (mr.value[0] == '.') mr.range.first else mr.range.last
                val side = if (mr.range.first == 0 || mr.range.last == s.lastIndex) { // outer
                    Side.OUTER
                } else {
                    Side.INNER
                }
                put(pos, Portal(name, side))
            }
        }

        // vertical
        maze.columns().forEachIndexed { col, chars ->
            val s = chars.joinToString("")
            portalRx.findAll(s).forEach { mr ->
                val name = mr.value.replace(".", "")
                val pos = col + (if (mr.value[0] == '.') mr.range.first else mr.range.last) * maze.width
                val side = if (mr.range.first == 0 || mr.range.last == s.lastIndex) { // outer
                    Side.OUTER
                } else {
                    Side.INNER
                }
                put(pos, Portal(name, side))
            }
        }
    }

    private val vertexMap: Map<Int, Int> = portals.keys.withIndex().associate { (index, pos) -> pos to index }

    private fun connectVertex(start: Int, vertices: Map<Int, Portal>): List<Pair<Int, Int>> {
        val visited = mutableSetOf<Int>()
        val q = mutableListOf(start to 0)
        return generateSequence { q.removeLastOrNull() }
            .onEach { (current, dist) ->
                if (current !in vertices || current == start) {
                    val neighbors = maze.getNeighborsIndexedValue(current).filter { (neighborPos, neighbor) ->
                        neighbor == '.' && neighborPos !in visited
                    }

                    neighbors.forEach { visited.add(it.index) }
                    q.addAll(neighbors.map { (index, _) -> index to dist + 1 })
                }
            }.filter { (current, _) -> current != start && current in vertices.keys }
            .toList()
    }

    private val innerPortalPositions: Map<String, Int> = portals.entries
        .filter { it.value.side == Side.INNER }
        .associate { (pos, portal) ->
            portal.name to vertexMap.getValue(pos)
        }

    private val outerPortalPositions: Map<String, Int> = portals.entries
        .filter { it.value.side == Side.OUTER }
        .associate { (pos, portal) ->
            portal.name to vertexMap.getValue(pos)
        }

    private val edges: List<List<EdgeInfo>> = buildList {
        portals.forEach { (pos, portal) ->
            val neighbors = connectVertex(pos, portals)
                .map { (neighbor, dist) ->
                    EdgeInfo(State(vertexMap.getValue(neighbor), 0), dist)
                }
            if (portal.name != "AA" && portal.name != "ZZ") {
                val warp = if (portal.side == Side.INNER) {
                    EdgeInfo(State(outerPortalPositions.getValue(portal.name), 1), 1)
                } else {
                    EdgeInfo(State(innerPortalPositions.getValue(portal.name), -1), 1)
                }
                add(neighbors + warp)
            } else {
                add(neighbors)
            }
        }
    }

    private fun findExit(dimensionWarp: Boolean): Int {
        val start = State(outerPortalPositions.getValue("AA"), 0) to 0
        val end = State(outerPortalPositions.getValue("ZZ"), 0)
        val q = PriorityQueue<Pair<State, Int>>(compareBy { it.second })
        q.add(start)
        val vertices: MutableMap<State, Int> = mutableMapOf()
        return generateSequence { q.poll() }
            .first { (state, dist) ->
                if (state == end) {
                    true
                } else {
                    // side effects - fills up queue
                    edges[state.pos].forEach { edge ->
                        if (!dimensionWarp || state.level != 0 || edge.state.level != -1) {
                            val edgeLevel = (state.level + if (dimensionWarp) edge.state.level else 0)
                            val edgeState = edge.state.copy(level = edgeLevel)
                            val alternateDist = dist + edge.dist
                            val existingDist = vertices.getOrDefault(edgeState, Int.MAX_VALUE)
                            if (alternateDist < existingDist) {
                                vertices[edgeState] = alternateDist
                                q.add(edgeState to alternateDist)
                            }
                        }
                    }
                    false
                }
            }.second
    }

    override fun part1(): Int = findExit(false)

    override fun part2(): Int = findExit(true)

    companion object {
        val portalRx = Regex("""\.\w{2}|\w{2}\.""")
    }
}

fun main() = Day.runDay(Y19D20::class)

//    Class creation: 67ms
//    Part 1: 528 (2ms)
//    Part 2: 6214 (14ms)
//    Total time: 84ms
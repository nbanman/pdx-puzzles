package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.algorithms.Graph

class Y23D23(private val trails: String) : Day {

    private val width = trails.indexOf('\n')
    private val start = trails.indexOf('.')
    private val end = trails.lastIndexOf('.')

    private val vertices: Set<Int> = buildSet {
        val branchPoints = trails
            .withIndex()
            .filter { (pos, c) ->
                c !in "#\n" && 3 <= listOf(pos - (width + 1), pos + 1, pos - 1, pos + (width + 1))
                    .count { neighbor -> trails.getOrNull(neighbor)?.let { it != '#' } == true }
            }.map { (pos, _) -> pos }
        add(start)
        addAll(branchPoints)
        add(end)
    }

    private val vertexMap = vertices.withIndex().associate { (index, pos) -> pos to index }

    private val newStart = vertexMap.getValue(start)
    private val newEnd = vertexMap.getValue(end)

    private fun connectVertex(pos: Int, vertices: Set<Int>, canGoUphill: Boolean): List<Pair<Int, Int>> {
        val visited = mutableSetOf<Int>()
        val q = mutableListOf(pos to 0)
        return generateSequence { q.removeLastOrNull() }
            .onEach { (current, dist) ->
                if (current !in vertices || current == pos) {
                    val neighbors =
                        if (canGoUphill) {
                            listOf(current - (width + 1), current + 1, current - 1, current + (width + 1))
                        } else {
                            val c = trails[current]
                            when (c) {
                                '^' -> listOf(current - (width + 1))
                                '>' -> listOf(current + 1)
                                'v' -> listOf(current + (width + 1))
                                '<' -> listOf(current - 1)
                                else -> listOf(current - (width + 1), current + 1, current - 1, current + (width + 1))
                            }
                        }.filter { neighbor ->
                            trails.getOrNull(neighbor)?.let { it !in "#\n" } == true && neighbor !in visited
                        }
                    neighbors.forEach { visited.add(it) }
                    q.addAll(neighbors.map { it to dist + 1 })
                }
            }.filter { (current, _) -> current != pos && current in vertices }
            .toList()
    }

    private data class State(val pos: Int, val weight: Int)

    private fun findLongestTrail(
        edgeMap: List<List<Pair<Int, Int>>>,
        start: Int,
        end: Int,
    ): Int {
        var visited = 0L
        
        fun longestTrail(state: State): Int =
            if (state.pos == end) {
                state.weight
            } else {
                visited += 1L shl state.pos
                val neighborStates = edgeMap[state.pos]
                    .filter { (neighbor, _) ->
                        visited shr neighbor and 1L == 0L
                    }.map { (neighbor, weight) -> State(neighbor, state.weight + weight) }
                val result = neighborStates.maxOfOrNull { neighborState -> longestTrail(neighborState) } ?: 0
                visited -= 1L shl state.pos
                result
            }
        
        return longestTrail(State(start, 0))
    }

    override fun part1(): Int {
        val edges = buildList {
            vertices.forEach { pos ->
                val neighbors = connectVertex(pos, vertices, false)
                    .map { (neighbor, dist) -> vertexMap.getValue(neighbor) to dist }
                add(neighbors)
            }
        }
        return findLongestTrail(edges, newStart, newEnd)
    }

    override fun part2(): Int {
        val initial: Map<Int, List<Pair<Int, Int>>> = buildMap<Int, MutableList<Pair<Int, Int>>> {
            // get weighted vertices
            vertices.forEach { pos ->
                val pp = vertexMap.getValue(pos)
                connectVertex(pos, vertices, true)
                    .forEach { (neighbor, dist) ->
                        getOrPut(pp) { mutableListOf() }.add(vertexMap.getValue(neighbor) to dist)
                    }
            }
        }

        // due to grid-like nature of the remaining nodes, the perimeter nodes (those with only three edges) are 
        // directional. Quick and dirty way of finding which directions to exclude is to run my standard BFS which
        // does flood fill of the nodes that don't have four edges, recording the parent node for each node. 
        // Then convert this to a map of node to parent node. Use this map to exclude certain edges in the edge map, 
        // thus making the perimeter nodes directional.
        // This works for everything except for the bottom corner node. This one fails because my standard BFS is
        // shortest-path, but the bottom corner node can be reached in two ways. So I handle the bottom corner 
        // individually.
        val verboten: Map<Int, Int> = Graph
            .bfsSequence(newStart) {
                initial.getValue(it).filter { (neighbor, _) ->
                    initial.getValue(neighbor).size != 4
                }.map { (neighbor, _) -> neighbor }
            }.mapNotNull { v -> v.parent?.let { parent -> v.id to parent.id } }
            .toMap()
        
        val bottomCorner = initial.values.last().first().first
        
        val edges = initial.entries.map { (pos, edges) -> 
            edges.filter { (neighbor) -> 
                if (pos == bottomCorner) {
                    neighbor == newEnd
                } else {
                    verboten[pos] != neighbor
                }    
            } 
        }
        
        return findLongestTrail(edges, newStart, newEnd)
    }
}

fun main() = Day.runDay(Y23D23::class)

// Parts 1 and 2: 289266 us/op [Average]

//    Class creation: 25ms
//    Part 1: 2210 (29ms)
//    Part 2: 6522 (420ms)
//    Total time: 476ms

@Suppress("unused")
private val sampleInput = listOf(
    """#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
""",
)
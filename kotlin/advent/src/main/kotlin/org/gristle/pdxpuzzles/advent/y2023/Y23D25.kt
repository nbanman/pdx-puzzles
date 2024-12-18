package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.algorithms.Graph
import org.gristle.pdxpuzzles.utilities.algorithms.Graph.path

class Y23D25(input: String) : Day {

    // parse to map of edges
    private val componentMap: Map<String, List<String>> = buildMap<String, MutableList<String>> {
        input.lines().map { line ->
            val components = line.split(": ", " ")
            getOrPut(components[0]) { mutableListOf() }.addAll(components.drop(1))
            for (component in components.drop(1)) {
                getOrPut(component) { mutableListOf() }.add(components[0])
            }
        }
    }

    // 35x speedup based on /u/maneatingape's solution
    // https://www.reddit.com/r/adventofcode/comments/18qbsxs/comment/kfoynua/
    override fun part1(): Int {
        // Get a node on the edge by taking a random node, running BFS and grabbing the farthest one.
        val start = Graph.bfs(componentMap.entries.first().key) { componentMap.getValue(it) }.last().id
        // Run bfs from the start node three times, each time removing edges in the path taken. This will saturate
        // the 3 edges to be cut.
        val cutEdges: Map<String, Set<String>> = buildMap<String, MutableSet<String>> {
            for (i in 1..3) {
                val path = Graph
                    .bfs(start) { pos ->
                        componentMap.getValue(pos).filter { it !in getOrDefault(pos, emptySet()) }
                    }.path()
                path.map { it.id }.zipWithNext().forEach { (prev, next) ->
                    getOrPut(prev) { mutableSetOf() }.add(next)
                }
            }
        }
        // Run bfs one more time. Since all the bridge edges are removed, this will only find the nodes on one side.
        val groupA = Graph
            .bfs(start) { pos ->
                componentMap.getValue(pos).filter { it !in cutEdges.getOrDefault(pos, emptySet()) }
            }.size
        return groupA * (componentMap.size - groupA)
    } 

    override fun part2() = "Merry Xmas!"
}

fun main() = Day.runDay(Y23D25::class)

//    Class creation: 26ms
//    Part 1: 569904 (30ms)
//    Total time: 56ms

@Suppress("unused")
private val sampleInput = listOf(
    """jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
""",
)
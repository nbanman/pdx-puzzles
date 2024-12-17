@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.graph

import org.gristle.pdxpuzzles.utilities.iteration.pollUntil
import java.util.*
import kotlin.math.sign

object Graph {

    /**
     * Container for standard vertex information.
     */
    sealed interface Vertex<E> : Comparable<Vertex<E>> {

        val id: E
        val weight: Double
        val parent: Vertex<E>?

        /**
         * Reconstructs a path from the start to the end using the parent reference of each vertex.
         */
        fun path(): List<Vertex<E>> {
            val returnPath = mutableListOf(this)
            var parent = parent
            while (parent != null) {
                returnPath.add(parent)
                parent = parent.parent
            }
            returnPath.reverse()

            return returnPath
        }

        fun steps(): Int = weight.toInt()

        override fun compareTo(other: Vertex<E>) = weight.compareTo(other.weight)

        operator fun component1() = id
        operator fun component2() = weight
        operator fun component3() = parent
    }

    /**
     * Vertex for BFS and Dijkstra algorithms.
     *
     * @param id is a unique identifier or "location" of the vertex. This can be an index, coordinates, or a more
     * complex object representing the state (e.g., coordinates plus an inventory of tools acquired that can
     * influence traversal).
     *
     * @param weight is the cost or distance of traveling to this vertex from the start vertex.
     *
     * @param parent links to the previous vertex and is used to reconstruct the shortest path from start to end.
     */
    class StdVertex<E> (
        override val id: E,
        override val weight: Double = Double.POSITIVE_INFINITY,
        override val parent: Vertex<E>? = null
    ) : Vertex<E> {
        override fun toString(): String {
            return "StdVertex(id=$id, weight=$weight, parent=${parent?.id})"
        }
    }

    /**
     * Vertex for A* algorithm.
     *
     * @param id is a unique identifier or "location" of the vertex. This can be an index, coordinates, or a more
     * complex object representing the state (e.g., coordinates plus an inventory of tools acquired that can
     * influence traversal).
     *
     * @param weight is the cost or distance of traveling to this vertex from the start vertex.
     *
     * @param h is the estimated cost or distance to the goal vertex, obtained by using a heuristic function
     * provided to the aStar function.
     *
     * @param parent links to the previous vertex and is used to reconstruct the shortest path from start to end.
     */
    class AStarVertex<E> (
        override val id: E,
        override val weight: Double = 0.0,
        val h: Double,
        override val parent: Vertex<E>? = null,
    ) : Vertex<E>, Comparable<Vertex<E>> {
        val f: Double
            get() = weight + h

        override fun compareTo(other: Vertex<E>): Int {
            return if (other is AStarVertex<*>) {
                (f - other.f).sign.toInt()
            } else {
                super.compareTo(other)
            }
        }

        override fun toString(): String {
            return "Vertex(id=$id, f=$f, g=$weight, h=$h, parent=${parent?.id})"
        }
    }

    /**
     * Utility function to provide the weight from the beginning to the end of the traversal as an Integer.
     * Returns -1 if the traversal did not complete.
     */
    fun <E> List<Vertex<E>>.steps() = lastOrNull()?.weight?.toInt() ?: -1

    /**
     * Utility function to provide the weight from the beginning to the end of the traversal as an Integer.
     * Returns -1 if the traversal did not complete.
     */
    fun <E> Sequence<Vertex<E>>.steps() = lastOrNull()?.weight?.toInt() ?: -1

    /**
     * Utility function to provide the path from the beginning to the end of the traversal.
     * Returns empty List if the traversal did not complete.
     */
    fun <E> List<Vertex<E>>.path() = lastOrNull()?.path() ?: emptyList()

    /**
     * Utility function to provide the path from the beginning to the end of the traversal.
     * Returns empty List if the traversal did not complete.
     */
    fun <E> Sequence<Vertex<E>>.path() = lastOrNull()?.path() ?: emptyList()

    /**
     * Contains id of a neighboring vertex and the weight to travel there. Used in constructing
     * a map of edges and/or a default edge function for the Dijkstra and AStar functions.
     */
    data class Edge<E>(val vertexId: E, val weight: Double) {
        fun toAStarVertex(parent: AStarVertex<E>, heuristic: (E) -> Double): AStarVertex<E> =
            AStarVertex(vertexId, parent.weight + weight, heuristic(vertexId), parent)
    }

    /**
     * Finds the shortest path in a directed, weighted graph from a starting point to an end condition.
     *
     * @param startId (required). The id for the starting vertex.
     *
     * @param heuristic (required) takes the current location id and outputs an admissible estimate
     * of the distance to the goal id.
     *
     * @param endCondition (optional) By default pathfinding will continue until all vertices are explored or
     * the heuristic returns 0.0 for the vertex in question. This can be changed by specifying your own predicate
     * for ending.
     *
     * @param edges
     * @param defaultEdges (optional) The function can either take in a map providing a list of edges for each
     * vertex id, or a function that takes a vertex id provides a list of edges, or both. If both are provided,
     * the function will only be used where the map entry is absent.
     */
    inline fun <E> aStar(
        startId: E,
        noinline heuristic: (E) -> Double,
        endCondition: (E) -> Boolean = { id -> heuristic(id) == 0.0 },
        edges: Map<E, List<Edge<E>>> = mapOf(),
        defaultEdges: (E) -> List<Edge<E>> = { emptyList() }
    ): List<Vertex<E>> {
        val startVertex = AStarVertex(startId, 0.0, heuristic(startId))

        val open = PriorityQueue<AStarVertex<E>>()
        open.add(startVertex)
        val closed = mutableSetOf<E>()
        while (open.isNotEmpty()) {
            val current = open.pollUntil { !closed.contains(it.id) } ?: break
            closed.add(current.id)
            if (endCondition(current.id)) {
                return current.path()
            }
            val neighbors = edges[current.id] ?: defaultEdges(current.id)
            for (neighbor in neighbors) {
                if (!closed.contains(neighbor.vertexId)) open.add(neighbor.toAStarVertex(current, heuristic))
            }
        }
        return emptyList()
    }

    /**
     * Finds the shortest path in a directed, unweighted graph from a starting point to all vertices traversed
     * until all reachable vertices have been traversed or the end condition is met. Faster than Dijkstra, but
     * does not work for weighted graphs.
     *
     * @param startId (required). The id for the starting vertex.
     *
     * @param endCondition (optional) By default pathfinding will continue until all vertices are explored.
     * This can be changed by specifying your own predicate for ending, such as arriving at a particular
     * vertex.
     *
     * @param edges
     * @param defaultEdges (optional) The function can either take in a map providing a list of edges for each
     * vertex id, or a function that takes a vertex id provides a list of edges, or both. If both are provided,
     * the function will only be used where the map entry is absent.
     */
    inline fun <E> bfs(
        startId: E,
        endCondition: (E) -> Boolean? = { null },
        edges: Map<E, List<E>> = mapOf(),
        defaultEdges: (E) -> List<E> = { emptyList() }
    ): List<Vertex<E>> {
        val start = StdVertex(startId, 0.0)
        val q: Deque<Vertex<E>> = ArrayDeque()
        q.add(start)
        // "visited" serves double duty here. If it were just to ensure that already determined vertices were
        // not visited again, a Set would do instead of a Map. But I take this opportunity to store the Vertex
        // which gets returned as part of the function return.
        val visited = mutableMapOf(startId to start)
        while (q.isNotEmpty()) {
            val current = q.removeFirst()
            val neighbors: List<StdVertex<E>> = (edges[current.id] ?: defaultEdges(current.id))
                .filter { it !in visited }
                .map { neighbor ->
                    StdVertex(neighbor, current.weight + 1.0, current)
                        .also { visited[neighbor] = it }
                }
            neighbors
                .forEach { neighbor ->
                    if (endCondition(neighbor.id) == true) return visited.values.toList()
                    q.add(neighbor)
                }
        }
        return if (endCondition(startId) == null) visited.values.toList() else emptyList()
    }

    /**
     * Finds the shortest path in a directed, unweighted graph from a starting point to all vertices traversed
     * until all reachable vertices have been traversed or the end condition is met.
     *
     * @param startId (required). The id for the starting vertex.
     *
     * @param endCondition (optional) By default pathfinding will continue until all vertices are explored.
     * This can be changed by specifying your own predicate for ending, such as arriving at a particular
     * vertex.
     *
     * @param edges
     * @param defaultEdges (optional) The function can either take in a map providing a list of edges for each
     * vertex id, or a function that takes a vertex id provides a list of edges, or both. If both are provided,
     * the function will only be used where the map entry is absent.
     */
    inline fun <E> dfs(
        startId: E,
        endCondition: (E) -> Boolean? = { null },
        edges: Map<E, List<E>> = mapOf(),
        defaultEdges: (E) -> List<E> = { emptyList() }
    ): List<Vertex<E>> {
        val start = StdVertex(startId, 0.0)
        val q: Deque<Vertex<E>> = ArrayDeque()
        q.add(start)
        // "visited" serves double duty here. If it were just to ensure that already determined vertices were
        // not visited again, a Set would do instead of a Map. But I take this opportunity to store the Vertex
        // which gets returned as part of the function return.
        val visited = mutableMapOf<E, Vertex<E>>()
        while (q.isNotEmpty()) {
            val current = q.pop() ?: break
            if (current.id !in visited) {
                visited[current.id] = current
                if (endCondition(current.id) == true) return visited.values.toList()
                (edges[current.id] ?: defaultEdges(current.id))
                    .map { StdVertex(it, current.weight + 1.0, current) }
                    .forEach { q.add(it) }
            }
        }
        return if (endCondition(startId) == null) visited.values.toList() else emptyList()
    }

    /**
     * Finds the shortest path in a directed, weighted graph from a starting point to all vertices traversed
     * until all reachable vertices have been traversed or the end condition is met.
     *
     * @param startId (required). The id for the starting vertex.
     *
     * @param endCondition (optional) By default pathfinding will continue until all vertices are explored.
     * This can be changed by specifying your own predicate for ending, such as arriving at a particular
     * vertex.
     *
     * @param edges
     * @param defaultEdges (optional) The function can either take in a map providing a list of edges for each
     * vertex id, or a function that takes a vertex id provides a list of edges, or both. If both are provided,
     * the function will only be used where the map entry is absent.
     */
    inline fun <E> dijkstra(
        startId: E,
        endCondition: (E) -> Boolean? = { null },
        edges: Map<E, List<Edge<E>>> = mapOf(),
        defaultEdges: (E) -> List<Edge<E>> = { emptyList() }
    ): List<Vertex<E>> {
        val start = StdVertex(startId, 0.0)
        val weights = mutableMapOf(startId to start.weight)
        val q = PriorityQueue<Vertex<E>>()
        q.add(start)
        // "visited" serves double duty here. If it were just to ensure that already determined vertices were
        // not visited again, a Set would do instead of a Map. But I take this opportunity to store the Vertex
        // which gets returned as part of the function return.
        val visited = mutableMapOf<E, Vertex<E>>()
        while (true) {
            val current = q.pollUntil { visited[it.id] == null } ?: break
            visited[current.id] = current
            if (endCondition(current.id) == true) return visited.values.toList()
            (edges[current.id] ?: defaultEdges(current.id)).forEach { neighborEdge ->
                val alternateWeight = current.weight + neighborEdge.weight
                val weight = weights.getOrDefault(neighborEdge.vertexId, Double.MAX_VALUE)
                if (alternateWeight < weight) {
                    weights[neighborEdge.vertexId] = alternateWeight
                    q.add(StdVertex(neighborEdge.vertexId, alternateWeight, current))
                }
            }
        }
        return if (endCondition(startId) == null) visited.values.toList() else emptyList()
    }


    inline fun <E> bfsSequence(
        startId: E,
        edges: Map<E, Iterable<E>> = mapOf(),
        crossinline defaultEdges: (E) -> Iterable<E> = { emptyList() }
    ): Sequence<Vertex<E>> = sequence {
        val start = StdVertex(startId, 0.0)
        val q = ArrayDeque<Vertex<E>>()
        q.add(start)
        val visited = mutableMapOf(startId to start)
        while (q.isNotEmpty()) {
            val current = q.removeFirst()
            yield(current)
            (edges[current.id] ?: defaultEdges(current.id))
                .filter { it !in visited }
                .map { StdVertex(id = it, weight = current.weight + 1.0, parent = current) }
                .forEach { neighbor ->
                    visited[neighbor.id] = neighbor
                    q.add(neighbor)
                }
        }
    }

    inline fun <E> dfsSequence(
        startId: E,
        edges: Map<E, Iterable<E>> = mapOf(),
        crossinline defaultEdges: (E) -> Iterable<E> = { emptyList() }
    ): Sequence<Vertex<E>> = sequence {
        val start = StdVertex(startId, 0.0)
        val q = ArrayDeque<StdVertex<E>>()
        q.add(start)
        // "visited" serves double duty here. If it were just to ensure that already determined vertices were
        // not visited again, a Set would do instead of a Map. But I take this opportunity to store the Vertex
        // which gets returned as part of the function return.
        val visited = mutableMapOf<E, Vertex<E>>()
        while (q.isNotEmpty()) {
            val current = q.pop()
            if (current.id !in visited) {
                visited[current.id] = current
                yield(current)
                (edges[current.id] ?: defaultEdges(current.id))
                    .map { StdVertex(it, current.weight + 1.0, current) }
                    .forEach { q.add(it) }
            }
        }
    }

    inline fun <E> dijkstraSequence(
        startId: E,
        edges: Map<E, List<Edge<E>>> = mapOf(),
        crossinline defaultEdges: (E) -> List<Edge<E>> = { emptyList() }
    ): Sequence<Vertex<E>> = sequence {
        val start = StdVertex(startId, 0.0)
        val weights = mutableMapOf(startId to start.weight)
        val q = PriorityQueue<Vertex<E>>()
        q.add(start)
        val visited = mutableSetOf<E>()
        while (true) {
            val current = q.pollUntil { !visited.contains(it.id) } ?: break
            yield(current)
            visited.add(current.id)
            val neighborEdges = (edges[current.id] ?: defaultEdges(current.id))

            neighborEdges.forEach { neighborEdge ->
                val alternateWeight = current.weight + neighborEdge.weight
                val weight = weights.getOrDefault(neighborEdge.vertexId, Double.MAX_VALUE)
                if (alternateWeight < weight) {
                    weights[neighborEdge.vertexId] = alternateWeight
                    q.add(StdVertex(neighborEdge.vertexId, alternateWeight, current))
                }
            }
        }
    }

    /**
     * Works but slower nad really don't see the need.
     */
    inline fun <E> aStarSequence(
        startId: E,
        noinline heuristic: (E) -> Double,
        edges: Map<E, List<Edge<E>>> = mapOf(),
        crossinline defaultEdges: (E) -> List<Edge<E>> = { emptyList() }
    ): Sequence<Vertex<E>> = sequence {
        val startVertex = AStarVertex(startId, 0.0, heuristic(startId))


        val open = PriorityQueue<AStarVertex<E>>()
        open.add(startVertex)
        val closed = mutableSetOf<E>()
        while (open.isNotEmpty()) {
            val current = open.pollUntil { !closed.contains(it.id) } ?: break
            yield(current)
            closed.add(current.id)
            val neighbors = edges[current.id] ?: defaultEdges(current.id)
            for (neighbor in neighbors) {
                if (!closed.contains(neighbor.vertexId)) open.add(neighbor.toAStarVertex(current, heuristic))
            }
        }
    }
}



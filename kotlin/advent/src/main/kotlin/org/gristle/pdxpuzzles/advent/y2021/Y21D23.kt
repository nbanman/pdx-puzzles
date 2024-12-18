package org.gristle.pdxpuzzles.advent.y2021

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.algorithms.Graph
import org.gristle.pdxpuzzles.utilities.algorithms.Graph.steps

class Y21D23(input: String) : Day {

    companion object {

        /**
         * Utility function takes an index and an element and swaps in that element at the index.
         */
        fun <T> List<T>.swapElementAt(index: Int, newElement: T): List<T> =
            mapIndexed { idx, t -> if (index == idx) newElement else t }
    }

    /**
     * Used to identify the Amphipod and store energy information. Amphipod.E is actually "Empty"
     */
    enum class Amphipod(val energy: Double) {
        A(1.0),
        B(10.0),
        C(100.0),
        D(1000.0),
        E(0.0),
    }

    /**
     * Represents the state of the room.
     */
    data class Room(val id: Amphipod, val spots: List<Amphipod>) {
        val hallIndex = id.ordinal + 1
        val isFinished: Boolean by lazy { spots.all { it == id } }
        val isOpen: Boolean by lazy { spots.all { it == Amphipod.E || it == id } && spots.any { it == Amphipod.E } }
        val isMixed: Boolean by lazy { spots.any { it !in listOf(Amphipod.E, id) } }
        val openings: Int by lazy { spots.count { it == Amphipod.E } }

        val topAmphipod: Amphipod = spots.find { it != Amphipod.E } ?: Amphipod.E

        /**
         * Adds an amphipod to the room.
         */
        fun addAmphipod(): Room {
            val lastEmptyIndex = spots.lastIndexOf(Amphipod.E)
            return copy(spots = spots.swapElementAt(lastEmptyIndex, id))
        }

        /**
         * Removes an amphipod from the room
         */
        fun removeAmphipod(): Room {
            val firstOccupiedIndex = spots.indexOfFirst { it != Amphipod.E }
            return copy(spots = spots.swapElementAt(firstOccupiedIndex, Amphipod.E))
        }
    }

    /**
     * Represents the state of all amphipods in the hallway and rooms.
     */
    data class State(
        val hallway: List<Amphipod>,
        val rooms: List<Room>,
    ) {

        val isEnd = hallway.all { it == Amphipod.E } && this.rooms.all(Room::isFinished)

        fun edges(): List<Graph.Edge<State>> {

            // find open rooms and get corresponding values
            this.rooms.filter(Room::isOpen).forEach { room ->
                // look left
                var steps = 1
                for (hallSpot in room.hallIndex downTo 0) {
                    // look in hall first
                    when (hallway[hallSpot]) {
                        room.id -> return listOf(edgeFromHallToRoom(hallSpot, room, steps)) // found a piece to place
                        Amphipod.E -> {} // keep going
                        else -> break // left blocked; go right
                    }
                    // look in room (if available)
                    val roomSpot = hallSpot - 2
                    if (roomSpot >= 0) {
                        val top = this.rooms[roomSpot].topAmphipod
                        if (top == room.id) {
                            return listOf(edgeFromRoomToRoom(roomSpot, room, steps))
                        }
                    }
                    steps = if (hallSpot == 1) steps + 1 else steps + 2
                }
                // look right
                steps = 1
                for (hallSpot in (room.hallIndex + 1)..hallway.lastIndex) {
                    // look in hall first
                    when (hallway[hallSpot]) {
                        room.id -> return listOf(edgeFromHallToRoom(hallSpot, room, steps))
                        Amphipod.E -> {}
                        else -> break // right blocked; go to pop
                    }
                    // look in room (if available)
                    val roomSpot = hallSpot - 1
                    if (roomSpot <= this.rooms.lastIndex) {
                        val top = this.rooms[roomSpot].topAmphipod
                        if (top == room.id) {
                            return listOf(edgeFromRoomToRoom(roomSpot, room, steps))
                        }
                    }
                    steps = if (hallSpot == 5) steps + 1 else steps + 2
                }
            }

            // Move to popping...
            return buildList {
                this@State.rooms.filter(Room::isMixed).forEach { room ->
                    // look left
                    var steps = 1
                    for (hallSpot in room.hallIndex downTo 0) {
                        if (hallway[hallSpot] == Amphipod.E) add(edgeFromRoomToHall(hallSpot, room, steps)) else break
                        steps = if (hallSpot == 1) steps + 1 else steps + 2
                    }
                    // look right
                    steps = 1
                    for (hallSpot in (room.hallIndex + 1)..hallway.lastIndex) {
                        if (hallway[hallSpot] == Amphipod.E) add(edgeFromRoomToHall(hallSpot, room, steps)) else break
                        steps = if (hallSpot == 5) steps + 1 else steps + 2
                    }
                }

            }
        }

        private inline fun List<Room>.newRooms(id: Amphipod, action: Room.() -> Room): List<Room> = List(size) { idx ->
            if (idx == id.ordinal) {
                rooms[idx].action()
            } else {
                rooms[idx]
            }
        }

        private fun edgeFromRoomToHall(hallSpot: Int, room: Room, steps: Int): Graph.Edge<State> {
            val newAmphipod = room.topAmphipod
            val newState = let {
                val newHallway = hallway.swapElementAt(hallSpot, newAmphipod)
                val newRooms = rooms.newRooms(room.id, Room::removeAmphipod)
                State(newHallway, newRooms)
            }
            val weight = (steps + room.openings + 1) * newAmphipod.energy
            return Graph.Edge(newState, weight)
        }

        private fun edgeFromHallToRoom(hallSpot: Int, room: Room, steps: Int): Graph.Edge<State> {
            val newState = let {
                val newHallway = hallway.swapElementAt(hallSpot, Amphipod.E)
                val newRooms = rooms.newRooms(room.id, Room::addAmphipod)
                State(newHallway, newRooms)
            }
            val weight = (steps + room.openings) * room.id.energy
            return Graph.Edge(newState, weight)
        }

        private fun edgeFromRoomToRoom(roomSpot: Int, room: Room, steps: Int): Graph.Edge<State> {
            val other = this.rooms[roomSpot]
            val newState = let {
                val newRooms = List(rooms.size) { index ->
                    when (index) {
                        room.id.ordinal -> room.addAmphipod()
                        roomSpot -> other.removeAmphipod()
                        else -> rooms[index]
                    }
                }
                copy(rooms = newRooms)
            }

            val weight = (steps + other.openings + 1 + room.openings + 1) * room.id.energy
            return Graph.Edge(newState, weight)
        }

        override fun hashCode(): Int {
            var result = hallway.hashCode()
            result = 31 * result + rooms.hashCode()
            return result
        }

        override fun equals(other: Any?): Boolean {
            if (this === other) return true
            if (javaClass != other?.javaClass) return false

            other as State

            if (hallway != other.hallway) return false
            if (rooms != other.rooms) return false

            return true
        }
    }

    private fun totalEnergy(state: State): Int = Graph
        .dijkstra(
            startId = state,
            endCondition = State::isEnd,
            defaultEdges = State::edges
        ).steps()

    /**
     * Parses strings into State object.
     */
    private fun getState(a: List<String>): State {
        return a.let {
            List(it.size * it.first().length) { i ->
                Amphipod.valueOf(it[i % it.size][i / it.size].toString())
            }.chunked(it.size)
        }.mapIndexed { index, s ->
            val name = Amphipod.entries[index]
            Room(name, s)
        }.let { State(List(7) { Amphipod.E }, it) }
    }

    private val strings = input
        .filter { it in "ABCD\n" }
        .split('\n')
        .filter { it.isNotBlank() }

    override fun part1(): Int = totalEnergy(getState(strings))

    override fun part2(): Int {
        val expandedStrings = strings.dropLast(1) + listOf("DCBA", "DBAC") + strings.last()
        return totalEnergy(getState(expandedStrings))
    }
}

fun main() = Day.runDay(Y21D23::class)

//    Class creation: 2ms
//    Part 1: 14148 (254ms)
//    Part 2: 43814 (532ms)
//    Total time: 789ms
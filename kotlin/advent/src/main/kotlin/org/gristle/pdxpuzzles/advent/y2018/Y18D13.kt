package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.Grid
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y18D13(input: String) : Day {

    // Tracks the different turning behaviors of cars at intersections
    enum class TurnState {
        LEFT {
            override fun advance() = STRAIGHT
            override fun dir(dir: Nsew): Nsew = dir.left()
        },
        STRAIGHT {
            override fun advance() = RIGHT
            override fun dir(dir: Nsew): Nsew = dir
        },
        RIGHT {
            override fun advance() = LEFT
            override fun dir(dir: Nsew): Nsew = dir.right()
        };

        // toggles the TurnState to the next state after being used 
        abstract fun advance(): TurnState

        // returns the new direction after being used
        abstract fun dir(dir: Nsew): Nsew
    }

    // Stores a car's direction and TurnState information
    data class Car(
        val dir: Nsew,
        val turnState: TurnState,
    )

    // Parse the input to a racetrack
    private val racetrack: Grid<Char> = input.toGrid()

    // Create initial map of cars by position    
    private val cars: Map<Coord, Car> = racetrack
        .mapIndexedNotNull { pos, c -> // look at all spaces on racetrack

            // set direction if the space holds a car; otherwise, null
            val dir = when (c) {
                '^' -> Nsew.NORTH
                '>' -> Nsew.EAST
                'v' -> Nsew.SOUTH
                '<' -> Nsew.WEST
                else -> null
            }

            // if the space holds a car, add it to map
            dir?.let { nsew -> racetrack.coordOf(pos) to Car(nsew, TurnState.LEFT) }
        }.toMap()

    // Advances state in the sequence below.
    private fun Pair<Map<Coord, Car>, Coord?>.move(racetrack: Grid<Char>): Pair<Map<Coord, Car>, Coord?> {

        // Makes mutable copy of the input state
        val stillRacing = first.toMutableMap()

        // Tracks the coordinates of any crashes
        var crashCoord: Coord? = null

        // Moves cars along track. If there's a collision, remove collided cars from map and note coordinates
        stillRacing
            .entries
            .sortedBy { (pos) -> racetrack.indexOf(pos) } // go from top-left to bottom-right
            .forEach { (pos, car) ->

                // entry could already be deleted from map if there was a collision with it, so only run if 
                // the map still contains the entry
                if (stillRacing.contains(pos)) {

                    // find the spot the car will move to
                    val newPos = pos.move(car.dir)

                    // remove spot the car previously occupied
                    stillRacing.remove(pos)

                    if (stillRacing[newPos] == null) { // if no crash...

                        // place car in new position, adjusting dir and TurnState as appropriate
                        when (racetrack[newPos]) {
                            '+' -> stillRacing[newPos] = Car(
                                car.turnState.dir(car.dir),
                                car.turnState.advance(),
                            )

                            '\\' -> {
                                val newDir = when (car.dir) {
                                    Nsew.NORTH -> Nsew.WEST
                                    Nsew.SOUTH -> Nsew.EAST
                                    Nsew.EAST -> Nsew.SOUTH
                                    Nsew.WEST -> Nsew.NORTH
                                }
                                stillRacing[newPos] = car.copy(dir = newDir)
                            }

                            '/' -> {
                                val newDir = when (car.dir) {
                                    Nsew.NORTH -> Nsew.EAST
                                    Nsew.SOUTH -> Nsew.WEST
                                    Nsew.EAST -> Nsew.NORTH
                                    Nsew.WEST -> Nsew.SOUTH
                                }
                                stillRacing[newPos] = car.copy(dir = newDir)
                            }

                            else -> stillRacing[newPos] = car
                        }
                    } else { // crash...
                        if (crashCoord == null) crashCoord = newPos // mark the crash coordinates
                        stillRacing.remove(newPos) // remove the car that was crashed into
                    }
                }
            }
        return stillRacing to crashCoord
    }

    private val initialState: Pair<Map<Coord, Car>, Coord?> = cars to null

    // sequence advances state indefinitely
    private val race = generateSequence(initialState) { it.move(racetrack) }

    override fun part1() = race
        .first { (_, crash) -> crash != null } // stop sequence when a crash is recorded
        .let { (_, crash) -> crash?.let { "${it.x},${it.y}" } ?: "" } // return crash coordinate

    override fun part2() = race
        .first { (cars) -> cars.size == 1 } // stop sequence when only one car remains
        .let { (cars) -> cars.keys.first().let { "${it.x},${it.y}" } } // return car coordinate}
}

fun main() = Day.runDay(Y18D13::class)

//    Class creation: 20ms
//    Part 1: 86,118 (8ms)
//    Part 2: 2,81 (39ms)
//    Total time: 69ms
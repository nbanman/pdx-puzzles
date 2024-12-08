package org.gristle.pdxpuzzles.advent.y2024

import kotlinx.coroutines.runBlocking
import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.iteration.parMap
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y24D6(input: String) : Day {
    private val lab = input.toGrid()
    private val start = lab.coordOf(lab.indexOf('^'))

    data class State(val pos: Coord, val dir: Nsew, val turned: Boolean)

    private val move: (State, Coord?) -> State? = { (pos, dir, _), obstacle ->
        val forward = pos.move(dir)

        // if this is null that means it's gone off the map, which is a valid thing to do!
        lab.getOrNull(forward)?.let { space ->
            if (space != '#' && forward != obstacle) {
                State(forward, dir, false)
            } else {
                val right = dir.right()
                val newMove = pos.move(right)
                if (newMove == obstacle || lab.getOrNull(newMove) == '#') {
                    val flipped = dir.flip()
                    State(pos.move(flipped), flipped, true)
                } else {
                    State(newMove, right, true)
                }
            }
        }
    }

    private val goldenPath = generateSequence(State(start, Nsew.NORTH, false)) { move(it, null) }

    override fun part1(): Int = goldenPath.map{ it.pos }.distinct().count()

    override fun part2(): Int = runBlocking {
        val obstacles = mutableSetOf<Coord>()

        goldenPath
            .zipWithNext()
            .toList()
            .filter { (_, next) -> obstacles.add(next.pos) }
            .parMap { (current, next) ->
                val obstacle = next.pos

                val visited = mutableSetOf<State>()
                generateSequence(current) { move(it, obstacle) }
                    .firstOrNull { state ->
                        if (state.turned) {
                            !visited.add(state)
                        } else {
                            false
                        }
                    }
                    ?.let { true }
                    ?: false
            }.count { it }
        }
    }

fun main() = Day.runDay(Y24D6::class)

//    Class creation: 11ms
//    Part 1: 5444 (13ms)
//    Part 2: 1946 (272ms)
//    Total time: 296ms


@Suppress("unused")
private val test = listOf(
    """....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
""",
)
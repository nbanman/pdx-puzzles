package org.gristle.pdxpuzzles.advent.y2024

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.toGrid

class Y24D6(input: String) : Day {
    private val lab = input.toGrid()
    private val start = lab.coordOf(lab.indexOf('^'))
    private val move: (State, Coord?) -> State? = { (pos, dir, _), obstacle ->
        val forward = pos.move(dir)
        lab.getOrNull(forward)?.let { space ->
            if (space != '#' && forward != obstacle) {
                State(forward, dir)
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

    data class State(val pos: Coord, val dir: Nsew = Nsew.NORTH, val turned: Boolean = false)

    val goldenPath = generateSequence(State(start)) { move(it, null) }
        .map { (pos, _) -> pos }
        .toSet()

    override fun part1(): Int = goldenPath.size

    override fun part2(): Int = runBlocking {
        withContext(Dispatchers.Default) {
            goldenPath
                .filter { lab[it] == '.' }
                .map { obstacle ->
                    async {
                        val visited = mutableSetOf<State>()
                        generateSequence(State(start)) { move(it, obstacle) }
                            .firstOrNull { state ->
                                if (state.turned) {
                                    !visited.add(state)
                                } else {
                                    false
                                }
                            }
                            ?.let { true }
                            ?: false
                    }
                }.count { it.await() }
        }
    }
}

fun main() = Day.runDay(Y24D6::class)

//    Class creation: 24ms
//    Part 1: 5444 (2ms)
//    Part 2: 1946 (517ms)
//    Total time: 544ms


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
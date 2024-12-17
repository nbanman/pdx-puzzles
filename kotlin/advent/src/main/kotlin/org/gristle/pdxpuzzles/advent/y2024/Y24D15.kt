package org.gristle.pdxpuzzles.advent.y2024

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.enums.Nsew
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.minMaxRanges
import org.gristle.pdxpuzzles.utilities.objects.toCoord
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit

class Y24D15(input: String) : Day {
    enum class Entity {
        Wall,
        Box,
        LeftBox,
        RightBox,
    }

    private val width: Int
    private val height: Int
    private val warehouse: Map<Coord, Entity>
    private val directions: List<Nsew>

    init {
        val (warehouseStr, dirStr) = input.blankSplit()
        directions = dirStr.mapNotNull {
            when (it) {
                '^' -> Nsew.NORTH
                '>' -> Nsew.EAST
                'v' -> Nsew.SOUTH
                '<' -> Nsew.WEST
                '\n' -> null
                else -> throw IllegalArgumentException("Movement $it not recognized.")
            }
        }
        width = warehouseStr.indexOf('\n') + 1
        height = warehouseStr.count { it == '\n' } + 1

        warehouse = warehouseStr.withIndex()
            .mapNotNull { (idx, c) ->
                val entity = when (c) {
                    '#' -> Entity.Wall
                    'O' -> Entity.Box
                    else -> null
                }
                entity?.let {
                    val pos = Coord(idx % width, idx / width)
                    pos to it
                }
            }.toMap()
    }

    private val robot = input.indexOf('@').toCoord(width)

    private fun Coord.findEmpty(dir: Nsew, warehouse: Map<Coord, Entity>): Coord? =
        generateSequence(this.move(dir)) { it.move(dir) }
            .takeWhile { pos ->
                pos.x in 1 until width - 2 && pos.y in 1 until height - 1
                        && warehouse[pos] != Entity.Wall
            }.find { pos -> pos !in warehouse }

    private fun Coord.gps(): Int = y * 100 + x

    override fun part1(): Int {
        val warehouse = warehouse.toMutableMap()
        var robot = robot
        for (dir in directions) {
            val empty = robot.findEmpty(dir, warehouse) ?: continue
            robot = robot.move(dir)
            if (warehouse.contains(robot)) {
                warehouse.remove(robot)
                warehouse[empty] = Entity.Box
            }
        }

        return warehouse
            .filter { (_, entity) -> entity == Entity.Box }
            .map { (pos, _) -> pos.gps() }
            .sum()
    }

    private fun Coord.pushHz(dir: Nsew, dist: Int, warehouse: MutableMap<Coord, Entity>, isRobot: Boolean): Boolean {
        val next = move(dir, dist)
        return when (warehouse[next]) {
            Entity.Wall -> false
            null -> {
                if (warehouse[this] != null) pushBoxHz(warehouse, dir)
                true
            }
            Entity.Box -> throw IllegalStateException("Normal boxes should not be in map.")
            else -> {
                val moveable = next.pushHz(dir, 2, warehouse, false)
                if (moveable && !isRobot) pushBoxHz(warehouse, dir)
                moveable
            }
        }
    }

    private fun Coord.pushBoxHz(
        warehouse: MutableMap<Coord, Entity>,
        dir: Nsew,
    ) {
        val next = this.move(dir)
        val nextNext = next.move(dir)
        warehouse[next] = if (dir == Nsew.EAST) Entity.LeftBox else Entity.RightBox
        warehouse[nextNext] = if (dir == Nsew.EAST) Entity.RightBox else Entity.LeftBox
        warehouse.remove(this)
    }

    private fun Coord.checkVt(dir: Nsew, warehouse: MutableMap<Coord, Entity>, isRobot: Boolean): Boolean {
        val next = move(dir)
        val nextRight = next.move(Nsew.EAST)
        return when (warehouse[next]) {
            Entity.Wall -> false
            null -> {
                if (isRobot) true else {
                    when (warehouse[nextRight]) {
                        Entity.Wall -> false
                        null -> true
                        Entity.LeftBox -> nextRight.checkVt(dir, warehouse, false)
                        else -> throw IllegalStateException("Should not be right box.")
                    }
                }
            }
            Entity.Box -> throw IllegalStateException("Normal boxes should not be in map.")
            Entity.RightBox -> {
                if (isRobot) {
                    val nextLeft = next.move(Nsew.WEST)
                    nextLeft.checkVt(dir, warehouse, false)
                } else {
                    when (warehouse[nextRight]) {
                        Entity.Wall -> false
                        null -> next.move(Nsew.WEST).checkVt(dir, warehouse, false)
                        Entity.LeftBox -> {
                            next.move(Nsew.WEST).checkVt(dir, warehouse, false)
                                    && nextRight.checkVt(dir, warehouse, false)
                        }

                        else -> throw IllegalStateException("Should not be right box")
                    }
                }
            }
            Entity.LeftBox -> next.checkVt(dir, warehouse, false)
        }
    }

    private fun Coord.pushBoxVt(dir: Nsew, warehouse: MutableMap<Coord, Entity>, isRobot: Boolean) {
        val next = move(dir)
        val nextRight = next.move(Nsew.EAST)
        when (warehouse[next]) {
            null -> {
                if (isRobot) return
                when (val right = warehouse[nextRight]) {
                    null -> {}
                    Entity.LeftBox -> nextRight.pushBoxVt(dir, warehouse, false)
                    else -> throw IllegalStateException("$right should be empty or leftBox")
                }
            }
            Entity.LeftBox -> next.pushBoxVt(dir, warehouse, false)
            Entity.RightBox -> {
                if (isRobot) {
                    next.move(Nsew.WEST).pushBoxVt(dir, warehouse, false)
                } else {
                    when (val right = warehouse[nextRight]) {
                        null -> next.move(Nsew.WEST).pushBoxVt(dir, warehouse, false)
                        Entity.LeftBox -> {
                            next.move(Nsew.WEST).pushBoxVt(dir, warehouse, false)
                            nextRight.pushBoxVt(dir, warehouse, false)
                        }
                        else -> throw IllegalStateException("$right should be empty or leftBox")
                    }
                }

            }
            else -> throw IllegalStateException("There cannot be a wall here.")
        }
        if (!isRobot) {
            warehouse.remove(this)
            warehouse.remove(this.move(Nsew.EAST))
            warehouse[next] = Entity.LeftBox
            warehouse[nextRight] = Entity.RightBox
        }
    }

    override fun part2(): Int {
        val warehouse = mutableMapOf<Coord, Entity>()
        this.warehouse.entries.forEach { (oldPos, oldEntity) ->
            val pos = oldPos.copy(x = oldPos.x * 2)
            if (oldEntity == Entity.Box) {
                warehouse[pos] = Entity.LeftBox
                warehouse[pos.east()] = Entity.RightBox
            } else {
                warehouse[pos] = Entity.Wall
                warehouse[pos.east()] = Entity.Wall
            }
        }
        var robot = robot.copy(x = robot.x * 2)

        for (dir in directions) {
            if (dir == Nsew.NORTH || dir == Nsew.SOUTH) {
                if (robot.checkVt(dir, warehouse, true)) {
                    robot.pushBoxVt(dir, warehouse, true)
                    robot = robot.move(dir)
                }
            } else {
                if (robot.pushHz(dir, 1, warehouse, true)) robot = robot.move(dir)
            }
        }



        return warehouse
            .filter { (_, entity) -> entity == Entity.LeftBox }
            .map { (pos, _) -> pos.gps() }
            .sum()
    }
}

fun main() = Day.runDay(Y24D15::class) // 1552463

//    Class creation: 10ms
//    Part 1: 1552463 (34ms)
//    Part 2: 1554058 (14ms)
//    Total time: 58ms

private val test = listOf("""########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<""", """##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^""", """#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^""")


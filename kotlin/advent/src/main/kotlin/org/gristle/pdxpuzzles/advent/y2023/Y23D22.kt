package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.iteration.minMax
import org.gristle.pdxpuzzles.utilities.objects.Xyz
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y23D22(input: String) : Day {
    
    data class Brick(val points: List<Xyz>) : Comparable<Brick> {
        override fun compareTo(other: Brick): Int = compareValuesBy(this, other) { it.points.first().z }
        
        companion object {
            fun of(from: Xyz, to: Xyz): Brick {
                val (minX, maxX) = minMax(from.x, to.x)
                val (minY, maxY) = minMax(from.y, to.y)
                val (minZ, maxZ) = minMax(from.z, to.z)
                val points = buildList { 
                    for (x in minX..maxX) {
                        for (y in minY..maxY) {
                            for (z in minZ..maxZ) {
                                add(Xyz(x, y, z))
                            }
                        }
                    }
                }
                return Brick(points)
            }
        }
    }
    
    class BrickHouse(dimensions: Xyz, bricks: Iterable<Brick>) {
        private val space: Array<Brick?> = 
            Array((dimensions.x + 1) * (dimensions.y + 1) * (dimensions.z + 1) ) { null } 
        
        private val restedOnMap = mutableMapOf<Brick, MutableSet<Brick>>()
        private val restingOnMap = mutableMapOf<Brick, MutableSet<Brick>>()
        private val bricks = mutableSetOf<Brick>()
        
        private val width = dimensions.x + 1
        private val lengthWidth = width * (dimensions.y + 1)
        
        init {
            bricks.forEach { brick -> place(brick) }
        }

        fun place(brick: Brick): Boolean {
            val (placement, lower) = generateSequence(brick.points) { it.below() }
                .zipWithNext()
                .takeWhile { (upper, _) -> 
                    isFree(upper) 
                }.lastOrNull()
                ?.let { (upper, lower) -> Brick(upper) to lower } 
                ?: return false
            lower.forEach { below ->
                space.getOrNull(below.index())?.let { belowBrick -> 
                    restedOnMap.getOrPut(belowBrick) { mutableSetOf() }.add(placement)
                    restingOnMap.getOrPut(placement) { mutableSetOf() }.add(belowBrick)
                }
            }
            placement.points.forEach { space[it.index()] = placement }
            bricks.add(placement)
            return true
        } 
        
        fun safeDisintegrations() = bricks.filter { brick -> 
            val restedBricks = restedOnMap.getOrDefault(brick, emptyList())
            restedBricks.isEmpty() || restedBricks.all { restedBrick -> restingOnMap.getValue(restedBrick).size > 1 }
        }

        fun chainReactions() = bricks.sumOf { disintegrated ->
            val fallenBricks = mutableSetOf(disintegrated)
            val q = ArrayDeque<Brick>()
            q.add(disintegrated)
            generateSequence { q.removeFirstOrNull() }
                .forEach { brick ->
                    val restedBricks = restedOnMap.getOrDefault(brick, emptyList())
                        .filter { restedBrick -> fallenBricks.containsAll(restingOnMap.getValue(restedBrick)) }
                        .onEach { restedBrick -> fallenBricks.add(restedBrick) }
                    q.addAll(restedBricks)
                }
            fallenBricks.size - 1
        }
        
        private fun Xyz.index() = x + y * width + z * lengthWidth

        private fun isFree(points: Iterable<Xyz>) = points.all {
            it.z >= 1 && space.getOrNull(it.index()) == null 
        }
        
        companion object {
            private fun Iterable<Xyz>.below() = map { it + Xyz(0, 0, -1) }
        }
    }
    
    private val brickHouse: BrickHouse
    
    init {
        var maxX = 0
        var maxY = 0
        var maxZ = 0
        
        val blocks = input.getInts()
            .chunked(3) { (x, y, z) ->
                if (x > maxX) maxX = x
                if (y > maxY) maxY = y
                if (z > maxZ) maxZ = z
                Xyz(x, y, z) 
            }.chunked(2) { (from, to) -> Brick.of(from, to) }
            .toList()
            .sorted()
        brickHouse = BrickHouse(Xyz(maxX, maxY, maxZ), blocks)
    }

    override fun part1(): Int = brickHouse.safeDisintegrations().size

    override fun part2(): Int = brickHouse.chainReactions()
}

fun main() = Day.runDay(Y23D22::class)

//    Class creation: 306ms
//    Part 1: 446 (5ms)
//    Part 2: 60287 (277ms)
//    Total time: 589ms

@Suppress("unused")
private val sampleInput = listOf(
    """1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
""",
)
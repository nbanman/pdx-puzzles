package org.gristle.pdxpuzzles.advent.y2019

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.gcd
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.objects.IndexedHeap
import kotlin.math.PI
import kotlin.math.atan2

class Y19D10(input: String) : Day {

    private val asteroids: List<Coord>
    private val station: Coord
    private val detectableFromStation: Int

    init {
        val width = input.takeWhile { it != '\n' }.length
        asteroids = input
            .replace("\n", "")
            .mapIndexedNotNull { index, c -> if (c == '.') null else Coord(index % width, index / width) }
        val (detectableFromStation, station) = asteroids
            .map { asteroid ->
                (asteroids - asteroid)
                    .map { otherAsteroid ->
                        val relativeCoord = asteroid - otherAsteroid
                        val gcd = gcd(relativeCoord.x, relativeCoord.y)
                        val new = Coord(relativeCoord.x / gcd, relativeCoord.y / gcd)
                        new
                    }.distinct()
                    .size to asteroid
            }.maxByOrNull { (size) -> size }
            ?: throw Exception("no asteroids")
        this.station = station
        this.detectableFromStation = detectableFromStation
    }

    override fun part1() = detectableFromStation

    override fun part2(): Int {
        val angles = (asteroids - station)
            .map { asteroid ->
                val relativeCoord = station - asteroid
                val gcd = gcd(relativeCoord.x, relativeCoord.y)
                val new = Coord(relativeCoord.x / gcd, relativeCoord.y / gcd)
                val newAngle = atan2(new.x.toDouble(), new.y.toDouble())
                    .let { if (it <= 0.0) it else (-2 * PI) + it }
                newAngle to asteroid
            }
            .sortedBy { (_, asteroid) -> asteroid.manhattanDistance(station) }
            .groupBy { (angle) -> angle }
            .values

        val pq = IndexedHeap.maxHeap<Pair<Double, Coord>> { (angle1, _), (angle2, _) ->
            (angle1 - angle2).let { if (it < 0.0) -1 else if (it > 0.0) 1 else 0 }
        }
        for (angle in angles) {
            angle.forEachIndexed { index, (angle, asteroid) -> pq.add(-10.0 * index + angle to asteroid) }
        }

        return pq.toList()[199].let { it.second.x * 100 + it.second.y }
    }
}

fun main() = Day.runDay(Y19D10::class)

//    [2019 Day 10]
//    Class creation: 89ms
//    Part 1: 286 (0ms)
//    Part 2: 504 (9ms)
//    Total time: 99ms
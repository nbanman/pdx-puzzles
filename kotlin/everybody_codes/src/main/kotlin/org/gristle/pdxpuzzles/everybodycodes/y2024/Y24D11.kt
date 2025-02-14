package org.gristle.pdxpuzzles.everybodycodes.y2024

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day
import org.gristle.pdxpuzzles.utilities.iteration.minMax

object Y24D11 : Day {
    private fun getPopulation(input: String, days: Int, start: String): Long {
        val generations = getGenerations(input, start)
        val population = LongArray(generations.size)
        population[0] = 1
        return breed(population, generations, days)
    }

    fun minmaxPopulation(input: String): Long {
        val generations = getGenerations(input)
        val (min, max) = generations.indices
            .map { termite ->
                val population = LongArray(generations.size)
                population[termite] = 1
                breed(population, generations, 20)
            }.minMax()
        return max - min
    }

    fun getGenerations(input: String, start: String? = null): List<List<Int>> {
        val indexer = Indexer<String>()
        start?.let { indexer.put(it) }
        val generations: List<List<Int>> = input.lines()
            .map { line ->
                val (prev, next) = line.split(':')
                val id = indexer.getOrPut(prev)
                val children = next
                    .split(',')
                    .map { child -> indexer.getOrPut(child) }
                id to children
            }.sortedBy { (id, _) -> id }
            .map { (_, children) -> children }
        return generations
    }

    private fun breed(population: LongArray, generations: List<List<Int>>, days: Int): Long =
        generateSequence(population) { nextGen(it, generations) }
            .take(days + 1)
            .last()
            .sum()

    private fun nextGen(population: LongArray, generations: List<List<Int>>): LongArray {
        val nextGen = LongArray(population.size)
        for ((termite, amt) in population.withIndex()) {
            val offspring: List<Int> = generations[termite]
            for (child in offspring) {
                nextGen[child] += amt
            }
        }
        return nextGen
    }

    override fun part1(input: String): Long = getPopulation(input, 4, "A")
    override fun part2(input: String): Long = getPopulation(input, 10, "Z")
    override fun part3(input: String): Long = minmaxPopulation(input)
}

interface Indexer<T> {
    fun getOrPut(value: T): Int
    operator fun get(value: T): Int?
    fun put(value: T): Int?
    fun valueOrNull(index: Int): T?
    fun removeByIndex(index: Int): T?
    fun removeByValue(value: T): Int?
}

class IndexerImpl<T>() : Indexer<T> {
    private var id = 0
    private val indexToValue = mutableMapOf<Int, T>()
    private val valueToIndex = mutableMapOf<T, Int>()
    override fun getOrPut(value: T): Int {
        val index = valueToIndex[value]
            ?: run {
                val valueId = id++
                valueToIndex[value] = valueId
                indexToValue[valueId] = value
                valueId
            }
        return index
    }

    override fun get(value: T): Int? = valueToIndex[value]

    override fun put(value: T): Int? {
        return if (valueToIndex.contains(value)) {
            null
        } else {
            val valueId = id++
            valueToIndex[value] = valueId
            indexToValue[valueId] = value
            valueId
        }
    }

    override fun valueOrNull(index: Int): T? = indexToValue[index]

    override fun removeByIndex(index: Int): T? = indexToValue
        .remove(index)
        ?.also { valueToIndex.remove(it) }

    override fun removeByValue(value: T): Int? = valueToIndex
        .remove(value)
        ?.also { indexToValue.remove(it)  }
}

fun <T> Indexer(): Indexer<T> = IndexerImpl<T>()

fun main() = Day.runDay(Y24D11::class)

//    Quest 1: 42 (2ms)
//    Quest 2: 193253 (3ms)
//    Quest 3: 1308907399812 (13ms)
//    Total time: 19ms
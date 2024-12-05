package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import java.util.*

class Y18D07(input: String) : Day {

    // Build two maps. nextSteps shows potential steps that may be assigned once a step is completed
    private val nextSteps: Map<Char, List<Char>> = buildMap<Char, MutableList<Char>> {
        input.lineSequence().forEach { line -> getOrPut(line[5]) { mutableListOf() }.add(line[36]) }
    }

    // precedingSteps shows steps that must be completed before a given step is assigned
    private val precedingSteps: Map<Char, List<Char>> = buildMap<Char, MutableList<Char>> {
        input.lineSequence().forEach { line -> getOrPut(line[36]) { mutableListOf() }.add(line[5]) }
    }

    // starts is the list of steps that have no prerequisites, and thus can be assigned immediately
    private val starts = nextSteps.keys - precedingSteps.keys

    // Worker represents a worker, what it's working on and when it will be ready.
    private data class Worker(var workingOn: Char, var ready: Int) {
        fun isFinished(t: Int) = workingOn.isUpperCase() && ready == t
        fun isIdle() = workingOn == '.'
        fun assign(step: Char, t: Int, offset: Int) {
            workingOn = step
            ready = t + offset + step.code - 64
        }
    }

    // Solve returns both the step sequence and the amount of time required by the workers to compile the sequence.
    // This way the same function can be used for both parts.
    private fun solve(numberOfWorkers: Int, timeOffset: Int): Pair<String, Int> {

        // Use a priority queue to continuously feed the available steps in alphabetical order
        val queue = PriorityQueue<Char>().apply { addAll(starts) }

        // steps tracks the letters that have been delivered
        val steps = mutableSetOf<Char>()

        // numberOfSteps used to terminate the sequence. When steps has all the letters, it will stop.
        val numberOfSteps = (nextSteps.keys + precedingSteps.keys).size

        // represents all the workers available to perform  steps. They begin idle.
        val workerPool = List(numberOfWorkers) { Worker('.', 0) }

        // This sequence starts at second 0 and keeps adding one second.
        // Each second, it harvests completed letters from workers, adding them to the steps
        // It then assigns available letters to idle workers.
        // It terminates when steps contains all the letters, returning the # of seconds.        
        val time = generateSequence(0) { it + 1 }
            .onEach { t -> // for each second...
                // finished workers deliver product and are reset. Newly available steps are added to queue.
                workerPool
                    .filter { it.isFinished(t) } // exclude workers that are not finished
                    .forEach { worker -> // for each finished worker...
                        val product = worker.workingOn
                        steps.add(product) // add finished step to steps
                        worker.workingOn = '.' // reset worker to idle

                        // take the steps that are potentially available now that the worker has completed the step
                        // check that all other preceding steps have already been added, then add to queue
                        val next = nextSteps[product]
                            ?.filter { nextChar -> precedingSteps.getValue(nextChar).all { it in steps } }
                            ?: emptyList()
                        queue.addAll(next)
                    }

                // idle workers are assigned new jobs
                workerPool
                    .filter(Worker::isIdle) // exclude workers that are not idle
                    .forEach { worker -> // for each idle worker...
                        if (queue.isNotEmpty()) { // ...and if a step is available...
                            worker.assign(queue.poll(), t, timeOffset) // ...assign that step to the worker
                        }
                    }
            }.first { steps.size == numberOfSteps } // end sequence and return time once steps is full

        return steps.joinToString("") to time
    }

    override fun part1() = solve(1, 0).first

    override fun part2() = solve(5, 60).second
}

fun main() = Day.runDay(Y18D07::class)

//    Class creation: 12ms
//    Part 1: ABGKCMVWYDEHFOPQUILSTNZRJX (3ms)
//    Part 2: 898 (3ms)
//    Total time: 19ms
package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.math.lcm

class Y23D20(input: String) : Day {
    
    enum class Pulse { HIGH, LOW }
    
    // Java OOP style, Modules are a bunch of objects with mutating state, strung together with various global 
    // queues and lookup tables.
    sealed interface Module {
        val name: String
        val downstream: List<String>
        
        fun onReceive(signal: Signal, round: Int)
        
        fun reset()
        
        companion object {
            internal val dispatchQueue = ArrayDeque<Signal>()
            internal val lookup = mutableMapOf<String, Module>()
            internal val upstreamCount = mutableMapOf<String, Int>()
        }
    }
    
    data object Button : Module {
        override val name = "button"
        override val downstream = listOf("broadcaster")
        
        init {
            Module.lookup[name] = this
            downstream.forEach { Module.upstreamCount[it] = (Module.upstreamCount[it] ?: 0) + 1 }
        }

        override fun onReceive(signal: Signal, round: Int) {
            val output = Signal(name, downstream, Pulse.LOW)
            Module.dispatchQueue.add(output)
        }

        override fun reset() { }
    }
    
    data class BroadCaster(override val downstream: List<String>) : Module {
        override val name = "broadcaster"
        
        init {
            Module.lookup[name] = this
            downstream.forEach { Module.upstreamCount[it] = (Module.upstreamCount[it] ?: 0) + 1 }
        }

        override fun onReceive(signal: Signal, round: Int) {
            val output = Signal(name, downstream, signal.pulse)
            Module.dispatchQueue.add(output)
        }

        override fun reset() { }
    }
    
    data class FlipFlop(override val name: String, override val downstream: List<String>) : Module {
        init {
            Module.lookup[name] = this
            downstream.forEach { Module.upstreamCount[it] = (Module.upstreamCount[it] ?: 0) + 1 }
        }

        private var on = false

        override fun onReceive(signal: Signal, round: Int) {
            if (signal.pulse == Pulse.LOW) {
                on = !on
                val pulse = if (on) Pulse.HIGH else Pulse.LOW
                val output = Signal(name, downstream, pulse)
                Module.dispatchQueue.add(output)
            }
        }

        override fun reset() { 
            on = false
        }
    }
    
    data class Conjunction(override val name: String, override val downstream: List<String>) : Module {
        init {
            Module.lookup[name] = this
            downstream.forEach { Module.upstreamCount[it] = (Module.upstreamCount[it] ?: 0) + 1 }
        }

        private val upstreamPulses = mutableMapOf<String, Pulse>()
        
        private val upstreamCount: Int by lazy { Module.upstreamCount.getValue(name) }
        
        override fun onReceive(signal: Signal, round: Int) {
            upstreamPulses[signal.sender] = signal.pulse
            val pulse = if (upstreamCount == upstreamPulses.size &&
                upstreamPulses.values.all { it == Pulse.HIGH }) Pulse.LOW else Pulse.HIGH
            val output = Signal(name, downstream, pulse)
            Module.dispatchQueue.add(output)
        }

        override fun reset() { 
            upstreamPulses.clear()
        }
    }
    
    data class Signal(val sender: String, val recipients: List<String>, val pulse: Pulse) {
        fun send(round: Int) {
            recipients.forEach { name -> Module.lookup[name]?.onReceive(this, round) }
        }
    }

    init {
        // creates modules from input. No need to assign a variable because the objects are stored in a global lookup
        // table.
        input.lines().forEach { line ->
            val (nameStr, downstreamStr) = line.split(" -> ")
            val downstream = downstreamStr.split(", ").toList()
            when (nameStr[0]) {
                '%' -> FlipFlop(nameStr.drop(1), downstream)
                '&' -> Conjunction(nameStr.drop(1), downstream)
                else -> BroadCaster(downstream)
            }
        }
    }
    
    // Simulates one round by pushing the button, then processing signals until none are sent.
    // Returns the number of high and low pulses sent, which is used for Part 1. Part 2 relies on side effects.
    private fun pressButton(round: Int): Pair<Int, Int> {
        Button.onReceive(Signal(Button.name, listOf("broadcaster"), Pulse.LOW), 0)
        return generateSequence { Module.dispatchQueue.removeFirstOrNull() }
            .fold(0 to 0) { (high, low), signal ->
                signal.send(round)
                if (signal.pulse == Pulse.HIGH) {
                    high + signal.recipients.size to low
                } else {
                    high to low + signal.recipients.size
                }
            }
    }
    
    // Runs sequence 1000 times, sums up the high and low pulses sent, and multiplies the two together.
    override fun part1(): Int {
        Module.lookup.values.forEach { it.reset() }
        return generateSequence(1, Int::inc).map { pressButton(it) }
            .take(1000)
            .fold(0 to 0) { (sumHigh, sumLow), (high, low) ->
                sumHigh + high to sumLow + low
            }.let { (high, low) ->
                high * low
            }
    } 
        
    // Recognizing that the input creates a set of binary counters that all must emit at the same time, analyze those
    // counters and determine when they individually emit. Then take the lcm of all the counters.
    // The broadcaster sends to four separate counters. These are chains of FlipFlop modules connected to a Conjunction
    // clock module. The FlipFlop modules that send a signal back to the clock become a "1" and the modules that 
    // receive a signal from the clock become a "0". Put these in reverse order, and you have a binary representation
    // of how many cycles it takes to trigger the Conjunction module.
    override fun part2(): Long {
        val flipFlops = Module.lookup.values.filterIsInstance<FlipFlop>().map { it.name }.toSet()
        val binaryCounterResults = Module.lookup.getValue("broadcaster").downstream
            .map { name ->
                val start = Module.lookup.getValue(name)
                val conjunction = start.downstream.first { Module.lookup.getValue(it) is Conjunction }
                generateSequence(start) { module -> Module.lookup[module.downstream.firstOrNull { it in flipFlops }] }
                    .map { if (conjunction in it.downstream) 1 else 0 }
                    .foldIndexed(0L) { index, acc, i -> acc + (i shl index) }
            }
        return lcm(binaryCounterResults)
    }
}

fun main() = Day.runDay(Y23D20::class)

//    Class creation: 9ms
//    Part 1: 938065580 (24ms)
//    Part 2: 250628960065793 (2ms)
//    Total time: 37ms

@Suppress("unused")
private val sampleInput = listOf(
    """broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
""", """broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
""", 
)
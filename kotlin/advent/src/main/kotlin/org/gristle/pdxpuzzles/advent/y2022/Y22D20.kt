package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getIntList

class Y22D20(input: String) : Day {

    /**
     * Basic DLL supporting a move function that moves the node n times in accordance with the puzzle rules.
     */
    class Node(
        val value: Long,
        prev: Node? = null,
        next: Node? = null,
    ) {
        var prev: Node = prev
            ?.also { it.next = this }
            ?: this
        var next: Node = next
            ?.also { it.prev = this }
            ?: this

        fun move(size: Int) {

            val steps = value.mod(size - 1)

            if (steps == 0) return

            val moveNode = if (steps > size / 2) {
                generateSequence(this, Node::prev).take(size - steps + 1).last()
            } else {
                generateSequence(this, Node::next).take(steps + 1).last()
            }

            // take this node out of DLL 
            prev.next = next
            next.prev = prev

            // fix this node's pointers
            next = moveNode.next
            prev = moveNode

            // fix the pointers that should point to this node
            moveNode.next.prev = this
            moveNode.next = this
        }

        override fun toString(): String {
            return "Node(value=$value, prev=${prev.value}, next=${next.value})"
        }
    }

    private val initialValues = input.getIntList()

    fun solve(factor: Long, times: Int): Long {

        // get list of numbers
        val values: List<Long> = initialValues.map { it * factor }

        // build linked list, and also put each node into a standard List so that we can loop through the nodes in
        // original order. 'nodes' is the List in original order. The DLL does not have a head but we have access to
        // all the nodes through the 'nodes' List.
        val nodes = buildList {
            val header = Node(values.first())
            add(header)
            var previous = header
            for (n in values.drop(1)) {
                previous = Node(n, previous, header)
                add(previous)
            }
        }

        // repeat moving process per part 2 rules
        repeat(times) {

            // perform move for each node in original order.
            for (node in nodes) {
                node.move(nodes.size)
            }
        }

        // gets the node valued at 0. This is the beginning node for the problem calculations.
        val zeroNode = nodes.find { it.value == 0L } ?: throw IllegalStateException("0 not found in nodes")

        // defines the DLL traversal sequence, starting at the zero node and moving forward
        val traverseSequence = generateSequence(zeroNode, Node::next).drop(1)
        fun traverse(n: Int) = traverseSequence.take(n).last().value
        
        // delivers sum of the 1000th, 2000th, and 3000th
        return (1000..3000 step 1000).sumOf(::traverse)
    }

    override fun part1() = solve(1, 1)

    override fun part2() = solve(811589153, 10)
}

fun main() = Day.runDay(Y22D20::class)

//    Class creation: 10ms
//    Part 1: 4151 (60ms)
//    Part 2: 7848878698663 (314ms)
//    Total time: 385ms
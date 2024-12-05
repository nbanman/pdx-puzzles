package org.gristle.pdxpuzzles.advent.y2017

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y17D25(input: String) : Day {
    private class Node(var isOne: Boolean = false, var left: Node? = null, var right: Node? = null) {
        fun value() = if (isOne) 1 else 0

        fun moveLeft(): Node {
            left = left ?: Node(false, null, this)
            return left as Node
        }

        fun moveRight(): Node {
            right = right ?: Node(false, this, null)
            return right as Node
        }

        fun sumList(): Int {
            return (sumLeft() + sumRight() - value())
        }

        private fun sumLeft(): Int {
            return value() + (left?.sumLeft() ?: 0)
        }

        private fun sumRight(): Int {
            return value() + (right?.sumRight() ?: 0)
        }
    }

    private data class State(
        val zeroWrite: Boolean,
        val zeroLeft: Boolean,
        val zeroChange: String,
        val oneWrite: Boolean,
        val oneLeft: Boolean,
        val oneChange: String
    )

    private val pattern = """In state ([A-F]):
  If the current value is 0:
    - Write the value ([01]).
    - Move one slot to the (left|right).
    - Continue with state ([A-F]).
  If the current value is 1:
    - Write the value ([01]).
    - Move one slot to the (left|right).
    - Continue with state ([A-F])."""

    private val data = input.replace("\r", "")

    override fun part1(): Int {
        val steps: Int = data.getInts().first()

        val states: Map<String, State> = pattern
            .toRegex()
            .findAll(data)
            .associate { result ->
                val (name, zeroWrite, zeroLeft, zeroChange, oneWrite, oneLeft, oneChange) = result.destructured
                name to State(
                    zeroWrite == "1",
                    zeroLeft == "left",
                    zeroChange,
                    oneWrite == "1",
                    oneLeft == "left",
                    oneChange
                )
            }

        val startNode = Node()
        var currentNode = startNode
        var currentName = "A"
        for (x in 1..steps) {
            val state = states.getValue(currentName)
            if (currentNode.isOne) {
                currentNode.isOne = state.oneWrite
                currentNode = if (state.oneLeft) currentNode.moveLeft() else currentNode.moveRight()
                currentName = state.oneChange
            } else {
                currentNode.isOne = state.zeroWrite
                currentNode = if (state.zeroLeft) currentNode.moveLeft() else currentNode.moveRight()
                currentName = state.zeroChange
            }
        }
        return startNode.sumList()
    }

    override fun part2() = true
}

fun main() = Day.runDay(Y17D25::class)

//    Class creation: 10ms
//    Part 1: 3745 (150ms)
//    Total time: 160ms

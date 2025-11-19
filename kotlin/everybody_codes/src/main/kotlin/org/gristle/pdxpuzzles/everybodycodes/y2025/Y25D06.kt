package org.gristle.pdxpuzzles.everybodycodes.y2025

import org.gristle.pdxpuzzles.everybodycodes.utilities.Day

object Y25D06 : Day {
    override fun part1(input: String): Int {
        var knights = 0
        var pairs = 0
        for (c in input) {
            when (c) {
                'A' -> knights++
                'a' -> pairs += knights
                else -> {}
            }
        }
        return pairs
    }
    override fun part2(input: String): Int {
        val knights = IntArray(3)
        var pairs = 0
        for (c in input) {
            when {
                c.isUpperCase() -> knights[c - 'A'] += 1
                else -> pairs += knights[c - 'a']
            }
        }
        return pairs
    }
    override fun part3(input: String): Int {
        val preRepeat = IntArray(3)
        val pre = IntArray(3)
        val postRepeat = IntArray(3)
        val post = IntArray(3)
        var pairs = 0

        // constants
        val distance = 1000
        val repeats = 1000

        // pre-fill
        for (c in input.takeLast(distance)) {
            if (c.isUpperCase()) {
                preRepeat[c - 'A']++
            }
        }
        for (c in input.take(distance)) {
            if (c.isUpperCase()) {
                post[c - 'A']++
            }
        }

        // main loop
        for ((idx, c) in input.withIndex()) {
            if (c.isUpperCase()) {
                post[c - 'A']--
            } else {
                val cIdx = c - 'a'
                pairs += preRepeat[cIdx] * (repeats - 1) +
                        pre[cIdx] * repeats +
                        post[cIdx] * repeats +
                        postRepeat[cIdx] * (repeats - 1)
            }

            //drop stuff behind
            val preIdx = idx - distance
            if (preIdx < 0) {
                val preDrop = input[preIdx.mod(input.length)]
                if (preDrop.isUpperCase()) {
                    preRepeat[preDrop - 'A']--
                }
            } else {
                val preDrop = input[preIdx]
                if (preDrop.isUpperCase()) {
                    pre[preDrop - 'A']--
                }
            }
            val postIdx = idx + distance + 1
            if (postIdx >= input.length) {
                val postDrop = input[postIdx % input.length]
                if (postDrop.isUpperCase()) {
                    preRepeat[postDrop - 'A']++
                }
            } else {
                val postDrop = input[postIdx]
                if (postDrop.isUpperCase()) {
                    pre[postDrop - 'A']++
                }
            }
            if (c.isUpperCase()) {
                pre[c - 'A']++
            }
        }
        return pairs
    }
}

fun main() = Day.runDay(Y25D06::class)

//    Quest 1: 190 (0ms)
//    Quest 2: 4011 (3ms)
//    Quest 3: 1665939853 (3ms)
//    Total time: 6ms

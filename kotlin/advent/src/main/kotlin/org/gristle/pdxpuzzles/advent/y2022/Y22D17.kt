package org.gristle.pdxpuzzles.advent.y2022

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import kotlin.math.max

class Y22D17(private val jetPattern: String) : Day {

    data class BlockInfo(val wind: Int, val x: Int = 2, val blockType: Int, val top: Int, val index: Int) {
        override fun equals(other: Any?): Boolean {
            if (this === other) return true
            if (javaClass != other?.javaClass) return false

            other as BlockInfo

            if (wind != other.wind) return false
            if (x != other.x) return false
            if (blockType != other.blockType) return false

            return true
        }

        override fun hashCode(): Int {
            var result = wind
            result = 31 * result + x
            result = 31 * result + blockType
            return result
        }
    }

    data class Block(val index: Int, val wind: Int, val x: Int, val y: Int, val previousTop: Int) {

        @Suppress("unused", "MemberVisibilityCanBePrivate")
        fun BooleanArray.representation() = buildString {
            for (y in top downTo 0) {
                append('|')
                for (x in 0..6) {
                    append(if (this@representation[y * 7 + x]) '@' else '.')
                }
                append("|\n")
            }
            append("+-------+\n")
        }

        private fun block() = blocks[index % 5]

        fun toIndices() = toCoords().map { it.asIndex(7) }

        val top: Int get() = y + block().maxOf { it.y } + 1

        val height: Int get() = max(top, previousTop)

        private fun toCoords() = block().map { it + Coord(x, y) }

        fun toBlockInfo() = BlockInfo(wind, x, index % 5, max(top, previousTop), index)

        fun drop(chamber: BooleanArray, jetPattern: String): Block {
            val provisionalX = if (jetPattern[wind] == '>') x + 1 else x - 1
            val provisionalBlock = block().map { it + Coord(provisionalX, y) }
            val newX = if (provisionalBlock.any { it.x !in 0..6 }) {
                x
            } else {
                val provisionalIndices = provisionalBlock.map { it.asIndex(7) }
                if (provisionalIndices.any { chamber[it] }) {
                    x
                } else {
                    provisionalX
                }
            }

            val below = block().map { blockCoords ->
                (blockCoords + Coord(newX, y - 1)).let {
                    it.y * 7 + it.x
                }
            }
            return if (y != 0 && below.none { chamber[it] }) {
                Block(index, (wind + 1) % jetPattern.length, newX, y - 1, previousTop)
                    .drop(chamber, jetPattern)
            } else {
                Block(index, wind, newX, y, previousTop)
            }
        }

        companion object {
            private val blocks = listOf(
                listOf(Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(3, 0)),
                listOf(Coord(1, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1), Coord(1, 2)),
                listOf(Coord(2, 0), Coord(2, 1), Coord(2, 2), Coord(0, 0), Coord(1, 0)),
                listOf(Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(0, 3)),
                listOf(Coord(0, 0), Coord(1, 0), Coord(0, 1), Coord(1, 1)),
            )
        }
    }

    private val blocks = sequence {
        val chamber = BooleanArray(10_000_000)
        var block = Block(0, 0, 2, 3, 0).drop(chamber, jetPattern)
        while (true) {
            yield(block)
            block.toIndices().forEach { chamber[it] = true }
//            println(chamber.representation(block))
            block = Block(
                block.index + 1,
                (block.wind + 1) % jetPattern.length,
                2,
                max(block.top, block.previousTop) + 3,
                max(block.top, block.previousTop)
            ).drop(chamber, jetPattern)
        }
    }

    @Suppress("unused", "MemberVisibilityCanBePrivate")
    fun BooleanArray.representation(lastBlock: Block) = buildString {
        for (y in lastBlock.top downTo 0) {
            append('|')
            for (x in 0..6) {
                append(if (this@representation[y * 7 + x]) '@' else '.')
            }
            append("|\n")
        }
        append("+-------+\n")
    }

    override fun part1() = blocks
        .take(2022)
        .last()
        .top

    override fun part2(): Long {
        val blockInfo = mutableSetOf<BlockInfo>()
        var blockList = emptyList<BlockInfo>()
        var preIndex = 0
        var nextIndex = 0
        var lastBeforeRepeat: BlockInfo? = null
        var previous: BlockInfo? = null
        blocks.first { block ->
            val bi = block.toBlockInfo()
            if (blockList.isEmpty()) {
                if (!blockInfo.add(bi)) {
                    blockList = blockInfo.toList()
                    nextIndex = blockInfo.indexOf(bi)
                    preIndex = nextIndex - 1
                    blockInfo.clear()
                }
                false
            } else {
                if (nextIndex == blockList.lastIndex) {
                    true
                } else {
                    if (blockList[++nextIndex] != bi) {
                        lastBeforeRepeat = previous
                        blockList = emptyList()
                        blockInfo.add(bi)
                    } else {
                        previous = bi
                    }
                    false
                }
            }
        }
        val previousTop = lastBeforeRepeat?.top ?: throw Error()
        val previousIndex = lastBeforeRepeat?.index ?: throw Error()
        val prevLines = if (preIndex == -1) previousTop else blockList[preIndex].top
        val blocksInRepeat = blockList.size - (preIndex + 1)
        val linesInRepeat = blockList.last().top - prevLines
        val repeats = (1_000_000_000_000L - previousIndex + 1) / blocksInRepeat
        val endBlocks = (1_000_000_000_000L - (previousIndex + 1) - repeats * blocksInRepeat).toInt()
        val endLines = blockList[preIndex + endBlocks].top - prevLines
        return prevLines + repeats * linesInRepeat + endLines
    }
}

fun main() = Day.runDay(Y22D17::class) // 3055, 1507692307690
//    Class creation: 2ms
//    Part 1: 3055 (23ms)
//    Part 2: 1507692307690 (41ms)
//    Total time: 68ms
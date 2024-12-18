@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.objects

import org.gristle.pdxpuzzles.utilities.algorithms.Graph
import kotlin.math.min

interface Grid<out E> : List<E> {
    val width: Int
    val height: Int
    val xIndices: IntRange
    val yIndices: IntRange

    fun representation(display: (E) -> Char): String

    operator fun get(x: Int, y: Int): E
    operator fun get(coord: Coord): E

    fun getOrNull(x: Int, y: Int): E?
    fun getOrNull(coord: Coord): E?

    fun coordOf(n: Int): Coord

    fun coordOfElement(e: @UnsafeVariance E): Coord

    fun lastCoordOfElement(e: @UnsafeVariance E): Coord

    fun lastCoord(): Coord

    fun coords(): List<Coord>

    fun indexOf(x: Int, y: Int): Int

    fun indexOf(coord: Coord): Int

    fun indexOfOrNull(x: Int, y: Int): Int?

    fun indexOfOrNull(coord: Coord): Int?

    fun subGrid(coord: Coord, newWidth: Int, newHeight: Int): Grid<E>

    fun subGrid(topLeft: Coord, bottomRight: Coord): Grid<E>

    fun rows(): List<List<E>>

    fun row(row: Int): List<E>

    fun columns(): List<List<E>>

    fun column(column: Int): List<E>

    fun columnsSequence(): Sequence<List<E>>

    fun rowsSequence(): Sequence<List<E>>

    fun getNeighborIndices(index: Int, includeDiagonals: Boolean = false, wrapAround: Boolean = false): List<Int>

    fun getNeighborIndices(x: Int, y: Int, includeDiagonals: Boolean = false, wrapAround: Boolean = false): List<Int>

    fun getNeighborIndices(coord: Coord, includeDiagonals: Boolean = false, wrapAround: Boolean = false): List<Int>

    fun getNeighbors(coord: Coord, includeDiagonals: Boolean = false, wrapAround: Boolean = false): List<E>

    fun getNeighbors(x: Int, y: Int, includeDiagonals: Boolean = false, wrapAround: Boolean = false): List<E>

    fun getNeighbors(index: Int, includeDiagonals: Boolean = false, wrapAround: Boolean = false): List<E>

    fun getNeighborsIndexedValue(
        coord: Coord, includeDiagonals: Boolean = false, wrapAround: Boolean = false
    ): List<IndexedValue<E>>

    fun getNeighborsIndexedValue(
        x: Int, y: Int, includeDiagonals: Boolean = false, wrapAround: Boolean = false
    ): List<IndexedValue<E>>

    fun getNeighborsIndexedValue(
        index: Int, includeDiagonals: Boolean = false, wrapAround: Boolean = false
    ): List<IndexedValue<E>>

    fun rotate270(): Grid<E>
    fun rotate180(): Grid<E>
    fun rotate90(): Grid<E>
    fun flipX(): Grid<E>
    fun flipY(): Grid<E>
    fun validCoord(coord: Coord): Boolean

    fun getBounds(predicate: (E) -> Boolean): Pair<IntRange, IntRange>

    fun getDimensionsAndOffset(padding: Int = 0, predicate: (E) -> Boolean): Pair<Coord, Coord>
}

interface MutableGrid<E> : Grid<E>, MutableList<E> {
    operator fun set(coord: Coord, e: E): E
    operator fun set(x: Int, y: Int, e: E): E
}

class ArrayGrid<E> private constructor(
    private val elementsImpl: ArrayList<E>,
    override val width: Int
) : MutableGrid<E>, MutableList<E> by elementsImpl {
    constructor(elements: Collection<E>, width: Int) : this (ArrayList(elements), width)

    override var height = size / width
    override val xIndices = 0 until width
    override val yIndices = 0 until height
    init {
        require(height * width == size)
    }


    override fun validCoord(coord: Coord) = coord.x in xIndices && coord.y in yIndices

    override fun representation(display: (E) -> Char): String {
        return buildString {
            for (row in yIndices) {
                for (col in xIndices) {
                    append(display(get(col, row)))
                }
                append('\n')
            }
        }
    }

    override operator fun get(x: Int, y: Int): E = get(indexOf(x, y))

    override operator fun get(coord: Coord): E = get(indexOf(coord.x, coord.y))

    override fun getOrNull(x: Int, y: Int): E? =
        if (validCoord(Coord(x, y))) {
            get(indexOf(x, y))
        } else {
            null
        }

    override fun getOrNull(coord: Coord): E? =
        if (validCoord(coord)) {
            get(indexOf(coord))
        } else {
            null
        }

    override fun coordOf(n: Int): Coord = Coord(n % width, n / width)

    override fun coordOfElement(e: @UnsafeVariance E): Coord = coordOf(indexOf(e))

    override fun lastCoordOfElement(e: @UnsafeVariance E): Coord = coordOf(lastIndexOf(e))

    override fun lastCoord(): Coord = coordOf(lastIndex)

//    override fun coords(): List<Coord> =
//        (0 until height).flatMap { y -> (0 until width).map { x -> Coord(x, y) } }

    override fun coords(): List<Coord> = indices.map { coordOf(it) }

    override fun indexOf(x: Int, y: Int): Int = y * width + x

    override fun indexOf(coord: Coord) = indexOf(coord.x, coord.y)

    override fun indexOfOrNull(x: Int, y: Int): Int? = when {
        x !in xIndices -> null
        y !in yIndices -> null
        else -> y * width + x
    }

    override fun indexOfOrNull(coord: Coord): Int? = when {
        coord.x !in xIndices -> null
        coord.y !in yIndices -> null
        else -> coord.y * width + coord.x
    }

    override fun subGrid(coord: Coord, newWidth: Int, newHeight: Int): Grid<E> {
        require(newWidth > 0 && newHeight > 0)
        val adjustedWidth = min(width - coord.x, newWidth)
        val adjustedHeight = min(height - coord.y, newHeight)
        return Grid(adjustedWidth, adjustedHeight) { i ->
            val newCoord = Coord(i % adjustedWidth, i / adjustedWidth)
            get(coord + newCoord)
        }
    }

    override fun subGrid(topLeft: Coord, bottomRight: Coord): Grid<E> {
        val diff = bottomRight - topLeft
        return subGrid(topLeft, diff.x + 1, diff.y + 1)
    }

    override fun rows(): List<List<E>> {
        return yIndices.fold(emptyList()) { acc, i ->
            acc + listOf(subList(i * width, i * width + width))
        }
    }

    override fun row(row: Int) = subList(row * width, row * width + width)

    override fun columns(): List<List<E>> {
        return xIndices.fold(emptyList()) { acc, i ->
            acc + listOf(List(height) { index -> get(i + index * width) })
        }
    }

    override fun columnsSequence(): Sequence<List<E>> = xIndices.asSequence().map { column(it) }

    override fun rowsSequence(): Sequence<List<E>> = yIndices.asSequence().map { row(it) }

    override fun column(column: Int) = List(height) { i -> get(column + i * width) }

    override fun getNeighborIndices(
        index: Int,
        includeDiagonals: Boolean,
        wrapAround: Boolean
    ): List<Int> = getNeighborIndices(coordOf(index), includeDiagonals, wrapAround)

    override fun getNeighborIndices(
        x: Int,
        y: Int,
        includeDiagonals: Boolean,
        wrapAround: Boolean
    ): List<Int> = getNeighborsIndexedValue(x, y, includeDiagonals, wrapAround).map { it.index }

    override fun getNeighborIndices(coord: Coord, includeDiagonals: Boolean, wrapAround: Boolean) =
        getNeighborIndices(coord.x, coord.y, includeDiagonals, wrapAround)

    override fun getNeighbors(
        coord: Coord,
        includeDiagonals: Boolean,
        wrapAround: Boolean
    ) = getNeighbors(coord.x, coord.y, includeDiagonals, wrapAround)

    override fun getNeighbors(
        x: Int,
        y: Int,
        includeDiagonals: Boolean,
        wrapAround: Boolean
    ): List<E> {
        return if (includeDiagonals) {
            listOf(
                x - 1 to y - 1,
                x to y - 1,
                x + 1 to y - 1,
                x - 1 to y,
                x + 1 to y,
                x - 1 to y + 1,
                x to y + 1,
                x + 1 to y + 1
            )
        } else {
            listOf(
                x to y - 1,
                x - 1 to y,
                x + 1 to y,
                x to y + 1
            )
        }.mapNotNull { (x, y) ->
            val neighborX = if (wrapAround) {
                when (x) {
                    -1 -> width - 1
                    width -> 0
                    else -> x
                }
            } else x

            val neighborY = if (wrapAround) {
                when (y) {
                    -1 -> height - 1
                    height -> 0
                    else -> y
                }
            } else y

            if (wrapAround || (neighborX in xIndices && neighborY in yIndices)) {
                this[neighborX, neighborY]
            } else {
                null
            }
        }
    }

    override fun getNeighbors(index: Int, includeDiagonals: Boolean, wrapAround: Boolean) =
        getNeighbors(coordOf(index), includeDiagonals, wrapAround)

    override fun getNeighborsIndexedValue(
        coord: Coord, includeDiagonals: Boolean, wrapAround: Boolean
    ): List<IndexedValue<E>> = getNeighborsIndexedValue(coord.x, coord.y, includeDiagonals, wrapAround)

    override fun getNeighborsIndexedValue(
        x: Int, y: Int, includeDiagonals: Boolean, wrapAround: Boolean
    ): List<IndexedValue<E>> {
        return if (includeDiagonals) {
            listOf(
                x - 1 to y - 1,
                x to y - 1,
                x + 1 to y - 1,
                x - 1 to y,
                x + 1 to y,
                x - 1 to y + 1,
                x to y + 1,
                x + 1 to y + 1
            )
        } else {
            listOf(
                x to y - 1,
                x - 1 to y,
                x + 1 to y,
                x to y + 1
            )
        }.mapNotNull { (x, y) ->
            val neighborX = if (wrapAround) {
                when (x) {
                    -1 -> width - 1
                    width -> 0
                    else -> x
                }
            } else x

            val neighborY = if (wrapAround) {
                when (y) {
                    -1 -> height - 1
                    height -> 0
                    else -> y
                }
            } else y

            if (wrapAround || (neighborX in xIndices && neighborY in yIndices)) {
                IndexedValue(indexOf(neighborX, neighborY), this[neighborX, neighborY])
            } else {
                null
            }
        }
    }

    override fun getNeighborsIndexedValue(
        index: Int, includeDiagonals: Boolean, wrapAround: Boolean
    ): List<IndexedValue<E>> = getNeighborsIndexedValue(coordOf(index), includeDiagonals, wrapAround)

    private inline fun rotate(changeShape: Boolean, transform: (Int) -> Int): Grid<E> {
        return List(size) { i ->
            get(transform(i))
        }.toGrid(if (changeShape) height else width)
    }

    override fun rotate270() = rotate(true) { (width - 1 - it / height) + it % height * width }
    override fun rotate180() = rotate(false) { size - 1 - it }
    override fun rotate90() = rotate(true) { (size - width) - (it % height) * width + it / height }
    override fun flipX() = rotate(false) { (size - width) - (width * (it / width)) + it % width }
    override fun flipY() = rotate(false) { (it / width) * width + (width - 1) - it % width }

    // From MutableGrid
    private fun Boolean.changeHeight(): Boolean {
        if (this) height = size / width
        return this
    }

    override fun set(coord: Coord, e: E): E = set(indexOf(coord), e)

    override fun set(x: Int, y: Int, e: E): E = set(indexOf(Coord(x, y)), e)

    override fun add(element: E): Boolean {
        return elementsImpl
            .add(element)
            .changeHeight()
    }

    override fun remove(element: E): Boolean {
        return elementsImpl
            .remove(element)
            .changeHeight()
    }

    override fun getBounds(predicate: (E) -> Boolean): Pair<IntRange, IntRange> {
        val xMin = columnsSequence().indexOfFirst { it.any(predicate) }
        val xMax = generateSequence(width - 1) { it - 1 }
            .map { if (it < 0) null else IndexedValue(it, column(it)) }
            .first { it?.value?.any(predicate) ?: false }
            ?.index
            ?: -1
        val yMin = rowsSequence().indexOfFirst { it.any(predicate) }
        val yMax = generateSequence(height - 1) { it - 1 }
            .map { if (it < 0) null else IndexedValue(it, row(it)) }
            .first { it?.value?.any(predicate) ?: false }
            ?.index
            ?: -1

        return xMin..xMax to yMin..yMax
    }

    override fun getDimensionsAndOffset(padding: Int, predicate: (E) -> Boolean): Pair<Coord, Coord> {
        val (xBound, yBound) = getBounds(predicate)
        val dimensions = Coord(
            xBound.last - xBound.first + 1 + padding * 2,
            yBound.last - yBound.first + 1 + padding * 2
        )
        val offset = Coord(
            padding - xBound.first,
            padding - yBound.first
        )
        return dimensions to offset
    }

    // Bulk Modification Operations
    /**
     * Adds all the elements of the specified collection to the end of this list.
     *
     * The elements are appended in the order they appear in the [elementsImpl] collection.
     *
     * @return `true` if the list was changed as the result of the operation.
     */
    override fun addAll(elements: Collection<E>): Boolean {
        return elementsImpl
            .addAll(elements)
            .changeHeight()
    }

    /**
     * Inserts all the elements of the specified collection [elementsImpl] into this list at the specified [index].
     *
     * @return `true` if the list was changed as the result of the operation.
     */
    override fun addAll(index: Int, elements: Collection<E>): Boolean {
        return elementsImpl
            .addAll(index, elements)
            .changeHeight()
    }

    override fun removeAll(elements: Collection<E>): Boolean {
        return elementsImpl
            .addAll(elements)
            .changeHeight()
    }

    override fun retainAll(elements: Collection<E>): Boolean {
        return elementsImpl
            .retainAll(elements.toSet())
            .changeHeight()
    }

    override fun clear() {
        elementsImpl.clear()
        true.changeHeight()
    }

    /**
     * Inserts an element into the list at the specified [index].
     */
    override fun add(index: Int, element: E) {
        elementsImpl.add(index, element)
        height = size / width
    }

    /**
     * Removes an element at the specified [index] from the list.
     *
     * @return the element that has been removed.
     */
    override fun removeAt(index: Int): E {
        return elementsImpl
            .removeAt(index)
            .apply { height = size / width }
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as ArrayGrid<*>

        if (elementsImpl != other.elementsImpl) return false
        if (width != other.width) return false

        return true
    }

    override fun hashCode(): Int {
        var result = elementsImpl.hashCode()
        result = 31 * result + width
        return result
    }
}

fun <E> gridOf(width: Int, vararg elements: E): Grid<E> = elements.toGrid(width)

fun <E> mutableGridOf(width: Int, vararg elements: E): Grid<E> = elements.toMutableGrid(width)

inline fun <T> Grid(width: Int, height: Int, init: (index: Int) -> T): Grid<T> = MutableGrid(width, height, init)

inline fun <T> MutableGrid(width: Int, height: Int, init: (index: Int) -> T): MutableGrid<T> {
    val list = ArrayList<T>(width * height)
    repeat(width * height) { index -> list.add(init(index)) }
    return ArrayGrid(list, width)
}

fun <E> Array<E>.toGrid(width: Int): Grid<E> = ArrayGrid(this.toList(), width)

fun <E> List<E>.toGrid(width: Int): Grid<E> = ArrayGrid(this, width)

fun <E> Sequence<E>.toGrid(width: Int): Grid<E> = toMutableGrid(width)

fun <E> Sequence<E>.toMutableGrid(width: Int): Grid<E> = toMutableList().toMutableGrid(width)

fun <E> Grid<E>.toGrid(): Grid<E> = ArrayGrid(this, this.width)

fun String.toGrid(width: Int): Grid<Char> = ArrayGrid(this.toList(), width)

fun String.toGrid(padding: Char = ' '): Grid<Char> {
    val width = indexOfAny(charArrayOf('\n', '\r')).let { if (it == -1) 1 else it }
    val noBreaks = replace("[\r\n]+".toRegex(), "")
    val padWidth = padding.toString().repeat((width - noBreaks.length % width) % width)
    return (noBreaks + padWidth).toGrid(width)
}

inline fun <R> String.toGrid(transform: (Char) -> R): Grid<R> {
    val width = indexOfAny(charArrayOf('\n', '\r')).let { if (it == -1) 1 else it }
    return replace("[\r\n]+".toRegex(), "")
        .map { transform(it) }
        .toGrid(width)
}

inline fun <R> String.toGridIndexed(transform: (index: Int, Char) -> R): Grid<R> {
    val width = indexOfAny(charArrayOf('\n', '\r')).let { if (it == -1) 1 else it }
    return replace("[\r\n]+".toRegex(), "")
        .mapIndexed { index, c -> transform(index, c) }
        .toGrid(width)
}

fun <E> Array<E>.toMutableGrid(width: Int): MutableGrid<E> = ArrayGrid(this.toList(), width)

fun <E> List<E>.toMutableGrid(width: Int): MutableGrid<E> = ArrayGrid(this, width)

fun <E> Grid<E>.toMutableGrid(): MutableGrid<E> = ArrayGrid(this, this.width)

fun String.toMutableGrid(width: Int): MutableGrid<Char> = ArrayGrid(this.toList(), width)

fun String.toMutableGrid(): MutableGrid<Char> {
    val width = indexOfAny(charArrayOf('\n', '\r')).let { if (it == -1) 1 else it }
    return replace("[\r\n]+".toRegex(), "").toMutableGrid(width)
}

inline fun <R> String.toMutableGrid(transform: (Char) -> R): MutableGrid<R> {
    val width = indexOfAny(charArrayOf('\n', '\r')).let { if (it == -1) 1 else it }
    return replace("[\r\n]+".toRegex(), "")
        .map { transform(it) }
        .toMutableGrid(width)
}

inline fun <R> String.toMutableGridIndexed(transform: (index: Int, Char) -> R): MutableGrid<R> {
    val width = indexOfAny(charArrayOf('\n', '\r')).let { if (it == -1) 1 else it }
    return replace("[\r\n]+".toRegex(), "")
        .mapIndexed { index, c -> transform(index, c) }
        .toMutableGrid(width)
}

inline fun <R> String.toMutableGridPos(transform: (pos: Coord, Char) -> R): MutableGrid<R> {
    val width = indexOfAny(charArrayOf('\n', '\r')).let { if (it == -1) 1 else it }
    return replace("[\r\n]+".toRegex(), "")
        .mapIndexed { index, c ->
            val pos = Coord.fromIndex(index, width)
            transform(pos, c)
        }.toMutableGrid(width)
}

fun <E> Grid<E>.addRight(addGrid: Grid<E>): Grid<E> {
    require(height == addGrid.height) {
        "New grid must have the same height (${addGrid.height}) as the one added to (${height})."
    }
    return List(size + addGrid.size) { index ->
        val coord = index.toCoord(width + addGrid.width)
        if (coord.x in xIndices) {
            get(coord)
        } else {
            addGrid[coord.x - width, coord.y]
        }
    }.toGrid(width + addGrid.width)
}

fun <E> Grid<E>.addDown(addGrid: Grid<E>): Grid<E> {
    require(width == addGrid.width) {
        "New grid must have the same width (${addGrid.width}) as the one added to (${width})."
    }
    return List(size + addGrid.size) { index ->
        val coord = index.toCoord(width)
        if (coord.y in yIndices) {
            get(coord)
        } else {
            addGrid[coord.x, coord.y - height]
        }
    }.toGrid(width)
}

inline fun <E, R> Grid<E>.mapToGrid(transform: (E) -> R): Grid<R> {
    return map { transform(it) }.toGrid(width)
}

inline fun <E, R> Grid<E>.mapToGridIndexed(transform: (index: Int, E) -> R): Grid<R> {
    return mapIndexed { index, e -> transform(index, e) }.toGrid(width)
}

inline fun <E, R> Grid<E>.mapToMutableGrid(transform: (E) -> R): MutableGrid<R> {
    return map { transform(it) }.toMutableGrid(width)
}

inline fun <E, R> Grid<E>.mapToMutableGridIndexed(transform: (index: Int, E) -> R): MutableGrid<R> {
    return mapIndexed { index, e -> transform(index, e) }.toMutableGrid(width)
}

@JvmName("BooleanRep")
fun Grid<Boolean>.rep() = representation { if (it) '*' else '.' }

fun Grid<Char>.rep() = representation { it }

fun Grid<Char>.getEdgeMapIndexed(ignore: String = "#. "): Map<IndexedValue<Char>, List<Graph.Edge<IndexedValue<Char>>>> {
    val edgeMap = mutableMapOf<IndexedValue<Char>, List<Graph.Edge<IndexedValue<Char>>>>()

    val getEdges: (IndexedValue<Char>) -> List<IndexedValue<Char>> = { node ->
        getNeighborsIndexedValue(node.index)
            .filter { it.value != '#' }
    }
    withIndex().filter { it.value !in ignore }.forEach { node ->
        edgeMap[node] = Graph.bfs(node, defaultEdges = getEdges)
            .filter { it.id.value !in ignore }
            .drop(1)
            .map { Graph.Edge(it.id, it.weight) }
    }
    return edgeMap
}

fun Grid<Char>.getEdgeMap(ignore: String = "#. "): Map<Char, List<Graph.Edge<Char>>> = getEdgeMapIndexed(ignore)
    .entries
    .associate { entry ->
        entry.key.value to entry.value.map { edge -> Graph.Edge(edge.vertexId.value, edge.weight) }
    }

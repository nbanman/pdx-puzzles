@file:Suppress("unused")

package org.gristle.pdxpuzzles.utilities.objects

private class KeyHolder<K: Comparable<K>, E>(val key: K, val element: E)

/**
 * A non-indexed heap. When the comparing values are non-mutable, use the superior java.util.PriorityQueue.
 * This version ensures that the heap does not fall apart if the underlying comparing values change.
 */
class Heap<K: Comparable<K>, E> private constructor(
    initialElements: Iterable<Pair<K, E>>,
    private val compare: (K, K) -> Boolean
) {
    private val elements = mutableListOf<KeyHolder<K, E>>()

    init {
        addAll(initialElements)
    }

    companion object {
        fun <K : Comparable<K>, E> minHeap(elements: Iterable<Pair<K, E>> = emptyList()) = Heap(elements) { a, b ->
            a < b
        }

        fun <K : Comparable<K>, E> maxHeap(elements: Iterable<Pair<K, E>> = emptyList()) = Heap(elements) { a, b ->
            a > b
        }

        private fun parent(i: Int) = ((i + 1) shr(1)) - 1
        private fun leftChild(i: Int) = ((i + 1) shl(1)) - 1
        private fun rightChild(i: Int) = leftChild(i) + 1
    }

    fun peek() = if (elements.isEmpty()) null else elements.first().element

    fun poll(): E? {
        val popped = peek() ?: return null
        elements[0] = elements.last()
        elements.removeLast()
        heapify(0)
        return popped
    }

    fun add(key: K, element: E) {
        elements.add(KeyHolder(key, element))
        var current = elements.lastIndex
        var parent = parent(current)
        while (parent != -1 && compare(elements[current].key, elements[parent].key)) {
            swap(parent, current)
            current = parent.also { parent = parent(parent) }
        }
    }

    fun addAll(newElements: Iterable<Pair<K, E>>) {
        for (newElement in newElements) {
            add(newElement.first, newElement.second)
        }
    }

    fun contains(element: E) = elements.any { it.element == element }

    private fun swap(a: Int, b: Int)  {
        elements[a] = elements[b].also { elements[b] = elements[a] }
    }

    private tailrec fun heapify(i: Int) {
        val leftChild = leftChild(i)
        val rightChild = rightChild(i)

        var ordered = if (leftChild <= elements.lastIndex) {
            if (compare(elements[i].key, elements[leftChild].key)) i else leftChild
        } else {
            i
        }

        if (rightChild <= elements.lastIndex && !compare(elements[ordered].key, elements[rightChild].key))
            ordered = rightChild

        if (ordered != i) {
            swap(ordered, i)
            heapify(ordered)
        }
    }

    fun isEmpty(): Boolean = elements.isEmpty()

    fun isNotEmpty(): Boolean = elements.isNotEmpty()

    tailrec fun pollUntil(predicate: (E) -> Boolean): E? {
        val poll = poll()
        return when {
            poll == null -> null
            predicate(poll) -> poll
            else -> pollUntil(predicate)
        }
    }

    fun dumpToList(): List<E> {
        val returnList = mutableListOf<E>()
        while (peek() != null) poll()?.let { returnList.add(it) }
        return returnList
    }
}

/**
 * An indexed heap that supports updating the value of specific elements at the expense of significant
 * decrease in performance and memory for maintaining the index. When the value only goes down, consider
 * using a standard PriorityQueue with redundant elements, using the pollUntil() extension function to skip
 * already visited elements.
 */
class IndexedHeap<E> private constructor(
    initialElements: Iterable<E>,
    private val compare: (E, E) -> Boolean
) {
    private val elements = mutableListOf<E>()
    private val references = mutableMapOf<E, Int>()

    init {
        addAll(initialElements)
    }

    companion object {
        fun <E : Comparable<E>> minHeap(elements: Iterable<E> = emptyList()) = IndexedHeap(elements) { a, b ->
            a < b
        }

        fun <E : Comparable<E>> maxHeap(elements: Iterable<E> = emptyList()) = IndexedHeap(elements) { a, b ->
            a > b
        }

        fun <E> minHeap(elements: Iterable<E>, comparator: Comparator<E>) = IndexedHeap(elements) { a, b ->
            comparator.compare(a, b) < 0
        }

        fun <E> maxHeap(elements: Iterable<E>, comparator: Comparator<E>) = IndexedHeap(elements) { a, b ->
            comparator.compare(a, b) > 0
        }

        fun <E> minHeap(comparator: Comparator<E>) = IndexedHeap<E>(emptyList()) { a, b ->
            comparator.compare(a, b) < 0
        }

        fun <E> maxHeap(comparator: Comparator<E>) = IndexedHeap<E>(emptyList()) { a, b ->
            comparator.compare(a, b) > 0
        }


        private fun parent(i: Int) = ((i + 1) shr(1)) - 1
        private fun leftChild(i: Int) = ((i + 1) shl(1)) - 1
        private fun rightChild(i: Int) = leftChild(i) + 1
    }

    fun contains(element: E) = references[element] != null

    fun peek(): E? = if (elements.isEmpty()) null else elements.first()

    fun poll(): E? {
        val popped = peek() ?: return null
        references.remove(elements[0])
        elements[0] = elements.last()
        elements.removeLast()
        if (elements.isNotEmpty()) {
            references[elements[0]] = 0
            heapifyDown(0)
        }
        return popped
    }

    fun add(element: E) {
        elements.add(element)
        references[element] = elements.lastIndex
        heapifyUp(elements.lastIndex)
    }

    private tailrec fun heapifyUp(i: Int) {
        val parent = parent(i)
        if (parent == -1 || compare(elements[parent], elements[i])) return
        swap(i, parent)
        heapifyUp(parent)
    }

    fun addAll(elements: Iterable<E>) {
        for (element in elements) {
            add(element)
        }
    }

    private fun swap(a: Int, b: Int)  {
        elements[a] = elements[b].also { elements[b] = elements[a] }
        references[elements[a]] = a
        references[elements[b]] = b
    }

    private tailrec fun heapifyDown(i: Int) {
        val leftChild = leftChild(i)
        val rightChild = rightChild(i)

        var ordered = if (leftChild <= elements.lastIndex) {
            if (compare(elements[i], elements[leftChild])) i else leftChild
        } else {
            i
        }

        if (rightChild <= elements.lastIndex && !compare(elements[ordered], elements[rightChild]))
            ordered = rightChild

        if (ordered != i) {
            swap(ordered, i)
            heapifyDown(ordered)
        }
    }

    fun updateElement(element: E, update: (E) -> E) {
        val index = references[element] ?: return
        val updatedElement = update(elements[index])
        references.remove(element)
        references[updatedElement] = index
        elements[index] = updatedElement
        if (compare(element, updatedElement)) {
            heapifyDown(index)
        } else {
            heapifyUp(index)
        }
    }

    fun isEmpty(): Boolean = elements.isEmpty()

    fun isNotEmpty(): Boolean = elements.isNotEmpty()

    fun toList(): List<E> {
        val returnList = mutableListOf<E>()
        val heapClone = IndexedHeap(elements.toList(), compare)
        while (heapClone.peek() != null) heapClone.poll()?.let { returnList.add(it) }
        return returnList
    }
}

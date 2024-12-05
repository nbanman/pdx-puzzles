package org.gristle.pdxpuzzles.utilities.objects

class IndexedLinkedList<E> (
    elements: List<E>,
    val circular: Boolean = false
) {

    class Node<E> (
        val value: E,
        val list: IndexedLinkedList<E>
    ) {
        var next: Node<E>? = null

        fun cut(n: Int): Node<E> {
            require(n > 0 && n < list.size)
            list.size -= n
            val first = next ?: if (list.circular) list.header else throw NoSuchElementException()
            var nexts: Node<E> = first
            for (i in 2 .. n) {
                nexts = nexts.next ?: if (!list.circular) {
                    next = nexts
                    break
                } else {
                    nexts.next = list.header
                    list.header = list.header.next!!
                    nexts.next!!
                }
                if (nexts == list.header) list.header = list.header.next!!
            }
            next = nexts.next
            nexts.next = null
            return first
        }

        fun add(node: Node<E>) {
            tailrec fun Node<E>.advanceToEnd(n: Int = 1): Pair<Node<E>, Int> {
                return if (next == null) this to n else next!!.advanceToEnd(n + 1)
            }
            val lastAdd = node.advanceToEnd()
            lastAdd.first.next = next
            next = node
            list.size += lastAdd.second
        }

        override fun toString(): String {
            return "$value, ${if (next != list.header) next.toString() else "end"}"
        }
    }

    var header: Node<E>
    var size = elements.size
    val index = mutableMapOf<E, Node<E>>()

    init {
        require(elements.firstOrNull() != null)
        header = Node(elements.first(), this)
        index[header.value] = header
        var last = header
        for (element in elements.drop(1)) {
            val next = Node(element, this)
            index[next.value] = next
            last.next = next
            last = next
        }
        if (circular) last.next = header
    }

    fun toList(): List<E> {
        val returnList = mutableListOf<E>()
        var start = header
        do {
            returnList.add(start.value)
            start = start.next ?: break
        } while (start != header)
        return returnList
    }

    override fun toString(): String {
        return "IndexedLinkedList(circular=$circular, size= $size, header=$header)"
    }

}

fun main() {
    val l1 = listOf(1, 2, 3, 4, 5)
    val t1 = IndexedLinkedList(l1, true)
    val cut = t1.index.getValue(3).cut(3)
    t1.index.getValue(2).add(cut)
    println(t1)
}
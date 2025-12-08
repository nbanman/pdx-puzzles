package org.gristle.pdxpuzzles.utilities.objects

class UnionFind(val parent: MutableList<Int>, val size: MutableList<Int>) {
    companion object {
        fun new(n: Int) = UnionFind(MutableList(n) { it }, MutableList(n) { 1 })
    }

    fun find(x: Int): Int {
        if (parent[x] != x) {
            parent[x] = find(parent[x])
        }
        return parent[x]
    }

    fun union(x: Int, y: Int): Boolean {
        val x = find(x)
        val y = find(y)

        if (x == y) {
            return false
        }
        if (size[x] >= size[y]) {
            parent[y] = x
            size[x] += size[y]
        } else {
            parent[x] = y
            size[y] += size[x]
        }
        return true
    }

    fun update() {
        for (i in parent.indices) {
            find(i)
        }
    }
}
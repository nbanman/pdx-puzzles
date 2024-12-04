package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day

class Y23D01(input: String) : Day {

    private val lines = input.lines()

    // searches the line forwards and backwards looking for the first Char that is a digit, combines and 
    // sums them.
    override fun part1() = lines.sumOf { "${it.first(Char::isDigit)}${it.last(Char::isDigit)}".toInt() }

    // For fun, part2 pursues a baroque strategy. Two tries are created, one for searching forwards, one backwards.
    // Each node is assigned a value and a map containing potential children, eventually spelling out the names of 
    // digits. So 'o' -> 'n' -> 'e' and 'f' -> ['o' -> 'u' -> 'r'] | ['i' -> 'v' -> 'e']. The fun 'getDigit' takes
    // a line and goes through each character, trying to traverse a trie to a leaf. If at any time a numerical digit
    // is encountered, the function immediately returns the value of the digit. Otherwise, it goes to the end. Any
    // time a word does not match it is immediately discarded.
    override fun part2(): Int {

        // One trie contains information for searching forwards, the other for searching backwards. 
        // E.g., "one" becomes "eno"
        val forwardTrie = Node(0)
        val reverseTrie = Node(0)

        val numbers = "one|two|three|four|five|six|seven|eight|nine".splitToSequence('|')
        numbers
            .forEachIndexed { index, s ->
                // get value of the number from the ordinal order of the list
                val value = index + 1

                // populate the tries with nodes relating to the digit
                forwardTrie.populateTrie(s, value)
                reverseTrie.populateTrie(s.reversed(), value)
            }

        return lines.sumOf { line ->
            val firstDigit = getDigit(line, forwardTrie) // search forward
            val secondDigit = getDigit(line.reversed(), reverseTrie) // search backward
            firstDigit * 10 + secondDigit
        }
    }

    class Node(value: Int, val children: MutableMap<Char, Node> = mutableMapOf()) {
        private val _value: Int = value
        val value: Int get() = if (children.isEmpty()) _value else 0
    }

    // See pt2 description for how this works.
    private fun getDigit(line: String, trie: Node): Int {

        // tracks snippets of sequences that may end up being spelled out digits
        var potentials: List<Node> = emptyList()

        // traverses string, forwards or backwards
        for (token in line) {
            // returns early if numerical digit found
            if (token.isDigit()) return token.digitToInt()  

            // Update each value in potentials. If a word is spelled out, return the value of that word.
            // If the token is not in a word, then remove it from the list of potentials. Add the start of
            // a new word to the end of the potentials list.
            potentials = buildList { 
                for (node in potentials) {
                    val child = node.children[token] // gets the next node, or null if such node doesn't exist
                    if (child != null) {
                        // if that is a final node spelling a word, return the value. Otherwise, add that node to
                        // list of potentials.
                        if (child.value != 0) return child.value 
                        add(child)
                    }
                }
                
                // starts a new search starting at the current token if that's the start of a word. 
                trie.children[token]?.let { add(it) }
            }
           
        }

        // if the string does not contain a numerical or spelled-out digit, the string is malformed so throw 
        throw IllegalArgumentException("No digit found in $line")
    }

    // Populates the trie with nodes corresponding to the string.
    private fun Node.populateTrie(s: String, value: Int) {
        s.fold(this) { previous, c -> previous.children.getOrPut(c) { Node(value) } }
    }
}

fun main() = Day.runDay(Y23D01::class)

//    Class creation: 15ms
//    Part 1: 54388 (2ms)
//    Part 2: 53515 (9ms)
//    Total time: 27ms
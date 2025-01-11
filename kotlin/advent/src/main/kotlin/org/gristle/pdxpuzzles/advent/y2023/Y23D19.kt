package org.gristle.pdxpuzzles.advent.y2023

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.parsing.blankSplit
import org.gristle.pdxpuzzles.utilities.parsing.getInts

class Y23D19(input: String) : Day {
    
    // A single workflow holds several rules in sequence. 
    // 'Category' is an int representation of the various XMAS categories
    // 'Amount' is used to compare whether there are the correct amount of prats.
    // 'Comparison' can be ">", "<", or "". Blank means automatically pass.
    // 'Destination' is the name of the next workflow to send to if the number/range passes.
    data class Rule(val category: Int, val amount: Int, val comparison: String, val destination: String) 
    
    // Workflows accessible by workflow name.
    private val workflows: Map<String, List<Rule>>
    
    // Parts used for Part 1.
    private val parts: List<List<Int>>
    
    // Parsing 
    init {
        val (workStanza, partStanza) = input.blankSplit()
        workflows = workStanza
            .lines().associate { line ->
                val split = line.split('{', ',')
                val name = split.first()
                val rules = split.drop(1)
                    .map { ruleStr -> // (categoryStr, comparison, amountStr, destination) ->
                        if (ruleStr.last() == '}') {
                            Rule(0, 0, "", ruleStr.dropLast(1))
                        } else {
                            val categoryStr = ruleStr[0]
                            val category = when (categoryStr) {
                                'x' -> 0
                                'm' -> 1
                                'a' -> 2
                                's' -> 3
                                else -> throw IllegalArgumentException("??")
                            }
                            val comparison = ruleStr[1].toString()
                            val amount = ruleStr.dropWhile { !it.isDigit() }.takeWhile { it.isDigit() }.toInt()
                            val destination = ruleStr.takeLastWhile { it.isLetter() }
                            Rule(category, amount, comparison, destination)
                        }
                    }.toList()
                name to rules
            }
        parts = partStanza.getInts()
            .chunked(4)
            .toList()
    }
    
    // For each part, run the recursive sort function on it, which sends it through the various workflows until it 
    // ends up being accepted "A" or rejected "R." Sum the XMAS values for each accepted part and sum the total.
    override fun part1() = parts
        .filter { xmas ->
            sort("in", xmas) == "A"
        }.sumOf { it.sum() }

    private fun sort(name: String, part: List<Int>): String {
        
        // get the relevant workflow
        val workflow = workflows.getValue(name)
        for (rule in workflow) {
            with (rule) {
                
                // perform comparison and route accordingly
                val resultOrNull = if (comparison == ">") {
                    if (part[category] > amount) destination else null
                } else if (comparison == "<") {
                    if (part[category] < amount) destination else null
                } else {
                    destination
                }
                
                // if routed to "A" or "R", we are done. Otherwise call sort again with the next workflow.
                if (resultOrNull == "A" || resultOrNull == "R") {
                    return resultOrNull
                } else if (resultOrNull != null) {
                    return sort(resultOrNull, part)
                }
            }
        }
        return ""
    }
    
    // Start with a PartRanges instance, a list of 4 IntRanges from 1..4000, representing each of X, M, A, and S.
    // Run the instance through the first workflow. Create two PartRanges instances, splitting the ranges for each 
    // branching. Then do the same thing to those instances, until the subranges reach "R" or "A". Keep track of the
    // ones that are accepted, and sum the number of possible permutations 
    override fun part2(): Long {
        val accepted = mutableListOf<PartRanges>()
        var remaining: List<Pair<String, PartRanges>> = listOf("in" to PartRanges(List(4) { 1..4000 }))
        while (remaining.isNotEmpty()) {
            val next = remaining.flatMap { (name, partRanges) ->
                workflows.getValue(name).route(partRanges)
            }
            remaining = next.filter { (name, partRanges) ->
                when (name) {
                    "R" -> false
                    "A" -> {
                        accepted.add(partRanges)
                        false
                    }
                    else -> true
                }
            }
        }
        return accepted.sumOf { it.permutations() }
    }

    // Tracks the ranges for each xmas category.
    @JvmInline
    value class PartRanges(val ranges: List<IntRange>) {
        
        // Number of possible permutations given the four ranges.
        fun permutations(): Long = ranges.fold(1L) { acc, range -> acc * (1 + range.last - range.first) }

        // Splits the PartRanges into two, one that meets the rule condition, and one that doesn't.
        fun split(rule: Rule): List<PartRanges> {
            return when (rule.comparison) {
                ">" -> {
                    val breakpoint = rule.amount + 1
                    val pass = breakpoint..ranges[rule.category].last
                    val fail = ranges[rule.category].first until breakpoint
                    makeSplits(rule.category, pass, fail)
                }
                "<" -> {
                    val breakpoint = rule.amount
                    val pass = ranges[rule.category].first until breakpoint
                    val fail = breakpoint..ranges[rule.category].last
                    makeSplits(rule.category, pass, fail)
                }
                else -> throw IllegalArgumentException("Non-comparisons should not be passed to split function.")
            }
        }
        
        private fun makeSplits(category: Int, pass: IntRange, fail: IntRange): List<PartRanges> =
            listOf(
                PartRanges(List(4) { i -> if (i == category) pass else ranges[i] }),
                PartRanges(List(4) { i -> if (i == category) fail else ranges[i] }),
            )
    }

    // for a particular workflow, return sub-PartRanges along with where they should be routed to next
    private fun List<Rule>.route(partRanges: PartRanges): List<Pair<String, PartRanges>> = buildList {
        var remaining: PartRanges? = partRanges
        for (rule in this@route) {
            if (remaining == null) return@buildList
            remaining = if (rule.comparison.isNotEmpty()) {
                val (pass, fail) = remaining.split(rule)
                add(rule.destination to pass)
                fail
            } else { // every rule conditional failed and base case reached
                add(rule.destination to remaining)
                null
            }
        }
    }
}

fun main() = Day.runDay(Y23D19::class)

//    Class creation: 47ms
//    Part 1: 449531 (4ms)
//    Part 2: 122756210763577 (10ms)
//    Total time: 63ms


// Part 1: 19114	Part 2: 167409079868000
@Suppress("unused")
private val sampleInput = listOf(
    """px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
""",
)
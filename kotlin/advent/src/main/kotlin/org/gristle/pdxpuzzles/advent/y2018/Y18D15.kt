package org.gristle.pdxpuzzles.advent.y2018

import org.gristle.pdxpuzzles.advent.utilities.Day
import org.gristle.pdxpuzzles.utilities.objects.Coord
import org.gristle.pdxpuzzles.utilities.graph.Graph

class Y18D15(val input: String) : Day {

    /**
     * Abstract class for both elves and goblins. Responsible for tracking health, playing each players turn,
     * including attacking and moving
     */
    abstract class Player {
        var health = 200

        private val isDead: Boolean get() = health < 1

        /**
         * Finds the player at the specified other position and commands it to take damage.
         */
        private fun attack(world: World, otherPos: Coord) {
            world[otherPos]?.takeDamage(world, otherPos, damage(world))
        }

        /**
         * Looks for adjacent targets. If found, attack the one with lowest health. Otherwise, move to closest
         * target and attack if now adjacent.
         */
        fun playTurn(world: World, pos: Coord) {
            if (isDead) return

            val enemies = enemies(world)

            // utility function for finding adjacent targets to attack
            fun adjacentOpponentPos(pos: Coord) = pos
                .getNeighbors()
                .filter { it in enemies }
                .minByOrNull { enemies.getValue(it).health }

            // find adjacent target...
            val adjacentTarget = adjacentOpponentPos(pos)

            if (adjacentTarget != null) {
                attack(world, adjacentTarget) // ...and attack it
            } else { // if no adjacent target...
                val newPos = move(world, pos) // move one space to a target
                val adjacentTargetAfterMove = adjacentOpponentPos(newPos) // and attack if adjacent
                if (adjacentTargetAfterMove != null) attack(world, adjacentTargetAfterMove)
            }
        }

        /**
         * Takes damage and removes the player from the world if dead.
         */
        private fun takeDamage(world: World, pos: Coord, damage: Int) {
            health -= damage
            if (isDead) {
                friends(world).remove(pos)
            }
        }

        /**
         * Finds the closest spot adjacent to a target and move one step toward it. Uses BFS.
         */
        fun move(world: World, pos: Coord): Coord {
            val friends = friends(world)
            val enemies = enemies(world)

            // The coordinates that are adjacent to targets.
            val inRange: Set<Coord> = enemies
                .keys
                .flatMap { opponentPos -> opponentPos.getNeighbors().filter { world.canMove(it) } }
                .toSet()

            // Function to find neighboring coordinates for any given position
            val edges: (Coord) -> List<Coord> = { it.getNeighbors().filter { neighbor -> world.canMove(neighbor) } }

            // BFS takes a starting coordinate and spreads out to all legal moving positions, stopping when it
            // finds a target in range. It then reconstructs the path it took, finds the first step in that path,
            // and that is the new position.
            val newPos = Graph.bfsSequence(pos, defaultEdges = edges)
                .firstOrNull { (pos) -> pos in inRange }
                ?.path()
                ?.get(1)
                ?.id

            return if (newPos == null) {
                pos
            } else {
                // update the friends map and return the new position
                friends.remove(pos)
                friends[newPos] = this
                newPos
            }
        }

        abstract fun damage(world: World): Int

        abstract fun friends(world: World): MutableMap<Coord, Player>

        abstract fun enemies(world: World): MutableMap<Coord, Player>

    }

    class Elf : Player() {
        override fun friends(world: World) = world.elves
        override fun enemies(world: World) = world.goblins
        override fun damage(world: World) = world.elfDamage
    }

    class Goblin : Player() {
        override fun friends(world: World) = world.goblins
        override fun enemies(world: World) = world.elves
        override fun damage(world: World) = 3
    }

    // State object for the "World;" ie, where each Elf, Goblin, and wall is, as well as the dimensions of the map.
    // It also stores how much damage elves do in this world.
    data class World(
        val width: Int,
        val height: Int,
        private val walls: Set<Coord>,
        val elves: MutableMap<Coord, Player>,
        val goblins: MutableMap<Coord, Player>,
        val elfDamage: Int
    ) {
        private val initialElves = elves.size

        /**
         * Creates a copy of the world with rejuvenated elves and goblins, and with a new elfDamage amount.
         */
        fun clone(elfDamage: Int): World {
            val newElves: MutableMap<Coord, Player> = elves
                .entries
                .associateTo(mutableMapOf()) { (pos) -> pos to Elf() }
            val newGoblins: MutableMap<Coord, Player> = goblins
                .entries
                .associateTo(mutableMapOf()) { (pos) -> pos to Goblin() }

            return World(width, height, walls, newElves, newGoblins, elfDamage)
        }

        /**
         * Checks whether a coordinate would be a legal move by checking if it's within the dimensions, and whether
         * there is a wall or player in that position.
         */
        fun canMove(pos: Coord): Boolean = pos.x in 0 until width
                && pos.y in 0 until height
                && pos !in walls
                && pos !in elves
                && pos !in goblins

        fun elfHealth() = elves.values.sumOf { it.health }
        fun goblinHealth() = goblins.values.sumOf { it.health }

        /**
         * Returns all players in reading order.
         */
        fun players() = (elves.entries + goblins.entries).sortedBy { (pos) -> pos.asIndex(width) }

        /**
         * Returns true if elves have lost. Part 2 logic makes elves lose if even one elf dies.
         */
        fun elvesLose(): Boolean = elves.isEmpty() || (elfDamage > 3 && elves.size < initialElves)

        /**
         * Returns player at a position.
         */
        operator fun get(pos: Coord): Player? = elves[pos] ?: goblins[pos]

        companion object {

            /**
             * Parses initial world from input string.
             */
            fun from(input: String): World {
                val width = input.indexOf('\n')

                // The string with no newlines. 
                val flattenedInput = input.replace("\n", "")
                val height = flattenedInput.length / width
                val elves = mutableMapOf<Coord, Player>()
                val goblins = mutableMapOf<Coord, Player>()
                val walls = mutableSetOf<Coord>()
                for (y in 0 until height) {
                    for (x in 0 until width) {
                        val pos = Coord(x, y)
                        when (flattenedInput[y * width + x]) {
                            '#' -> walls.add(pos)
                            'E' -> elves[pos] = Elf()
                            'G' -> goblins[pos] = Goblin()
                        }
                    }
                }
                return World(width, height, walls, elves, goblins, 3)
            }
        }
    }

    // The initial world used by both parts.
    private val initialWorld = World.from(input)

    /**
     * State class holding the final Round info plus opponentHp, used to find the final score.
     */
    data class Game(val round: Round, val opponentHp: Int) {
        fun score(): Int = round.round * opponentHp
    }

    /**
     * Enum class says whether the game continues, or elves or goblins won.
     */
    enum class WinState { ELVES, GOBLINS, CONTINUE }

    /**
     * Round state class holding the round number and the current winstate.
     */
    data class Round(val round: Int = 0, val winState: WinState = WinState.CONTINUE)

    /**
     * Main game loop. Returns a game object that has enough info to determine whether the game needs to be run
     * again (in part 2) and to calculate the score.
     */
    fun solve(elfDamage: Int = 3): Game {

        // Make a working copy of the initial world.
        val world = initialWorld.clone(elfDamage)

        // Takes a Round state, and plays another Round
        fun playRound(round: Round): Round {

            // get all remaining players in reading order
            val players = world.players()

            // for each player...
            players.forEachIndexed { index, (pos, player) ->

                // ...play a turn
                player.playTurn(world, pos)

                // determine if there is a winner
                val winState = when {
                    world.elvesLose() -> WinState.GOBLINS
                    world.goblins.isEmpty() -> WinState.ELVES
                    else -> WinState.CONTINUE
                }

                // if there is a winner, note that in the return Round. Only advance the round if the last player
                // won the game
                if (winState != WinState.CONTINUE) {
                    val winRound = if (index == players.lastIndex) round.round + 1 else round.round
                    return Round(winRound, winState)
                }
            }

            // otherwise return a continuing round
            return round.copy(round = round.round + 1)
        }

        // sequence runs rounds in succession until a winner is found. When found, returns the game state.
        generateSequence(Round(), ::playRound)
            .first { (_, winState) -> winState != WinState.CONTINUE }
            .let { round ->
                val opponentHp = if (round.winState == WinState.ELVES) world.elfHealth() else world.goblinHealth()
                return Game(round, opponentHp)
            }
    }

    // Runs the game once and returns the score.
    override fun part1() = solve().score()

    // Sequence runs games with increasing elf damage until there is a game where the elves win. Returns score.
    override fun part2() = generateSequence(4) { elfDamage -> elfDamage + 1 }
        .map { elfDamage -> solve(elfDamage) }
        .first { game -> game.round.winState == WinState.ELVES }
        .score()
}

fun main() = Day.runDay(Y18D15::class)

//    Class creation: 5ms
//    Part 1: 224370 (140ms)
//    Part 2: 45539 (252ms)
//    Total time: 399ms
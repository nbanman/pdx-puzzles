use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::{graphs::{bfs, EdgeInfo, PathInfo}, structs::{coord::Coord2U, stopwatch::{ReportDuration, Stopwatch}, str_grid::StrGrid}};
use utilities::enums::cardinals::Cardinal;

const READING_ORDER: [Cardinal; 4] = [
    Cardinal::North,
    Cardinal::West,
    Cardinal::East,
    Cardinal::South,
];

type Input = World;
type Output = usize;
type Pos = Coord2U;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Race { Elf, Goblin }

#[derive(Clone, Copy, Debug)]
struct Player {
    race: Race,
    health: i16,
}

impl Player {
    fn new(race: Race) -> Self {
        Self { race, health: 200 }
    }
    
    fn is_dead(&self) -> bool {
        self.health <= 0
    }

    fn enemy_race(&self) -> Race {
        match self.race {
            Race::Elf => Race::Goblin,
            Race::Goblin => Race::Elf,
        }
    }

    fn attack(&self, world: &mut World, opponent_pos: Pos, killed: &mut FxHashSet<Pos>) {
        let opponent: &mut Player = world.players.get_mut(&opponent_pos).unwrap();
        let damage = match self.race {
            Race::Elf => world.elf_damage,
            Race::Goblin => 3,
        };
        opponent.health -= damage;
        if opponent.is_dead() {
            world.players.remove(&opponent_pos);
            killed.insert(opponent_pos);
        }
    }

    fn move_pos(&self, world: &mut World, pos: Pos) -> Option<Pos> {
        // The coordinates that are adjacent to targets.
        let opponent_race = match self.race {
            Race::Elf => Race::Goblin,
            Race::Goblin => Race::Elf,
        };
        let in_range: FxHashSet<Pos> = world.player_positions_by_race(opponent_race)
            .flat_map(|pos| {
                pos.adjacent(false).into_iter().filter(|&n| world.can_move(n))
            })
            .collect();
        let search: PathInfo<Pos, usize> = bfs(
            pos,
            |_: EdgeInfo<usize>, pos| {
                READING_ORDER.iter()
                    .map(|&dir| pos.move_direction(dir, 1).unwrap())
                    .filter(|&n| world.can_move(n))
                    .collect()
            },
            |_: EdgeInfo<_>, pos| in_range.contains(pos)
        );
        search.end_index.map(|end_index| search.path(end_index)[1].state)
    }

    fn play_turn(&self, world: &mut World, pos: Pos, killed: &mut FxHashSet<Pos>) -> Option<Pos> {
        if self.is_dead() { return None; }
        let enemies: Vec<Pos> = world.player_positions_by_race(self.enemy_race()).collect();

        if let Some(adjacent_target) = Self::adjacent_opponent_pos(pos, &enemies, world) {
            self.attack(world, adjacent_target, killed);
            None
        } else {
            let new_pos = self.move_pos(world, pos);
            if let Some(new_pos) = new_pos {
                if let Some(adjacent_target) = Self::adjacent_opponent_pos(new_pos, &enemies, world) {
                    self.attack(world, adjacent_target, killed);
                }
            }
            new_pos
        }
    }

    fn adjacent_opponent_pos(pos: Pos, enemies: &[Pos], world: &World) -> Option<Pos> {
        READING_ORDER.iter()
            .map(|&dir| pos.move_direction(dir, 1).unwrap())
            .filter(|it| enemies.contains(it))
            .min_by_key(|it| world.players.get(it).map(|p| p.health))
    }
}

#[derive(Clone, Debug)]
struct World {
    elf_damage: i16,
    initial_elves: usize,
    walls: FxHashSet<Pos>,
    players: FxHashMap<Pos, Player>,
    width: usize,
    height: usize,
}

impl World {
    fn player_positions_by_race(&self, race: Race) -> impl Iterator<Item = Pos> {
        self.players.iter()
            .filter(move |(_, player)| player.race == race)
            .map(|(&pos, _)| pos)
    }

    fn can_move(&self, pos: Pos) -> bool {
        (0..self.height).contains(&pos.x())
            && (0..self.width).contains(&pos.y())
            && !self.walls.contains(&pos)
            && !self.players.contains_key(&pos)
    }

    // Returns all players in reading order.
    fn positions_in_order(&self) -> impl Iterator<Item = Pos> {
        self.players.keys()
            .sorted_unstable_by_key(|&pos| pos)
            .copied()
    }

    fn elves_lose(&self) -> bool {
        let elves = self.player_positions_by_race(Race::Elf).count();
        elves == 0 || (self.elf_damage > 3 && elves < self.initial_elves)
    }
}

impl From<&str> for World {
    fn from(value: &str) -> Self {
        let grid = StrGrid::new(value).unwrap();
        let width = grid.width - 1;
        let height = grid.height;
        let mut walls = FxHashSet::default();
        let mut players = FxHashMap::default();
        for (index, &b) in grid.s.iter().enumerate() {
            let x = index % grid.width;
            let y = index / grid.width;
            let pos = Pos::new2d(x, y);
            match b {
                b'#' => { walls.insert(pos); },
                b'G' => { players.insert(pos, Player::new(Race::Goblin)); },
                b'E' => { players.insert(pos, Player::new(Race::Elf)); },
                _ => {},
            }
        }

        let initial_elves = players.values()
            .filter(|p| p.race == Race::Elf)
            .count();

        Self {
            elf_damage: 3,
            initial_elves,
            walls,
            players,
            width,
            height,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Game {
    round: Round,
    remaining_health: i16,
}

impl Game {
    fn score(&self) -> usize {
        self.round.round_number * self.remaining_health as usize
    }
}

#[derive(Debug, Copy, Clone)]
struct Round {
    round_number: usize,
    win_state: WinState,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum WinState { Elves, Goblins, Continue }

fn play_game(mut world: World) -> Game {
    let mut round = Round { round_number: 0, win_state: WinState::Continue };
    while round.win_state == WinState::Continue {
        round = play_round(&mut world, round);
    }   
    let winner = match round.win_state {
        WinState::Elves => Race::Elf,
        WinState::Goblins => Race::Goblin,
        WinState::Continue => unreachable!(),
    };
    let remaining_health: i16 = world.players.values()
        .filter(|player| player.race == winner)
        .map(|player| player.health)
        .sum();

    Game { round, remaining_health }
}

#[allow(dead_code)]
fn rep(world: &World) -> String {
    let mut rep = String::new();
    for y in 0..world.height {
        for x in 0..world.width {
            let pos = Pos::new2d(x, y);
            if world.walls.contains(&pos) {
                rep.push('#');
            } else if !world.players.contains_key(&pos) {
                rep.push('.');
            } else {
                if let Some(player) = world.players.get(&pos) {
                    if player.race == Race::Elf {
                        rep.push('E');
                    } else {
                        rep.push('G');
                    }
                }
            }
        }
        rep.push('\n');
    }
    rep
}

fn play_round(world: &mut World, round: Round) -> Round {
    let mut killed = FxHashSet::default();
    for (index, pos) in world.positions_in_order().collect_vec()
        .into_iter()
        .enumerate()
    {
        if killed.contains(&pos) { continue; }
        let Some(player) = world.players.remove(&pos) else { continue; };
        let new_pos = player.play_turn(world, pos, &mut killed)
            .unwrap_or(pos);
        world.players.insert(new_pos, player);
        let win_state = if world.elves_lose() {
            WinState::Goblins
        } else if world.player_positions_by_race(Race::Goblin).count() == 0 {
            WinState::Elves
        } else {
            WinState::Continue
        };
        if win_state != WinState::Continue {
            let win_round = if index == world.players.len() - 1 {
                round.round_number + 1
            } else {
                round.round_number
            };
            return Round { round_number: win_round, win_state }
        }
    }
    Round { round_number: round.round_number + 1, win_state: WinState::Continue }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 15).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.into()
}

fn part1(world: Input) -> Output {
    play_game(world).score()
}

fn part2(initial_world: Input) -> Output {
    let mut min: i16 = 3;
    let mut max: i16 = 3;
    let mut score: Option<usize> = None;
    loop {
        let elf_damage = if score == None {
            max += 40;
            max
        } else {
            (max - min) / 2 + min
        };
        let mut world = initial_world.clone();
        world.elf_damage = elf_damage;
        let game = play_game(world);
        if game.round.win_state == WinState::Elves {
            if score.is_some() {
                max = elf_damage;
            }
            score = Some(game.score());
        } else {
            if score == None {
                min = 44;
                max += 40;
            } else {
                min = elf_damage + 1;
            }
        }
        if min + 1 == max {
            return score.unwrap();
        }
    }
}

#[test]
fn default() {
    let input = get_input(18, 15).unwrap();
    let input = parse_input(&input);
    assert_eq!(224370, part1(input.clone()));
    assert_eq!(45539, part2(input));
}

// Input parsed (56μs)
// 1. 224370 (27ms)
// 2. 45539 (16ms)
// Total: 44ms
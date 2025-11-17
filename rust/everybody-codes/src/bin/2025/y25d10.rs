use std::iter::successors;
use std::sync::OnceLock;

use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::structs::coord::{Coord2, Coord2U};
use utilities::structs::grid::Grid2;
use utilities::structs::grid::grid_display::GridDisplay;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Pos = Coord2U;
type Board = Grid2<Space>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Sheep,
    Hideout,
    SheepHide,
    Empty,
}

impl GridDisplay for Space {
    fn rep(&self) -> char {
        match self {
            Space::Sheep => 'S',
            Space::Hideout => '#',
            Space::SheepHide => '$',
            Space::Empty => '.',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Chess {
    dragon: Pos,
    board: Board,
}

impl Chess {
    fn moves(&self, dragon: Pos) -> impl Iterator<Item = Pos> {
        static MOVE_TEMPLATE: OnceLock<[Coord2; 8]> = OnceLock::new();
        let dragon = Coord2::from((dragon.x() as i64, dragon.y() as i64));
        MOVE_TEMPLATE
            .get_or_init(|| {
                [
                    Coord2::from((-2, -1)),
                    Coord2::from((-1, -2)),
                    Coord2::from((1, -2)),
                    Coord2::from((2, -1)),
                    Coord2::from((2, 1)),
                    Coord2::from((1, 2)),
                    Coord2::from((-1, 2)),
                    Coord2::from((-2, 1)),
                ]
            })
            .into_iter()
            .filter_map(move |&move_pos| {
                let dragon_move = dragon + move_pos;
                if dragon_move.x() < 0 || dragon_move.y() < 0 {
                    None
                } else {
                    let dragon_move =
                        Pos::from((dragon_move.x() as usize, dragon_move.y() as usize));
                    Some(dragon_move)
                }
            })
    }

    fn count_sheep(&self, moves_allowed: usize) -> usize {
        let mut sheep_eaten = 0;
        let mut moves = 0;
        let mut todo = vec![self.dragon];
        let mut next = Vec::new();
        let mut visited: FxHashSet<Pos> = todo.iter().copied().collect();

        while !todo.is_empty() {
            for cur in todo.drain(..) {
                if self.board.get2(cur).unwrap() == &Space::Sheep {
                    sheep_eaten += 1;
                }
                if moves < moves_allowed {
                    for dragon_move in self.moves(cur) {
                        if visited.insert(dragon_move) {
                            next.push(dragon_move);
                        }
                    }
                }
            }
            moves += 1;
            std::mem::swap(&mut todo, &mut next);
        }
        sheep_eaten
    }

    fn count_moving_sheep(&mut self, moves_allowed: usize) -> usize {
        let mut sheep_eaten = 0;
        let mut moves = 0;
        let mut todo = vec![self.dragon];
        let mut next = Vec::new();
        let mut visited: FxHashSet<Pos> = FxHashSet::default();

        while !todo.is_empty() {
            visited.clear();
            for &cur in todo.iter() {
                if *self.board.get2(cur).unwrap() == Space::Sheep {
                    sheep_eaten += 1;
                    self.board[cur] = Space::Empty;
                }
                if moves < moves_allowed {
                    for dragon_move in self.moves(cur) {
                        if visited.insert(dragon_move) {
                            next.push(dragon_move);
                        }
                    }
                }
            }
            if moves != 0 {
                self.move_sheep(&todo, &mut sheep_eaten);
            }
            moves += 1;
            todo.clear();
            std::mem::swap(&mut todo, &mut next);
        }
        sheep_eaten
    }

    fn move_sheep(&mut self, dragon_reach: &[Pos], sheep_eaten: &mut usize) {
        let dragons = dragon_reach
            .iter()
            .map(|dragon| dragon.x() + dragon.y() * self.board.width())
            .collect_vec();
        // move sheep in bottom row to safety
        let bottom_row = self.board.width() * (self.board.height() - 1);
        for x in 0..self.board.width() {
            self.board[x + bottom_row] = match self.board[x + bottom_row] {
                Space::Sheep => Space::Empty,
                Space::Empty => Space::Empty,
                Space::Hideout => Space::Hideout,
                Space::SheepHide => Space::Hideout,
            }
        }
        // move remaining sheep down one
        let height = self.board.height();
        let width = self.board.width();
        for row in (0..height - 1).rev().map(|y| y * width) {
            for col in 0..width {
                let cur = self.board[row + col];
                self.board[row + col] = match cur {
                    Space::Sheep => Space::Empty,
                    Space::Empty => Space::Empty,
                    Space::Hideout => Space::Hideout,
                    Space::SheepHide => Space::Hideout,
                };
                if cur == Space::Sheep || cur == Space::SheepHide {
                    let below_idx = row + col + width;
                    let below = self.board.get_mut(below_idx).unwrap();
                    *below = match below {
                        Space::Sheep => {
                            panic!("All sheep should have been cleared out of row below!")
                        }
                        Space::Empty => {
                            if dragons.contains(&below_idx) {
                                *sheep_eaten += 1;
                                Space::Empty
                            } else {
                                Space::Sheep
                            }
                        }
                        Space::Hideout => Space::SheepHide,
                        Space::SheepHide => {
                            panic!("All sheep should have been cleared out of row below!")
                        }
                    }
                }
            }
        }
    }
}

impl From<&str> for Chess {
    fn from(value: &str) -> Self {
        let board = Grid2::new2d_map_str(value, |c| match c {
            'S' => Space::Sheep,
            '#' => Space::Hideout,
            '.' | 'D' => Space::Empty,
            c => panic!("{c} is not a valid character!"),
        })
        .unwrap();
        let dragon = value.as_bytes().iter().position(|&b| b == b'D').unwrap();
        let dragon = Pos::new2d(dragon % (board.width() + 1), dragon / (board.width() + 1));
        Self {
            dragon,
            board,
        }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 10);
    println!("Input parsed ({})", stopwatch.lap().report());
    // println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    // println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> usize {
    let chess = Chess::from(input);
    chess.count_sheep(4)
}

fn part2(input: Input) -> usize {
    let mut chess = Chess::from(input);
    chess.count_moving_sheep(20)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Turn {
    Dragon,
    Sheep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    dragon: usize,
    sheep: u64,
    turn: Turn,
}

fn parse(input: Input) -> (Grid2<bool>, State) {
    let board = Grid2::new2d_map_str(input, |c| c == '#').unwrap();
    let dragon = input.as_bytes().iter().position(|&b| b == b'D').unwrap();
    let dragon = dragon % (board.width() + 1) + dragon / (board.width() + 1) * board.width();
    let sheep = input.as_bytes().iter()
        .fold(0u64, |acc, &b| {
            match b {
                b'S' => acc << 1 | 1,
                b'\n' => acc,
                _ => acc << 1,
            }
        });
    let state = State { dragon, sheep, turn: Turn::Sheep };
    (board, state)   
}

fn iter_sheep(sheep: u64) -> impl Iterator<Item = usize> {
    successors(Some(sheep), |&n| Some(n >> 1))
        .take_while(|&n| n != 0)
        .enumerate()
        .filter_map(move |(idx, n)| {
            if n & 1 == 1 {
                Some(idx)
            } else {
                None
            }
        })
}

fn dragon_moves(dragon: usize, board: &Grid2<bool>) -> impl Iterator<Item = usize> {
    static MOVE_TEMPLATE: OnceLock<[Coord2; 8]> = OnceLock::new();
    let dragon = Coord2::from(
        (
            (dragon % board.width()) as i64,
            (dragon / board.width()) as i64
        )
    );
    MOVE_TEMPLATE
        .get_or_init(|| {
            [
                Coord2::from((-2, -1)),
                Coord2::from((-1, -2)),
                Coord2::from((1, -2)),
                Coord2::from((2, -1)),
                Coord2::from((2, 1)),
                Coord2::from((1, 2)),
                Coord2::from((-1, 2)),
                Coord2::from((-2, 1)),
            ]
        })
        .into_iter()
        .filter_map(move |&move_pos| {
            let dragon_move = dragon + move_pos;
            if dragon_move.x() < 0
                || dragon_move.y() < 0
                || dragon_move.x() >= board.width() as i64
                || dragon_move.y() >= board.height() as i64
            {
                None
            } else {
                let dragon_move =
                    dragon_move.x() as usize + dragon_move.y() as usize * board.width();
                Some(dragon_move)
            }
        })
}

fn part3(input: Input) -> usize {
    let mut chess_boards = [
        r"SSS
..#
#.#
#D.",
        r"SSS
..#
..#
.##
.D#",
        r"..S..
.....
..#..
.....
..D..",
        r".SS.S
#...#
...#.
##..#
.####
##D.#",
        r"SSS.S
.....
#.#.#
.#.#.
#.D.#",
    ];
    let (board, initial_state) = parse(chess_boards[0]);
    let mut todo = vec![initial_state];
    let mut visited: FxHashMap<State, String> = FxHashMap::default();
    let mut variants = 0;

    while !todo.is_empty() {
        variants += todo.iter().filter(|it| it.sheep == 0).count();
        todo = todo
            .into_iter()
            .filter(|it| it.sheep != 0)
            .flat_map(|State { dragon, sheep, turn }| {
                let mut next = Vec::new();
                match turn {
                    Turn::Dragon => {

                        let test_dragon_moves = dragon_moves(dragon, &board).collect_vec();
                        for dragon_move in dragon_moves(dragon, &board) {
                            let mut new_sheep = sheep;

                            if !board[dragon_move] { // if no hideaway...
                                let rev_pos = board.len() - 1 - dragon_move;
                                if sheep >> rev_pos & 1 == 1 { // if a sheep exists there...
                                    // ...remove the sheep
                                    let sheep_pos = 2u64.pow(rev_pos as u32);
                                    new_sheep -= sheep_pos;
                                }
                            }
                            let next_state = State {
                                dragon: dragon_move,
                                sheep: new_sheep,
                                turn: Turn::Sheep,
                            };
                            next.push(next_state);
                        }
                    },
                    Turn::Sheep => {
                        for one_sheep in iter_sheep(sheep) {
                            if one_sheep >= board.width() { // if not in bottom row...
                                let below_pos = one_sheep - board.width();
                                if sheep >> below_pos & 1 == 0 { // if below row doesn't have a sheep...
                                    let og_val = 2u64.pow(one_sheep as u32);
                                    let mut new_sheep = sheep - og_val;
                                    if (board.len() - 1 - dragon) != below_pos { // if dragon isn't below...
                                        let below_val = 2u64.pow((one_sheep - board.width()) as u32);
                                        new_sheep = sheep - og_val + below_val;
                                    }
                                    let new_state = State {
                                        dragon,
                                        sheep: new_sheep,
                                        turn: Turn::Dragon,
                                    };
                                    if visited.insert(new_state) {
                                        next.push(new_state);
                                    }
                                }
                            }
                        }
                        if next.is_empty() {
                            let pass_the_sheep = State { dragon, sheep, turn: Turn::Dragon };
                            if visited.insert(pass_the_sheep) {
                                next.push(pass_the_sheep);
                            }
                        }
                    },
                }
                next
            })
            .collect();
    }
    variants
}

#[test]
fn default() {
    // let (input1, input2, input3) = get_event_inputs(25, 10);
    // assert_eq!(ZZ, part1(&input1));
    // assert_eq!(ZZ, part2(&input2));
    // assert_eq!(ZZ, part3(&input3));
}

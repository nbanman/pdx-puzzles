use std::iter::successors;
use std::sync::LazyLock;

use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::structs::coord::{Coord2, Coord2U};
use utilities::structs::grid::grid_display::GridDisplay;
use utilities::structs::grid::Grid2;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Pos = Coord2U;
type BasicBoard = Grid2<Space>;
type AdvancedBoard = Grid2<Hedges>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 10);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

static MOVE_TEMPLATE: LazyLock<[Coord2; 8]> = LazyLock::new(|| {
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
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Sheep,
    Hideout,
    SheepHide,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hedges {
    Empty,
    HomeFree,
    Hedge,
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

#[derive(Debug)]
struct Chess {
    dragon: Pos,
    board: BasicBoard,
}

impl Chess {
    fn moves(&self, dragon: Pos) -> impl Iterator<Item = Pos> {
        let dragon = Coord2::from((dragon.x() as i64, dragon.y() as i64));
        MOVE_TEMPLATE
            .into_iter()
            .filter_map(move |move_pos| {
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

fn parse(input: Input) -> (AdvancedBoard, State) {
    let mut board = AdvancedBoard::new2d_map_str(input, |c| {
        match c {
            '#' => Hedges::Hedge,
            _ => Hedges::Empty,
        }
    })
        .unwrap();

    for x in 0..board.width() {
        for y in (0..board.height()).rev() {
            let idx = x + y * board.width();
            if board[idx] == Hedges::Hedge {
                board[idx] = Hedges::HomeFree;
            } else {
                break;
            }
        }
    }
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

fn dragon_moves(dragon: usize, board: &AdvancedBoard) -> impl Iterator<Item = usize> {
    let dragon = Coord2::from(
        (
            (dragon % board.width()) as i64,
            (dragon / board.width()) as i64
        )
    );
    MOVE_TEMPLATE
        .into_iter()
        .filter_map(move |move_pos| {
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

fn count_variants(s: State, cache: &mut FxHashMap<State, usize>, board: &AdvancedBoard, history: String) -> usize {

    // base case 1: all sheep are eaten
    if s.sheep == 0 {
        cache.insert(s, 1);
        return 1;
    }

    if s.turn == Turn::Sheep {
        let sheep_down = iter_sheep(s.sheep)
            .map(|rev_i| board.len() - 1 - rev_i + board.width())
            .collect_vec();

        // base case 2: only one sheep left, next to a home free hedge, so it must escape.
        if sheep_down.len() == 1
            && (sheep_down[0] >= board.len() || board[sheep_down[0]] == Hedges::HomeFree)
        {
            return 0;
        }

        // base case 3: all sheep are on verge of escaping or trapped by dragon and thus one
        // must escape
        if sheep_down.len() > 1 && sheep_down.into_iter().all(|one_down| {
            one_down >= board.len()
                || board[one_down] == Hedges::HomeFree
                || (board[one_down] == Hedges::Empty && one_down == s.dragon)
        }) {
            return 0;
        }
    }

    let mut variants = 0;

    let mut next: Vec<State> = Vec::new();
    match s.turn {
        Turn::Dragon => {
            for dragon_move in dragon_moves(s.dragon, &board) {
                let mut new_sheep = s.sheep;

                if board[dragon_move] == Hedges::Empty { // if no hideaway...
                    let rev_pos = board.len() - 1 - dragon_move;
                    if s.sheep >> rev_pos & 1 == 1 { // if a sheep exists there...
                        // ...remove the sheep
                        let sheep_pos = 1 << rev_pos;
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
            for one_sheep in iter_sheep(s.sheep) {
                if one_sheep >= board.width() { // if not in bottom row...
                    let below_pos = one_sheep - board.width();

                    // if below row doesn't have a sheep or dragon, push the state
                    if s.sheep >> below_pos & 1 == 0 {
                        let below_is_hideaway = board[board.len() - 1 - below_pos] == Hedges::Hedge;
                        if below_is_hideaway || (board.len() - 1 - s.dragon) != below_pos {
                            let og_val = 1 << one_sheep;
                            let below_val = 1 << (one_sheep - board.width());
                            let new_sheep = s.sheep - og_val + below_val;
                            let new_state = State {
                                dragon: s.dragon,
                                sheep: new_sheep,
                                turn: Turn::Dragon,
                            };
                            next.push(new_state);
                        }
                    }
                }
            }
            if next.is_empty() {
                let pass_the_sheep = State { dragon: s.dragon, sheep: s.sheep, turn: Turn::Dragon };
                next.push(pass_the_sheep);
            }
        },
    }

    for next_s in next {
        let mut history = history.clone();
        let idx = match s.turn {
            Turn::Dragon => Some(next_s.dragon),
            Turn::Sheep => {
                let diff = s.sheep ^ next_s.sheep;
                (board.len() - 1).checked_sub(diff.trailing_zeros() as usize)
            }
        };
        if let Some(idx) = idx {
            if s.turn == Turn::Sheep {
                history.push_str("S>");
            } else {
                history.push_str("D>");
            }
            let x = (idx % board.width() + 65) as u8 as char;
            history.push(x);
            let y = idx / board.width() + 1;
            history.push_str(&y.to_string());
            history.push(' ');
        }
        variants += if let Some(&sub_variants) = cache.get(&next_s) {
            sub_variants
        } else {
            let sub_variants = count_variants(next_s, cache, board, history.clone());
            cache.insert(next_s, sub_variants);
            sub_variants
        };
    }
    variants
}

fn part1(input: Input) -> usize {
    let chess = Chess::from(input);
    chess.count_sheep(4)
}

fn part2(input: Input) -> usize {
    let mut chess = Chess::from(input);
    chess.count_moving_sheep(20)
}

fn part3(input: Input) -> usize {
    let (board, initial_state) = parse(input);
    let mut cache: FxHashMap<State, usize> = FxHashMap::default();
    count_variants(initial_state, &mut cache, &board, String::new())
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 10);
    assert_eq!(153, part1(&input1));
    assert_eq!(1743, part2(&input2));
    assert_eq!(3270764079035, part3(&input3));
}

// Input parsed (38μs)
// 1. 153 (48μs)
// 2. 1743 (7.676ms)
// 3. 3270764079035 (529.882ms)
// Total: 537.656ms
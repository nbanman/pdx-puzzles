use advent::utilities::get_input::get_input;
use rustc_hash::FxHashMap;
use utilities::{enums::cardinals::Cardinal, structs::{coord::Coord2, grid::Grid2, stopwatch::{ReportDuration, Stopwatch}}};

type Input = (Pos, Nodes);
type Nodes = FxHashMap<Pos, NodeState>;
type Output = usize;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 22).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl NodeState {
    fn advance(&self) -> Self {
        match self {
            NodeState::Clean => NodeState::Weakened,
            NodeState::Weakened => NodeState::Infected,
            NodeState::Infected => NodeState::Flagged,
            NodeState::Flagged => NodeState::Clean,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Virus {
    pos: Pos,
    dir: Cardinal,
    infections: usize,
}

impl Virus {
    fn move_virus(&self, infected: bool, turn: fn(&Cardinal) -> Cardinal) -> Self {
        let turn_dir = turn(&self.dir);
        Self {
            pos: self.pos.move_direction(turn_dir, 1).unwrap(),
            dir: turn_dir,
            infections: self.infections + if infected { 1 } else { 0 },
        }
    }
}

fn parse_input(input: &str) -> Input {
    let grid = Grid2::try_from(input).unwrap();
    let pos = Pos::new2d((grid.width() / 2) as i64, (grid.height() / 2) as i64);
    let nodes = grid.iter_with_coords()
        .filter(|&(_, &c)| c == '#')
        .map(|(pos, _)| {
            let pos = Pos::new2d(pos.x() as i64, pos.y() as i64);
            (pos, NodeState::Infected)
        })
        .collect();
    (pos, nodes)
}

fn solve(input: Input, bursts: usize, burst: fn(Virus, &mut Nodes) -> Virus) -> Output {
    let (pos, mut nodes) = input;
    let virus = Virus {
        pos,
        dir: Cardinal::North,
        infections: 0,
    };
    (0..bursts).fold(virus, |acc, _x| burst(acc, &mut nodes)).infections
}

fn part1(input: Input) -> Output {
    let burst = |virus: Virus, nodes: &mut Nodes| {
        let current_node = *nodes.get(&virus.pos).unwrap_or(&NodeState::Clean);
        if current_node == NodeState::Clean {
            nodes.insert(virus.pos, NodeState::Infected);
            virus.move_virus(true, Cardinal::left)
        } else {
            nodes.insert(virus.pos, NodeState::Clean);
            virus.move_virus(false, Cardinal::right)
        }
    };
    solve(input, 10_000, burst)
}

fn part2(input: Input) -> Output {
    let burst = |virus: Virus, nodes: &mut Nodes| {
        let current_node = *nodes.get(&virus.pos).unwrap_or(&NodeState::Clean);
        nodes.insert(virus.pos, current_node.advance());
        match current_node {
            NodeState::Clean => virus.move_virus(false, Cardinal::left),
            NodeState::Weakened => virus.move_virus(true, Cardinal::straight),
            NodeState::Infected => virus.move_virus(false, Cardinal::right),
            NodeState::Flagged => virus.move_virus(false, Cardinal::flip),
        }
    };
    solve(input, 10_000_000, burst)
}

#[test]
fn default() {
    let input = get_input(17, 22).unwrap();
    let input = parse_input(&input);
    assert_eq!(5348, part1(input.clone()));
    assert_eq!(2512225, part2(input));
}

// Input parsed (161μs)
// 1. 5348 (433μs)
// 2. 2512225 (298ms)
// Total: 299ms

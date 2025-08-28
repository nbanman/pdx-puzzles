use std::iter::Map;
use std::num::ParseIntError;
use everybody_codes::utilities::inputs::get_story_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};
use lazy_regex::{regex, Captures};
use std::str::Lines;
use color_eyre::eyre::{eyre, Report};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;

enum Command {
    Add { id: usize, left_rank: usize, left_symbol: char, right_rank: usize, right_symbol: char, },
}

impl TryFrom<&str> for Command {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value.split_whitespace().next().unwrap() {
            "ADD" => {
                let pattern = regex!(r"id=(?<id>\d+) left=\[(?<leftrank>\d+),(?<leftsymbol>.)\] right=\[(?<rightrank>\d+),(?<rightsymbol>.)\]");
                let groups = pattern
                    .captures(value)
                    .ok_or(Box::<dyn std::error::Error>::from(format!("Add regex did not match: {}", value)))?;
                Ok(Command::Add {
                    id: Self::capture_int(&groups, "leftrank")?,
                    left_rank: Self::capture_int(&groups, "leftrank")?,
                    left_symbol: Self::capture_char(&groups, "leftsymbol")?,
                    right_rank: Self::capture_int(&groups, "rightrank")?,
                    right_symbol: Self::capture_char(&groups, "rightsymbol")?,
                })
            },
            command => Err(format!("Unknown command: {}", command).into()),
        }
    }
}

impl Command {
    fn capture_int(groups: &Captures, name: &'static str) -> Result<usize> {
        groups
            .name(name)
            .ok_or(Box::<dyn std::error::Error>::from(format!("Missing group: {}", name)))?
            .as_str()
            .parse()
            .map_err(|e|
                Box::<dyn std::error::Error>::from(format!("Could not parse as number: {}", e))
            )
    }

    fn capture_char(groups: &Captures, name: &'static str) -> Result<char> {
        groups
            .name(name)
            .ok_or(Box::<dyn std::error::Error>::from(format!("Missing group: {}", name)))?
            .as_str()
            .parse()
            .map_err(|e|
                Box::<dyn std::error::Error>::from(format!("Could not parse to char: {}", e))
            )
    }
}

struct TangledTree {
    nodes: Vec<Node>,
    left_head: usize,
    right_head: usize,
    left_index_by_id: Vec<usize>,
    right_index_by_id: Vec<usize>,
}
struct Node {
    id: usize,
    level: usize,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    rank: usize,
    symbol: char,
}

fn main() -> Result<()> {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();

    // let (input1, input2, input3) = get_story_inputs(25, 1, 1);
    let input1 = get_story_input(25, 1, 2, 1);
    println!("Input parsed ({})", stopwatch.lap().report());

    println!("1. {} ({})", solve(&input1), stopwatch.lap().report());
    // println!("2. {} ({})", solve(&input2, 5), stopwatch.lap().report());
    // println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
    Ok(())
}

fn solve(input: &str) -> String {
    let commands: Map<Lines, fn(&str) -> Command> = input
        .lines()
        .map(|line| line.try_into().unwrap());

    let test: Vec<_> = commands.collect();
    "hi".to_string()
}

#[test]
fn example() {
    // let (input1, input2, input3) = get_story_inputs(24, 1);
    let input1 = r"ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]";
    let input2 = r"ADD id=1 left=[160,E] right=[175,S]
ADD id=2 left=[140,W] right=[224,D]
ADD id=3 left=[122,U] right=[203,F]
ADD id=4 left=[204,N] right=[114,G]
ADD id=5 left=[136,V] right=[256,H]
ADD id=6 left=[147,G] right=[192,O]
ADD id=7 left=[232,I] right=[154,K]
ADD id=8 left=[118,E] right=[125,Y]
ADD id=9 left=[102,A] right=[210,D]
ADD id=10 left=[183,Q] right=[254,E]
ADD id=11 left=[146,E] right=[148,C]
ADD id=12 left=[173,Y] right=[299,S]
ADD id=13 left=[190,B] right=[277,B]
ADD id=14 left=[124,T] right=[142,N]
ADD id=15 left=[153,R] right=[133,M]
ADD id=16 left=[252,D] right=[276,M]
ADD id=17 left=[258,I] right=[245,P]
ADD id=18 left=[117,O] right=[283,!]
ADD id=19 left=[212,O] right=[127,R]
ADD id=20 left=[278,A] right=[169,C]";
    assert_eq!("CFGNLK".to_string(), solve(input1));
    assert_eq!("EVERYBODYCODES".to_string(), solve(input2));
}

// #[test]
// fn default() {
//     let (input1, input2, input3) = get_story_inputs(25, 1, 1);
//     assert_eq!(1281421558, solve(&input1, 100));
//     assert_eq!(165117476211886, solve(&input2, 5));
//     assert_eq!(670944509842136, part3(&input3));
// }

// Input parsed (54μs)
// 1. 1281421558 (10μs)
// 2. 165117476211886 (82μs)
// 3. 670944509842136 (1ms)
// Total: 1ms

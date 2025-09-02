use anyhow::{anyhow, Error, Result};
use everybody_codes::utilities::inputs::get_story_inputs;
use itertools::Itertools;
use lazy_regex::{regex, Captures};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

#[derive(Debug, Copy, Clone)]
enum Command {
    Add { id: usize, left_rank: usize, left_symbol: char, right_rank: usize, right_symbol: char, },
    Swap(usize),
}

impl TryFrom<&str> for Command {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value.split_whitespace().next().ok_or_else(|| anyhow!("Empty command"))? {
            "ADD" => {
                let pattern = regex!(r"id=(?<id>\d+) left=\[(?<leftrank>\d+),(?<leftsymbol>.)\] right=\[(?<rightrank>\d+),(?<rightsymbol>.)\]");
                let groups = pattern
                    .captures(value)
                    .ok_or_else(|| anyhow!("Add regex did not match (line {}): \"{}\"", line!(), value))?;
                Ok(Command::Add {
                    id: Self::capture_int(&groups, "id")?,
                    left_rank: Self::capture_int(&groups, "leftrank")?,
                    left_symbol: Self::capture_char(&groups, "leftsymbol")?,
                    right_rank: Self::capture_int(&groups, "rightrank")?,
                    right_symbol: Self::capture_char(&groups, "rightsymbol")?,
                })
            },
            "SWAP" => {
                let (_, id) = value
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("Could not parse swap (line {}): \"{}\"", line!(), value))?;
                Ok(Command::Swap(id.parse()?))
            },
            command => Err(anyhow!("Unknown command: {} (line {})", command, line!())),
        } 
    }
}

impl Command {
    fn capture_int(groups: &Captures, name: &'static str) -> Result<usize> {
        let value = groups
            .name(name)
            .ok_or_else(|| anyhow!("Missing group: {}", name))?
            .as_str()
            .parse()?;
        Ok(value)
    }

    fn capture_char(groups: &Captures, name: &'static str) -> Result<char> {
        let value = groups
            .name(name)
            .ok_or_else(|| anyhow!("Missing group: {}", name))?
            .as_str()
            .parse()?;
        Ok(value)
    }
}

#[derive(Debug)]
struct TangledTree {
    nodes: Vec<Node>,
    left_head: usize,
    right_head: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Side {
    Left,
    Right,
}
#[derive(Debug, Copy, Clone)]
struct Node {
    id: usize,
    index: usize,
    side: Side,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    rank: usize,
    symbol: char,
}

impl Node {
    fn child(&self, side: Side) -> Option<usize> {
        match side {
            Side::Left => self.left,
            Side::Right => self.right,
        }
    }
}

fn main() -> Result<()>{
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();

    let (input1, input2, input3) = get_story_inputs(25, 1, 2);
    println!("Input parsed ({})", stopwatch.lap().report());

    println!("1. {} ({})", solve(&input1, false)?, stopwatch.lap().report());
    println!("2. {} ({})", solve(&input2, false)?, stopwatch.lap().report());
    println!("3. {} ({})", solve(&input3, true)?, stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
    Ok(())
}

fn solve(input: &str, swap_entire_node: bool) -> Result<String> {
    let mut commands = input
        .lines()
        .map(|line| Command::try_from(line));

    let mut tree = TangledTree {
        nodes: Vec::new(),
        left_head: 0,
        right_head: 1,
    };

    // establish heads
    let heads = commands
        .next()
        .ok_or_else(|| anyhow!("Missing heads"))??;
    match heads {
        Command::Add { id, left_rank, left_symbol, right_rank, right_symbol } => {
            let left_head = Node {
                id,
                index: 0,
                side: Side::Left,
                parent: None,
                left: None,
                right: None,
                rank: left_rank,
                symbol: left_symbol,
            };
            let right_head = Node {
                id,
                index: 1,
                side: Side::Right,
                parent: None,
                left: None,
                right: None,
                rank: right_rank,
                symbol: right_symbol,
            };
            tree.nodes.push(left_head);
            tree.nodes.push(right_head);
        },

        Command::Swap(_) => {
            return Err(anyhow!("Swap cannot be the first command"));
        }
    }

    for command in commands {
        let command = command?;
        match command {
            Command::Add { id, left_rank, left_symbol, right_rank, right_symbol } => {
                let (left_parent, left_side) =
                    find_parent(left_rank, tree.nodes[tree.left_head], &tree.nodes)?;
                let index = tree.nodes.len();
                match left_side {
                    Side::Left => {
                        tree.nodes[left_parent.index].left = Some(index);
                    },
                    Side::Right => {
                        tree.nodes[left_parent.index].right = Some(index)
                    },
                }
                let left_node = Node {
                    id,
                    index,
                    side: left_side,
                    parent: Some(left_parent.index),
                    left: None,
                    right: None,
                    rank: left_rank,
                    symbol: left_symbol,
                };
                tree.nodes.push(left_node);
                let (right_parent, right_side) =
                    find_parent(right_rank, tree.nodes[tree.right_head], &tree.nodes)?;
                let index = index + 1;
                match right_side {
                    Side::Left => {
                        tree.nodes[right_parent.index].left = Some(index);
                    }
                    Side::Right => {
                        tree.nodes[right_parent.index].right = Some(index);
                    },
                }
                let right_node = Node {
                    id,
                    index,
                    side: right_side,
                    parent: Some(right_parent.index),
                    left: None,
                    right: None,
                    rank: right_rank,
                    symbol: right_symbol,
                };
                tree.nodes.push(right_node);
            },
            Command::Swap(id) => {
                if swap_entire_node {
                    if id == 1 { // if it's the head nodes, just swap head markers
                        let swap_head = tree.left_head;
                        tree.left_head = tree.right_head;
                        tree.right_head = swap_head;
                    } else { // if it's a child node, more complicated
                        let (a, b) = nodes_at_id(&tree, id)?; // get the nodes

                        // clone some values to make things easier
                        let a_node = tree.nodes[a].clone();
                        let b_node = tree.nodes[b].clone();

                        // swap parents and sides
                        tree.nodes[a_node.index].parent = b_node.parent;
                        tree.nodes[a_node.index].side = b_node.side;
                        tree.nodes[b_node.index].parent = a_node.parent;
                        tree.nodes[b_node.index].side = a_node.side;

                        // swap child links
                        let a_parent_index = a_node.parent
                            .ok_or_else(|| anyhow!("Missing parent index"))?;
                        swap_children(&mut tree, a_node, b_node, a_parent_index);

                        let b_parent_index = b_node.parent
                            .ok_or_else(|| anyhow!("Missing parent index"))?;
                        swap_children(&mut tree, b_node, a_node, b_parent_index);
                    }
                } else {
                    let (a, b) = nodes_at_id(&tree, id)?;
                    let swap_rank = tree.nodes[a].rank;
                    let swap_symbol = tree.nodes[a].symbol;
                    tree.nodes[a].rank = tree.nodes[b].rank;
                    tree.nodes[a].symbol = tree.nodes[b].symbol;
                    tree.nodes[b].rank = swap_rank;
                    tree.nodes[b].symbol = swap_symbol;
                }
            },
        }
    }
    let left_message = sort_by_rank(tree.left_head, &tree.nodes)?;
    let right_message = sort_by_rank(tree.right_head, &tree.nodes)?;

    Ok(left_message + &right_message)
}

fn swap_children(tree: &mut TangledTree, first: Node, second: Node, parent_index: usize) {
    match first.side {
        Side::Left => {
            tree.nodes[parent_index].left = Some(second.index);
        },
        Side::Right => {
            tree.nodes[parent_index].right = Some(second.index);
        },
    }
}

fn nodes_at_id(tree: &TangledTree, id: usize) -> Result<(usize, usize), Error> {
    let (a, b) = tree.nodes.iter()
        .filter(|node| node.id == id)
        .map(|node| node.index)
        .collect_tuple()
        .ok_or_else(|| anyhow!("Can't find two nodes with id {}", id))?;
    Ok((a, b))
}

fn sort_by_rank(index: usize, nodes: &[Node]) -> Result<String> {
    let mut levels: Vec<String> = Vec::new();
    traverse(&mut levels, index, 0, nodes);
    let longest = levels.iter().map(|level| level.len()).max().unwrap();
    let s = levels.into_iter()
        .find(|level| level.len() == longest)
        .ok_or_else(|| anyhow!("Empty levels"))?;
    Ok(s)
}

fn traverse(levels: &mut Vec<String>, index: usize, level: usize, nodes: &[Node]) {
    let node = &nodes[index];
    if levels.len() == level {
        levels.push(String::new());
    }
    levels[level].push(node.symbol);
    if let Some(left_index) = node.left {
        traverse(levels, left_index, level + 1, nodes);
    }
    if let Some(right_index) = node.right {
        traverse(levels, right_index, level + 1, nodes);
    }
}

fn find_parent(rank: usize, parent: Node, nodes: &Vec<Node>) -> Result<(Node, Side)> {
    if rank < parent.rank {
        check_branch(rank, parent, nodes, Side::Left)
    } else if rank > parent.rank {
        check_branch(rank, parent, nodes, Side::Right)
    } else {
        Err(anyhow!("Rank identical to parent"))
    }
}

fn check_branch(rank: usize, parent: Node, nodes: &Vec<Node>, side: Side) -> Result<(Node, Side)> {
    if let Some(child_index) = parent.child(side) {
        find_parent(rank, nodes[child_index], nodes)
    } else {
        Ok((parent, side))
    }
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
    let input3 = r"ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]";
    let input4 = r"ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]
SWAP 2";
    let input5 = r"ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]
SWAP 2
SWAP 5";
    assert_eq!("CFGNLK".to_string(), solve(input1, false).unwrap());
    assert_eq!("EVERYBODYCODES".to_string(), solve(input2, false).unwrap());
    assert_eq!("MGFLNK".to_string(), solve(input3, false).unwrap());
    assert_eq!("DJMGL".to_string(), solve(input4, true).unwrap());
    assert_eq!("DJCGL".to_string(), solve(input5, true).unwrap());
}

#[test]
fn default() {
    let (input1, input2, input3) = get_story_inputs(25, 1, 2);
    assert_eq!("QUACK!LWXRVSGG", solve(&input1, false).unwrap());
    assert_eq!("QUACK!VPFSJYPGYNTVPY", solve(&input2, false).unwrap());
    assert_eq!("QUACK!GMRZLRSZFLPLZJRYTSJWPRZYZLJW", solve(&input3, true).unwrap());
}

// Input parsed (44μs)
// 1. QUACK!LWXRVSGG (548μs)
// 2. QUACK!VPFSJYPGYNTVPY (78μs)
// 3. QUACK!GMRZLRSZFLPLZJRYTSJWPRZYZLJW (184μs)
// Total: 857μs
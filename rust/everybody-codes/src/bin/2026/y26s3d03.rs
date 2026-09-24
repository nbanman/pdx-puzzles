use std::sync::LazyLock;

use everybody_codes::utilities::inputs::get_story_inputs_provisional;
use itertools::Itertools;
use lazy_regex::Regex;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;

type Tree<'a> = Vec<Node<'a>>;

#[derive(Clone, Debug)]
struct Node<'a> {
    id: usize,
    plug_color: &'a str,
    plug_shape: &'a str,
    left_color: &'a str,
    left_shape: &'a str,
    left_bond: Bond,
    right_color: &'a str,
    right_shape: &'a str,
    right_bond: Bond,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BondRules {
    StrongOnly,
    Weak,
    StrongBreaksWeak,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Bond {
    None,
    Strong(usize),
    Weak(usize),
}

impl<'a> From<&'a str> for Node<'a> {
    fn from(line: &'a str) -> Self {
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"([^=\n]+)=([^,\n]+)(?:, )?").unwrap());
        let (id, plug, left, right, _) = RE
            .captures_iter(line)
            .map(|caps| caps.get(2).unwrap().as_str())
            .collect_tuple()
            .unwrap();
        let (plug_color, plug_shape) = plug.split_once(' ').unwrap();
        let (left_color, left_shape) = left.split_once(' ').unwrap();
        let (right_color, right_shape) = right.split_once(' ').unwrap();
        Self {
            id: id.parse().unwrap(),
            plug_color,
            plug_shape,
            left_color,
            left_shape,
            left_bond: Bond::None,
            right_color,
            right_shape,
            right_bond: Bond::None,
        }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_story_inputs_provisional(26, 3, 3);
    println!("Input parsed ({})", stopwatch.lap().report());
    if let Some(input1) = input1 {
        println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    }
    if let Some(input2) = input2 {
        println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    }
    if let Some(input3) = input3 {
        println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    }
    println!("Total: {}", stopwatch.stop().report());
}

fn build_tree(input: Input, bond_rules: BondRules) -> Tree {
    let mut tree = Tree::new();
    let mut lines = input.lines();
    let root_node = Node::from(lines.next().unwrap());
    tree.push(root_node);
    for (node_index, line) in lines.enumerate() {
        tree.push(Node::from(line));
        let mut node_index = node_index + 1;
        add_node(&mut tree, 0, &mut node_index, bond_rules);
    }
    tree
}

fn add_node(tree: &mut Tree, root_index: usize, node_index: &mut usize, bond_rules: BondRules) -> bool {
    let mut placed = false;

    let color_match = tree[root_index].left_color == tree[*node_index].plug_color;
    let shape_match = tree[root_index].left_shape == tree[*node_index].plug_shape;
    match tree[root_index].left_bond {
        Bond::None => {
            if color_match && shape_match {
                tree[root_index].left_bond = Bond::Strong(*node_index);
                placed = true;
            } else if bond_rules != BondRules::StrongOnly {
                if color_match || shape_match {
                    tree[root_index].left_bond = Bond::Weak(*node_index);
                    placed = true;
                }
            }
        },
        Bond::Strong(index) => {
            placed = add_node(tree, index, node_index, bond_rules);
        },
        Bond::Weak(index) => {
            match bond_rules {
                BondRules::StrongOnly => unreachable!(),
                BondRules::Weak => {
                    placed = add_node(tree, index, node_index, bond_rules);
                },
                BondRules::StrongBreaksWeak => {
                    if color_match && shape_match {
                        tree[root_index].left_bond = Bond::Strong(*node_index);
                        *node_index = index;
                    } else {
                        placed = add_node(tree, index, node_index, bond_rules);
                    }
                },
            }
        },
    }
    if placed {
        return true;
    }
    let color_match = tree[root_index].right_color == tree[*node_index].plug_color;
    let shape_match = tree[root_index].right_shape == tree[*node_index].plug_shape;
    match tree[root_index].right_bond {
        Bond::None => {
            if color_match && shape_match {
                tree[root_index].right_bond = Bond::Strong(*node_index);
                placed = true;
            } else if bond_rules != BondRules::StrongOnly {
                if color_match || shape_match {
                    tree[root_index].right_bond = Bond::Weak(*node_index);
                    placed = true;
                }
            }
        },
        Bond::Strong(index) => {
            placed = add_node(tree, index, node_index, bond_rules);
        },
        Bond::Weak(index) => {
            match bond_rules {
                BondRules::StrongOnly => unreachable!(),
                BondRules::Weak => {
                    placed = add_node(tree, index, node_index, bond_rules);
                },
                BondRules::StrongBreaksWeak => {
                    if color_match && shape_match {
                        tree[root_index].right_bond = Bond::Strong(*node_index);
                        *node_index = index;
                    } else {
                        placed = add_node(tree, index, node_index, bond_rules);
                    }
                },
            }
        },
    }
        if placed {
            return true;
        }
        if root_index == 0 {
            add_node(tree, root_index, node_index, bond_rules)
        } else {
            false
        }
}

fn read_ids(tree: &Tree, root: usize, ids: &mut Vec<usize>) {
    match tree[root].left_bond {
        // read_ids(tree, left_index, ids);
        Bond::None => {},
        Bond::Strong(index) => read_ids(tree, index, ids),
        Bond::Weak(index) => read_ids(tree, index, ids),
    }
    ids.push(tree[root].id);
    match tree[root].right_bond {
        // read_ids(tree, left_index, ids);
        Bond::None => {},
        Bond::Strong(index) => read_ids(tree, index, ids),
        Bond::Weak(index) => read_ids(tree, index, ids),
    }
}

fn checksum(ids: &Vec<usize>) -> usize {
    ids.into_iter()
        .enumerate()
        .map(|(idx, id)| (idx + 1) * id)
        .sum()
}

fn part1(input: Input) -> usize {
    let tree = build_tree(input, BondRules::StrongOnly);
    let mut ids = Vec::new();
    read_ids(&tree, 0, &mut ids);
    checksum(&ids)
}

fn part2(input: Input) -> usize {
    let tree = build_tree(input, BondRules::Weak);
    let mut ids = Vec::new();
    read_ids(&tree, 0, &mut ids);
    checksum(&ids)
}

fn part3(input: Input) -> usize {
    let tree = build_tree(input, BondRules::StrongBreaksWeak);
    let mut ids = Vec::new();
    read_ids(&tree, 0, &mut ids);
    checksum(&ids)
}

#[test]
fn default() {
    // let (input1, input2, input3) =
    //        everybody_codes::utilities::inputs::get_story_inputs(26, 3, 2);
    // assert_eq!(ZZ, part1(&input1));
    // assert_eq!(ZZ, part2(&input2));
    // assert_eq!(ZZ, part3(&input3));
}

#[test]
fn example() {
    let input1 = r"id=1, plug=BLUE HEXAGON, leftSocket=GREEN CIRCLE, rightSocket=BLUE PENTAGON, data=?
id=2, plug=GREEN CIRCLE, leftSocket=BLUE HEXAGON, rightSocket=BLUE CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=BLUE CIRCLE, data=?
id=4, plug=BLUE CIRCLE, leftSocket=RED HEXAGON, rightSocket=BLUE HEXAGON, data=?
id=5, plug=RED HEXAGON, leftSocket=GREEN CIRCLE, rightSocket=RED HEXAGON, data=?";
    assert_eq!(43, part1(input1));
    let input2 = r"id=1, plug=RED TRIANGLE, leftSocket=RED TRIANGLE, rightSocket=RED TRIANGLE, data=?
id=2, plug=GREEN TRIANGLE, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=4, plug=RED TRIANGLE, leftSocket=BLUE PENTAGON, rightSocket=GREEN PENTAGON, data=?
id=5, plug=RED PENTAGON, leftSocket=GREEN CIRCLE, rightSocket=GREEN CIRCLE, data=?";
    assert_eq!(50, part2(input2));
    assert_eq!(38, part3(input2));
    let input3 = r"id=1, plug=RED TRIANGLE, leftSocket=BLUE TRIANGLE, rightSocket=GREEN TRIANGLE, data=?
id=2, plug=GREEN TRIANGLE, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=4, plug=RED TRIANGLE, leftSocket=BLUE PENTAGON, rightSocket=GREEN PENTAGON, data=?
id=5, plug=BLUE TRIANGLE, leftSocket=GREEN CIRCLE, rightSocket=RED CIRCLE, data=?
id=6, plug=BLUE TRIANGLE, leftSocket=GREEN CIRCLE, rightSocket=RED CIRCLE, data=?";
    assert_eq!(60, part3(input3));
}

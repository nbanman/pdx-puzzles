use advent::utilities::get_input::get_input;
use itertools::Itertools;
use lazy_regex::regex;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = (&'a str, FxHashMap<&'a str, Program<'a>>);

#[derive(Debug, Clone)]
struct Program<'a> {
    id: usize,
    weight: usize,
    children: Option<Vec<&'a str>>,
}

impl<'a> Program<'a> {
    fn rebalance(
        &self,
        register: &FxHashMap<&str, Program>,
        total_weight: &mut [Option<usize>], 
        ideal_weight: usize
    ) -> usize {
        let Some(children) = &self.children else { return ideal_weight; };
        let children = children.iter()
            .map(|&child_name| register.get(child_name).unwrap())
            .collect_vec();

        // if all children have the same total weight...
        let child_weights = children.iter()
            .map(|&child| child.total_weight(register, total_weight))
            .collect_vec();
        // ...then this particular program's weight needs to change, then we can RETURN that value
        // calculated by taking the ideal weight and subtracting the weight of the children
        if child_weights.iter().all(|&weight| weight == child_weights[0]) {
            return ideal_weight - child_weights.into_iter().sum::<usize>()
        }

        // otherwise we continue...
        // we calculate the weight that each child needs to be. There is a starting case and a general case.
        let ideal_child_weight = if ideal_weight == 0 {
            // starting case where we don't know what's unbalanced yet, but we definitely have one child
            // that's unlike at least two others that are the same. Thus, the weight that is shared between
            // at least two children is the ideal child weight.
            children.iter()
                .counts_by(|&child| child.total_weight(register, total_weight))
                .iter()
                .find(|&(_, &count)| count > 1)
                .map(|(&weight, _)| weight)
                .unwrap()
        } else {
            // The "starting case" algorithm works in cases where there are 3+ children, but it doesn't
            // know which branch to correct when there are two children.
            // However, we now know the amount needed to balance down below, so the ideal child weight is the
            // ideal weight minus the current node's weight, then divided evenly among the children.
            (ideal_weight - self.weight) / children.len()
        };

        // Find the child that does not have the proper weight and rebalance it recursively.
        let odd_child = children.iter()
            .find(|child| child.total_weight(register, total_weight) != ideal_child_weight)
            .unwrap();
        odd_child.rebalance(register, total_weight, ideal_child_weight)
    }
    
    fn total_weight(&self, register: &FxHashMap<&str, Program>, total_weight: &mut [Option<usize>]) -> usize {
        if let Some(total) = total_weight[self.id] {
            return total;
        }
        let mut total = self.weight;
        if let Some(children) = &self.children {
            for &child in children {
                total += register.get(child).unwrap().total_weight(register, total_weight);
            }
        }
        total
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 7).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input<'_> {
    let rx = regex!(r"(?<name>[a-z]+) \((?<weight>\d+)\)(?: -> (?<children>[^\n]+))?");
    let register: FxHashMap<&str, Program> = rx.captures_iter(input).enumerate()
        .map(|(id, caps)| {
            let name = caps.name("name").unwrap().as_str();
            let weight: usize = caps.name("weight").unwrap().as_str().parse().unwrap();
            let children = caps.name("children")
                .map(|children| {
                    children.as_str().split(", ").collect_vec()
                });
            let program = Program { id, weight, children };
            (name, program)
        })
        .collect();

    let program_names: FxHashSet<&str> = register.keys().copied().collect();
    let child_names: FxHashSet<&str> = register.values()
        .filter(|program| program.children.is_some())
        .flat_map(|program| program.children.as_ref().unwrap())
        .copied()
        .collect();
    let bottom_program = *program_names.difference(&child_names).next().unwrap();
    (bottom_program, register)
}

fn part1(input: &Input) -> String {
    let (bottom_program, _) = input;
    bottom_program.to_string()
}

fn part2(input: &Input) -> usize {
    let (bottom_program, register) = input;
    let mut total_weight = vec![None; register.len()];
    register.get(*bottom_program).unwrap().rebalance(register, &mut total_weight, 0)
}

#[test]
fn default() {
    let input = get_input(17, 7).unwrap();
    let input = parse_input(&input);
    assert_eq!("airlri".to_string(), part1(&input));
    assert_eq!(1206, part2(&input));
}

// Input parsed (1ms)
// 1. airlri (5μs)
// 2. 1206 (51μs)
// Total: 1ms

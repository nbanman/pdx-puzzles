use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use std::collections::HashMap;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Branches<'a> = HashMap<&'a str, Vec<&'a str>>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(24, 6);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!(
        "1. {} ({})",
        solve(&input1, false),
        stopwatch.lap().report()
    );
    println!("2. {} ({})", solve(&input2, true), stopwatch.lap().report());
    println!("3. {} ({})", solve(&input3, true), stopwatch.lap().report());
    println!("Total: {}", stopwatch);
}

fn solve(input: &str, truncate: bool) -> String {
    let branches = get_branches(input);
    let paths = get_paths(truncate, branches);
    get_strongest(paths)
}

fn get_branches(input: &str) -> Branches {
    input
        .lines()
        .filter(|line| {
            let possible_pest = &line[0..3];
            possible_pest != "ANT" && possible_pest != "BUG"
        })
        .map(|line| {
            let (parent, children) = line.split_once(':').unwrap();
            let children: Vec<&str> = children.split(',').collect();
            (parent, children)
        })
        .collect()
}

fn get_paths(truncate: bool, branches: HashMap<&str, Vec<&str>>) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();

    let mut q = vec![vec!["RR"]];
    while let Some(path) = q.pop() {
        let &current = path.last().unwrap();
        if current == "@" {
            let mut path_name = String::new();

            for s in path {
                if truncate {
                    path_name.push(s.chars().next().unwrap());
                } else {
                    path_name.push_str(s);
                }
            }
            paths.push(path_name);
        } else if let Some(children) = branches.get(current) {
            for child in children {
                let mut new_path = path.clone();
                new_path.push(child);
                q.push(new_path);
            }
        }
    }
    paths
}

fn get_strongest(paths: Vec<String>) -> String {
    paths
        .iter()
        .into_group_map_by(|&s| s.len())
        .values()
        .find(|paths| paths.len() == 1)
        .expect("All values have a matching length value.")[0]
        .to_string()
}

#[test]
fn examples() {
    let test1 = r"
RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@
    "
    .trim();
    assert_eq!("RRB@".to_string(), solve(test1, false));
    assert_eq!("RB@", solve(test1, true));
}

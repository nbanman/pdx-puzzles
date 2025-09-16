use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, BitXor};
use itertools::Itertools;
use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};
use crate::Gate::{And, Or, Output, Xor};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 24).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone)]
enum Gate {
    Output(bool),
    And { left: String, right: String },
    Or { left: String, right: String },
    Xor { left: String, right: String },
}

impl Gate {
    fn to_output(&self, wiring: &HashMap<String, Gate>) -> Option<Gate> {
        let value = match self {
            Gate::Output(value) => *value,
            Gate::And { left, right } =>
                Self::op_output(left.clone(), right.clone(), bool::bitand, wiring)?,
            Gate::Or { left, right } =>
                Self::op_output(left.clone(), right.clone(), bool::bitor, wiring)?,
            Gate::Xor { left, right } =>
                Self::op_output(left.clone(), right.clone(), bool::bitxor, wiring)?,
        };
        Some(Output(value))
    }

    fn op_output<F>(
        left: String,
        right: String,
        op: F,
        wiring: &HashMap<String, Gate>
    ) -> Option<bool>
    where F: Fn(bool, bool) -> bool
    {
        let Output(left_gate) = &wiring[&left] else { return None };
        let Output(right_gate) = &wiring[&right] else { return None };
        Some(op(*left_gate, *right_gate))
    }
}
fn get_wiring(input: Input) -> HashMap<String, Gate> {
    let mut wiring = HashMap::new();
    let (output_gates, conditional_gates) = input.split_once("\n\n").unwrap();

    for s in output_gates.lines() {
        let (id, value) = s.split_once(": ").unwrap();
        wiring.insert(id.to_string(), Gate::Output(value == "1"));
    }

    for s in conditional_gates.lines() {
        let (left, op, right, _, id) = s.split(' ').collect_tuple().unwrap();
        let left = left.to_string();
        let right = right.to_string();
        let id = id.to_string();
        let gate = match op {
            "AND" => And { left, right },
            "OR" => Or { left, right },
            "XOR" => Xor { left, right },
            _ => panic!("invalid input {}", s),
        };
        wiring.insert(id, gate);
    }

    wiring
}


fn part1(input: Input) -> u64 {
    let mut wiring = get_wiring(input);
    let mut pending: HashMap<_, _> = wiring.clone().into_iter()
        .filter(|(_, gate)| !matches!(gate, Output(_)))
        .collect();
    let mut remaining_zs = wiring.keys()
        .filter(|id| id.starts_with('z'))
        .count();
    while remaining_zs > 0 {
        for (id, gate) in pending.clone() {
            let Some(output) = gate.to_output(&wiring) else { continue; };
            pending.remove(&id);
            if id.starts_with('z') {
                remaining_zs -= 1;
            }
            wiring.insert(id, output);
        }
    }

    wiring.into_iter()
        .filter(|(id, _)| id.starts_with('z'))
        .sorted_unstable_by(|(a, _), (b, _)| b.cmp(a))
        .filter_map(|(_, gate)| {
            if let Output(value) = gate {
                Some(value)
            } else {
                None
            }
        })
        .fold(0, |acc, bit| (acc << 1) | bit as u64)
}

fn part2(input: Input) -> String {
    let wiring = get_wiring(input);
    let mut upstream: HashMap<String, Vec<(String, Gate)>> = HashMap::new();
    for (output_id, gate) in wiring.iter() {
        let (left, right) = match gate {
            Output(_) => { continue; },
            And { left, right} => (left, right),
            Gate::Or { left, right} => (left, right),
            Gate::Xor { left, right} => (left, right),
        };
        upstream
            .entry(left.clone())
            .or_insert(Vec::new())
            .push((output_id.clone(), gate.clone()));
        upstream
            .entry(right.clone())
            .or_insert(Vec::new())
            .push((output_id.clone(), gate.clone()));
    }
    let upstream = upstream;

    // get first carry value
    let mut carry = upstream["x00"].iter()
        .find(|(_, gate)| matches!(gate, And { .. }))
        .unwrap()
        .0
        .clone();

    let mut other_errors = Vec::new();

    for i in 1..wiring.iter().filter(|(_, gate)| matches!(gate, Output(_))).count() / 2 {
        let (mut xor1, mut and1) = upstream[&format!("x{:02}", i)].iter()
            .tuple_windows()
            .map(|(a, b)| {
                let a = a.clone();
                let b = b.clone();
                if let Xor { .. } = a.1 {
                    (a.0, b.0)
                } else {
                    (b.0, a.0)
                }
            })
            .next()
            .unwrap();

        let check = upstream[&carry].first().unwrap().1.clone();
        if let Xor { left, right } = check.clone() {
            if xor1 != left && xor1 != right {
                std::mem::swap(&mut xor1, &mut and1);
                other_errors.push(xor1.clone());
                other_errors.push(and1.clone());
            }
        }
        if let And { left, right } = check {
            if xor1 != left && xor1 != right {
                std::mem::swap(&mut xor1, &mut and1);
                other_errors.push(xor1.clone());
                other_errors.push(and1.clone());
            }
        }

        let (swap, and2) = upstream[&xor1].iter()
            .tuple_windows()
            .map(|(a, b)| {
                let a = a.clone();
                let b = b.clone();
                if let Xor { .. } = a.1 {
                    (a.0, b.0)
                } else {
                    (b.0, a.0)
                }
            })
            .next()
            .unwrap();

        if and1.starts_with('z') {
            other_errors.push(swap.clone());
            carry = upstream[&and2]
                .first()
                .unwrap()
                .clone()
                .0;
        } else if and2.starts_with('z') {
            other_errors.push(swap.clone());
            carry = upstream[&and1]
                .first()
                .unwrap()
                .clone()
                .0;
        } else {
            let or = upstream[&and1]
                .first()
                .unwrap()
                .clone()
                .0;
            if or.starts_with('z') && &or != "z45" {
                other_errors.push(swap.clone());
                carry = swap
            } else {
                carry = or
            }
        }
    }

    let z_errors = wiring.into_iter()
        .filter(|(id, gate)| {
            id.starts_with('z') && !matches!(gate, Xor { .. }) && id != "z45"
        })
        .map(|(id, _)| id)
        .collect_vec();

    let combined = other_errors.into_iter()
        .chain(z_errors.into_iter())
        .sorted_unstable()
        .join(",");
    combined
}

#[test]
fn default() {
    let input = get_input(24, 24).unwrap();
    assert_eq!(51410244478064, part1(&input));
    assert_eq!("gst,khg,nhn,tvb,vdc,z12,z21,z33".to_string(), part2(&input));
}

// Input parsed (19μs)
// 1. 51410244478064 (587μs)
// 2. gst,khg,nhn,tvb,vdc,z12,z21,z33 (153μs)
// Total: 763μs
use std::{
    cmp::{Reverse, max},
    collections::BinaryHeap,
};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Vec<u32>>;
type Output = u32;
type State = (usize, u8);

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 9).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn city_index<'a>(c: &'a str, cities: &mut Vec<&'a str>) -> usize {
    cities
        .iter()
        .position(|&city| c == city)
        .unwrap_or_else(|| {
            cities.push(c);
            cities.len() - 1
        })
}

fn insert_dist(edge_map: &mut Vec<Vec<u32>>, dist: &str, a: usize, b: usize) {
    let a_list = edge_map.get_mut(a).unwrap();
    while a_list.len() < b + 1 {
        a_list.push(0);
    }
    a_list[b] = dist.parse().unwrap();
}

fn parse_input(input: &str) -> Input {
    let mut cities: Vec<&str> = Vec::with_capacity(10);
    let mut edge_map: Vec<Vec<u32>> = Vec::with_capacity(10);
    for (a, _, b, _, dist) in input
        .lines()
        .map(|it| it.split(' ').collect_tuple().unwrap())
    {
        let a = city_index(a, &mut cities);
        let b = city_index(b, &mut cities);
        while edge_map.len() < max(a, b) + 1 {
            edge_map.push(Vec::new());
        }
        insert_dist(&mut edge_map, dist, a, b);
        insert_dist(&mut edge_map, dist, b, a);
    }
    edge_map
}

fn part1(edges: &Input) -> Output {
    (0..edges.len())
        .map(|city| {
            let start_state: State = (city, 1 << city);
            let mut weights: FxHashMap<State, u32> = FxHashMap::default();
            let mut q: BinaryHeap<Reverse<(u32, State)>> = BinaryHeap::new();
            q.push(Reverse((0, start_state)));
            let mut visited: FxHashSet<State> = FxHashSet::default();
            let completed = (2u32.pow(edges.len() as u32) - 1) as u8;
            let mut steps = u32::MAX;
            while let Some(Reverse((dist, (city, tour)))) = q.pop() {
                if !visited.insert((city, tour)) {
                    continue;
                }
                if tour == completed {
                    steps = dist;
                    break;
                }
                for n in 0..edges.len() {
                    if (tour >> n) & 1 == 1 {
                        continue;
                    }
                    let n_dist = edges[city][n];
                    if n_dist == 0 {
                        continue;
                    }
                    let n_tour = tour | (1 << n);
                    let alt_dist = dist + n_dist;
                    let n_state = (n, n_tour);
                    let weight = *weights.get(&n_state).unwrap_or(&u32::MAX);
                    if alt_dist < weight {
                        weights.insert(n_state, alt_dist);
                        q.push(Reverse((alt_dist, n_state)));
                    }
                }
            }
            steps
        })
        .min()
        .unwrap()
}

fn part2(edges: &Input) -> Output {
    (0..edges.len())
        .map(|city| {
            let start_state = vec![city];
            let mut todo = Vec::new();
            let mut next = Vec::new();
            todo.push((start_state.clone(), 0));
            let mut visited: FxHashSet<Vec<usize>> = FxHashSet::default();
            visited.insert(start_state);
            let mut longest = 0;
            while !todo.is_empty() {
                for (cities, dist) in todo.drain(..) {
                    if cities.len() == edges.len() && dist > longest {
                        longest = dist;
                    }
                    for n in 0..edges.len() {
                        let n_dist = edges[*cities.last().unwrap()].get(n);
                        let Some(&n_dist) = n_dist else { continue; };
                        if n_dist == 0 || cities.contains(&n) { continue; }
                        let mut n_cities = cities.clone();
                        n_cities.push(n);
                        if visited.insert(n_cities.clone()) {
                            next.push((n_cities, dist + n_dist));
                        }
                    }
                }
                std::mem::swap(&mut todo, &mut next);
            }
            longest
        })
        .max()
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(15, 9).unwrap();
    let input = parse_input(&input);
    assert_eq!(207, part1(&input));
    assert_eq!(804, part2(&input));
}

// Input parsed (26μs)
// 1. 207 (292μs)
// 2. 804 (20ms)
// Total: 20ms

use advent::utilities::get_input::get_input;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use itertools::Itertools;
use utilities::graphs::{bfs, dijkstra, no_end_condition, EdgeInfo, PathInfo};
use utilities::structs::grid::Grid2;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Grid2<char>;
type Output = usize;
type CharSet = [bool; 26];

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 18).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

/**
 * Used for the edgemap, contains enhanced edge data. This includes the location of the edge, but
 * also all keys needed to be collected to make the move (precedingKeys) as well as all keys that
 * are encountered while traveling to the key (interveningKeys). precedingKeys is used to filter out
 * moves that are not yet possible due to not having the right key to pass through a door.
 * interveningKeys is used to update the state so that all keys are added in one swoop.
 */
#[derive(Clone, Copy)]
struct Key {
    current: char,
    preceding: CharSet,
    intervening: CharSet,
}

impl Debug for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let p = self.preceding.iter().enumerate()
            .filter(|&(_, &b)| b)
            .map(|(i, _)| (i + 'a' as usize) as u8 as char)
            .join(",");
        let i = self.intervening.iter().enumerate()
            .filter(|&(_, &b)| b)
            .map(|(i, _)| (i + 'a' as usize) as u8 as char)
            .join(",");
        format!("{:?}, pre: {p}, inter: {i}", self.current).fmt(f)
    }
}

/**
 * Used for the main Dijkstra search. Contains the current position of each robot (location) and all
 * keys that have been collected up to that point (keys).
 */
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    location: Vec<char>,
    keys: CharSet,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let k = self.keys.iter().enumerate()
            .filter(|&(_, &b)| b)
            .map(|(i, _)| i + 'a' as usize)
            .join(",");
        format!("{:?}: {k}", self.location).fmt(f)
    }
}

fn parse_input(input: &str) -> Input {
    input.try_into().unwrap()
}

fn solve(tunnels: &Input, robots: Vec<char>) -> Output {
    // Intermediate edge map used to later make the keyEdges edge map used for the final solution.
    // Just gives the distance to the nearest keys and doors. Once a key or door is reached, the BFS
    // does not explore beyond that key or door. But it will still look for other keys or doors that
    // do not require going through that key or door.
    let mut basic_edges: HashMap<char, Vec<(char, usize)>, FxBuildHasher> = FxHashMap::default();
    for (index, &start) in tunnels.iter()
        .enumerate()
        .filter(|&(_, c)| c.is_ascii_alphabetic() || robots.contains(c))
    {
        let get_edges = |_: EdgeInfo<usize>, (index, current): &(usize, char)|
            -> Vec<(usize, char)>
        {
            // If a key or door is reached, end this particular path.
            if current.is_ascii_alphabetic() && *current != start {
                Vec::new()
            } else {
                tunnels.adjacent(*index, false)
                    .unwrap()
                    .filter(|adj| *adj.value != '#')
                    .map(|adj| (adj.index, *adj.value))
                    .collect()
            }
        };

        let edges = bfs((index, start), get_edges, no_end_condition);

        let edges = edges
            .nodes
            .into_iter()
            .skip(1)
            .filter(|&((_, c), _)| c.is_ascii_alphabetic())
            .map(|((_, c), info)| {
                (c, info.cost)
            })
            .collect();
        basic_edges.insert(start, edges);
    }

    let key_edges: FxHashMap<char, Vec<(Key, usize)>> = basic_edges
        .keys()
        .filter(|&&key| key.is_lowercase() || robots.contains(&key))
        .map(|&key| {
            let edges: PathInfo<char, usize> = dijkstra(
                key,
                |_, c| basic_edges.get(c).unwrap().clone(),
                no_end_condition
            );
            let edges: Vec<_> = edges
                .nodes
                .iter()
                .enumerate()
                .skip(1)
                .filter(|&(_, (c, _))| c.is_lowercase())
                .map(|(index, _)| {
                    // get full path to key
                    let path = edges.path(index);
                    let last = path.last().unwrap().clone();

                    // separate doors from keys
                    let (keys, intervening_keys): (Vec<_>, Vec<_>) = path
                        .into_iter()
                        .skip(1)
                        .map(|step| step.state)
                        .partition(|c| c.is_uppercase());

                    let keys = hash(keys);
                    let intervening_keys = hash(intervening_keys);

                    (
                        Key {
                            current: last.state,
                            preceding: keys,
                            intervening: intervening_keys,
                        },
                        last.cost
                    )
                })
                .collect();
            (key, edges)
        })
        .collect();

    // initial state for the main Dijkstra algorithm
    let start_state = State { location: robots, keys: [false; 26] };

    // continue Dijkstra until all keys have been collected
    let end_condition = | _: EdgeInfo<usize>, state: &State | {
        state.keys.iter().all(|&k| k)
    };

    let find_edges = | _: EdgeInfo<usize>, state: &State | -> Vec<(State, usize)> {
        let clumped: Vec<_> = state.location.iter().enumerate()
            .map(|(robot, location)| {
                key_edges.get(location).unwrap().iter()
                    .filter(|&&(key, _)| {
                        !state.keys[to_usize(key.current)] && key.preceding.iter().enumerate()
                            .filter(|&(_, k)| *k)
                            .all(|(i, _)| state.keys[i])
                    })
                    .map(|&(key, cost)| {
                        // updates robot locations
                        let mut new_location = state.location.clone();
                        new_location[robot] = key.current;

                        let mut new_keys = state.keys.clone();
                        for (i, _) in key.intervening.iter().enumerate()
                            .filter(|&(_, k)| *k)
                        {
                            new_keys[i] = true;
                        }
                        let new_state = State { location: new_location, keys: new_keys };
                        (new_state, cost)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        clumped.into_iter().flatten().collect()
    };
    let search = dijkstra(start_state, find_edges, end_condition);

    search.steps().unwrap()
}

fn to_usize(c: char) -> usize {
    if c.is_uppercase() {
        c as usize - 'A' as usize
    } else {
        c as usize - 'a' as usize
    }
}

fn hash(i: impl IntoIterator<Item = char>) -> CharSet {
    let mut cs = [false; 26];
    for c in i {
        cs[to_usize(c)] = true;
    }
    cs
}

fn part1(tunnels: &Input) -> Output {
    solve(tunnels, vec!['@'])
}

fn part2(tunnels: &Input) -> Output {
    let robots = vec!['@', '$', '%', '^'];
    let mut quadrants = tunnels.clone();
    let original_start = quadrants.iter().position(|&c| c == '@').unwrap();
    quadrants[original_start] = '#';
    let adj_indices: Vec<usize> = quadrants
        .adjacent(original_start, true)
        .unwrap()
        .map(|adj| adj.index)
        .collect();
    for &pos in adj_indices.iter().skip(1).step_by(2) {
        quadrants[pos] = '#';
    }
    for (index, &pos) in adj_indices.iter().step_by(2).enumerate() {
        quadrants[pos] = robots[index];
    }
    solve(&quadrants, robots)
}

#[test]
fn default() {
    let input = get_input(19, 18).unwrap();
    let input = parse_input(&input);
    assert_eq!(3918, part1(&input));
    assert_eq!(2004, part2(&input));
}

// Input parsed (60μs)
// 1. 3918 (15ms)
// 2. 2004 (30ms)
// Total: 46ms
use std::{cmp::{min, Reverse}, collections::BinaryHeap};

use advent::utilities::get_input::get_input;
use bit_set::BitSet;
use bit_vec::BitVec;
use lazy_regex::regex;
use rustc_hash::FxHashSet;
use utilities::structs::{indexer::Indexer, stopwatch::{ReportDuration, Stopwatch}};

type Int = usize;
type FlowMap = Vec<Int>;
type EdgeMap = Vec<Vec<Option<Int>>>;
type Input = (FlowMap, EdgeMap, Int);
type Output = usize;

#[derive(Debug, PartialEq, Eq, Ord, Hash)]
struct State {
    pos: [Option<(Int, i64)>; 2],
    valves: BitSet,
    flow: Int,
    total: i64,
    minute: i64,
}

impl PartialOrd for State {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}


fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 16).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    // use indexer to be able to use Vecs instead of HashMaps
    let mut indexer = Indexer::new();

    // assign a usize id to each valve
    for line in input.lines() {
        indexer.assign(&line[6..8]);
    }

    // parse to edgeMapNoValves and flowRate maps
    let rx = regex!(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)");
    let mut old_flow_map: FlowMap = vec![0; indexer.len()];
    let mut edge_map_no_valves: Vec<Vec<(Int, Int)>> = vec![Vec::new(); indexer.len()];

    for (_, [valve, rate, tunnels]) in rx.captures_iter(input).map(|c| c.extract()) {
        let valve = indexer.get_index(&valve).unwrap();
        old_flow_map[valve] = rate.parse().unwrap();
        let tunnels: Vec<(Int, Int)> = tunnels.split(", ")
            .map(|tunnel| {
                let tunnel: Int = indexer.get_index(&tunnel).unwrap();
                (tunnel, 1)
            })
            .collect();
        edge_map_no_valves[valve] = tunnels;
    }

    let aa = indexer.get_index(&"AA").unwrap();

    for valve in 0..edge_map_no_valves.len() {
        if valve == aa || old_flow_map[valve] > 0 {
            let start: (Int, Int) = (0, valve);
            let mut weights = vec![Int::MAX; indexer.len()];
            weights[valve] = 0;
            let mut q = BinaryHeap::new();
            q.push(Reverse(start));

            while let Some(Reverse((weight, id))) = q.pop() {
                for &(edge_id, edge_weight) in edge_map_no_valves[id].iter() {
                    let alternate_weight = weight + edge_weight;
                    if alternate_weight < weights[edge_id] {
                        weights[edge_id] = alternate_weight;
                        q.push(Reverse((alternate_weight, edge_id)));
                    }
                }
            }

            edge_map_no_valves[valve] = weights.into_iter().enumerate()
                .filter(|&(edge, weight)| {
                    edge != valve && edge != aa && old_flow_map[edge] > 0 && weight != Int::MAX 
                })
                .collect();
        }
    }

    let mut reindexer = Indexer::new();

    for (id, &rate) in old_flow_map.iter().enumerate() {
        if id == aa || rate > 0 {
            reindexer.assign(*indexer.get_value(id).unwrap());
        }
    }

    let nones: Vec<Option<Int>> = vec![None; reindexer.len()];
    let mut edge_map = vec![nones.clone(); reindexer.len()];

    for (old_id, old_edges) in edge_map_no_valves.into_iter().enumerate() {
        if old_id == aa || old_edges.len() > 2 {
            let new_id = get_new_id(old_id, &indexer, &reindexer);
            let new_edges = edge_map.get_mut(new_id).unwrap();
            for (old_edge, weight) in old_edges {
                let new_edge = get_new_id(old_edge, &indexer, &reindexer);
                (new_edges[new_edge] = Some(weight + 1));
            }
        }
    }

    let mut flow_map: FlowMap = vec![0; reindexer.len()];
    
    old_flow_map.into_iter().enumerate()
        .filter(|&(old_id, rate)| old_id == aa || rate > 0)
        .for_each(|(old_id, rate)| {
            let new_id = get_new_id(old_id, &indexer, &reindexer);
            flow_map[new_id] = rate;
        });

    let start = reindexer.get_index(&"AA").unwrap();

    (flow_map, edge_map, start)
}

fn get_new_id(old_id: usize, indexer: &Indexer<&str>, reindexer: &Indexer<&str>) -> usize {
    let og = indexer.get_value(old_id).unwrap();
    reindexer.get_index(og).unwrap()
}

fn solve(
    start: usize, 
    elephant_helper: bool, 
    minutes: Int, 
    flow_map: &FlowMap, 
    edge_map: &EdgeMap,
) -> Int {
    let mut max = 0;

    let mut valves = BitSet::from_bit_vec(BitVec::from_elem(flow_map.len(), true));
    valves.remove(start);

    let start = State {
        pos: [
            Some((start, 0)),
            if elephant_helper {
                Some((start, 0))
            } else {
                None
            }
        ],
        valves,
        flow: 0,
        total: 0,
        minute: minutes as i64 + 1,
    };
    let mut open = BinaryHeap::new();
    open.push((0usize, start));
    let mut closed = FxHashSet::default();

    while let Some((heuristic, state)) = open.pop() {
        if closed.contains(&state) { continue; }

        let potential_future = heuristic + 
            state.valves.iter()
                .map(|valve| {
                    let rate = flow_map[valve];
                    state.pos.iter()
                        .filter_map(|&room_info| {
                            let (room, time_offset) = room_info?;
                            let time = edge_map[room][valve]? as i64;
                            let distance = state.minute as i64 + time_offset - time;
                            if distance < 0 {
                                None
                            } else {
                                Some(distance as usize * rate)
                            }
                        })
                        .max()
                        .unwrap_or_default()
                })
                .sum::<usize>();
        if heuristic > max {
            max = heuristic;
        }
        if potential_future < max {
            continue;
        }

        let mut distances: Vec<Option<(Int, i64)>> = vec![None; flow_map.len()];
        for valve in state.valves.iter() {
            distances[valve] = state.pos.iter().enumerate()
                .filter_map(|(room_no, &room_info)| {
                    let (room, time_offset) = room_info?;
                    let time = edge_map[room][valve]? as i64;
                    let dist = time - time_offset;
                    Some((room_no, dist))
                })
                .min_by_key(|&(_, dist)| dist);
        }

        for (valve, room_info) in distances.into_iter().enumerate() {
            let Some((room_no, distance)) = room_info else { continue; };

            let new_pos = if elephant_helper {
                if room_no == 0 {
                    let new_first_offset = min(0, distance) as i64;
                    let new_second_offset = core::cmp::max(0, distance) as i64;
                    let second_room = state.pos[1].unwrap().0;
                    [Some((valve, new_first_offset)), Some((second_room, new_second_offset))]
                } else {
                    let new_second_offset = -(min(0, distance) as i64);
                    let new_first_offset = core::cmp::max(0, distance) as i64;
                    let first_room = state.pos[0].unwrap().0;
                    [Some((first_room, new_first_offset)), Some((valve, new_second_offset))]
                }
            } else {
                [Some((valve, 0)), None]
            };

            let mut new_valves = state.valves.clone();
            new_valves.remove(valve);
            let new_flow = state.flow + flow_map[valve];
            let new_total = state.total + state.flow as i64 * distance;
            let new_minute = min(state.minute - distance as i64, state.minute);

            let new_state = State {
                pos: new_pos,
                valves: new_valves,
                flow: new_flow,
                total: new_total,
                minute: new_minute,
            };

            let new_heuristic = (heuristic as i64 + flow_map[valve] as i64 * (new_minute as i64 - 1)) as usize;

            open.push((new_heuristic, new_state));
        }
        closed.insert(state);
    }
    max
}

fn part1((flow_map, edge_map, start): &Input) -> Output {
    solve(*start, false, 30, flow_map, edge_map)
}

fn part2((flow_map, edge_map, start): &Input) -> Output {
    solve(*start, true, 26, flow_map, edge_map)
}

#[test]
fn default() {
    let input = get_input(22, 16).unwrap();
    let input = parse_input(&input);
    assert_eq!(2059, part1(&input));
    assert_eq!(2790, part2(&input));
}

// Input parsed (1ms)
// 1. 2059 (4ms)
// 2. 2790 (270ms)
// Total: 276ms
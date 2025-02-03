use std::collections::{HashMap, HashSet, VecDeque};

use advent::utilities::get_input::get_input;
use indexmap::{IndexMap, IndexSet};
use utilities::structs::{stopwatch::{ReportDuration, Stopwatch}, str_grid::StrGrid};

type Output = usize;
type Vertices = IndexSet<usize>;
type VertexMap = HashMap<usize, usize>;
type State = (usize, usize);

#[derive(Debug)]
struct Input<'a> {
    trails: StrGrid<'a>,
    vertices: Vertices,
    vertex_map: VertexMap,
    start: usize,
    end: usize,
}

impl<'a> Input<'a> {
    fn new(s: &'a str) -> Self {
        let trails = StrGrid::new(s).unwrap();
        let start = s.find('.').unwrap();
        let end = trails.s.iter().enumerate().rev()
            .find(|(_, &c)| c == b'.')
            .unwrap()
            .0;
        let mut vertices = Vertices::new();

        vertices.insert(start);
        
        for (pos, &c) in trails.s.iter().enumerate() {
                if c != b'\n' && c != b'#' {
                    let neighbor_count = trails.adjacent(pos)
                        .filter(|neighbor| neighbor.b != b'#')
                        .count();
                    if neighbor_count >= 3 {
                        vertices.insert(pos);
                    }
                } 
        }

        vertices.insert(end);

        let vertex_map: VertexMap = vertices.iter().cloned().enumerate()
            .map(|(index, pos)| (pos, index))
            .collect();
        
        let start = vertex_map[&start];
        let end = vertex_map[&end];

        Self { trails, vertices, vertex_map, start, end }
    }
}

fn connect_vertex(pos: usize, vertices: &Vertices, can_go_uphill: bool, trails: &StrGrid) -> Vec<State> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut q: Vec<State> = Vec::new();
    q.push((pos, 0));
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    while let Some((current, dist)) = q.pop() {
        if !vertices.contains(&current) || current == pos {
            let these_neighbors: Vec<_> = if can_go_uphill {
                get_neighbors(current, trails)
            } else {
                match trails.get(current) {
                    Some(b'^') => [current - trails.width].into_iter().collect(),
                    Some(b'>') => [current + 1].into_iter().collect(),
                    Some(b'v') => [current + trails.width].into_iter().collect(),
                    Some(b'<') => [current - 1].into_iter().collect(),
                    _ => get_neighbors(current, trails),
                }
            };

            for neighbor in these_neighbors {
                if visited.contains(&neighbor) { continue; }
                let b = trails.get(neighbor).unwrap();
                if b != b'\n' && b != b'#' {
                    visited.insert(neighbor);
                    q.push((neighbor, dist + 1));
                }
            }
        } else {
            neighbors.push((current, dist));
        }
    }
    neighbors
}

fn get_neighbors(pos: usize, trails: &StrGrid<'_>) -> Vec<usize> {
    trails.adjacent(pos).into_iter()
        .map(|neighbor| neighbor.pos)
        .collect()
}


fn find_longest_trail(
    edge_map: &Vec<Vec<State>>, 
    pos: usize,
    weight: usize, 
    end: usize,
    visited: usize,
) -> usize {
    if pos == end {
        weight
    } else {
        let visited = visited + (1 << pos);
        edge_map[pos].iter()
            .filter(|(neighbor, _)| {
                // println!("{}", neighbor);
                (visited.clone() >> *neighbor) & 1 == 0
            })
            .map(|(neighbor, neighbor_weight)| {
                find_longest_trail(
                    edge_map,
                    neighbor.clone(),
                    weight + neighbor_weight,
                    end,
                    visited,
                )
            })
            .max()
            .unwrap_or_default()
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 23).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    Input::new(input)
}

fn part1(input: &Input) -> Output {
    let Input { trails, vertices, vertex_map, start, end } = input;
    let mut edges = Vec::new();
    for pos in vertices.iter() {
        let neighbors: Vec<(usize, usize)> = connect_vertex(pos.clone(), vertices, false, trails)
            .into_iter()
            .map(|(neighbor, dist)| (vertex_map[&neighbor], dist))
            .collect();
        edges.push(neighbors);
    }
    find_longest_trail(&edges, *start, 0, *end, 0)
}

fn part2(input: &Input) -> Output {
    let Input { trails, vertices, vertex_map, start, end } = input;
    let mut initial: IndexMap<usize, Vec<(usize, usize)>> = IndexMap::new();
    for &pos in vertices {
        let pp = vertex_map[&pos];
        for (neighbor, dist) in connect_vertex(pos, vertices, true, trails) {
            initial.entry(pp)
                .or_insert(Vec::new())
                .push((vertex_map[&neighbor], dist));
        }
    }

    // due to grid-like nature of the remaining nodes, the perimeter nodes (those with only three edges) are 
    // directional. Quick and dirty way of finding which directions to exclude is to run my standard BFS which
    // does flood fill of the nodes that don't have four edges, recording the parent node for each node. 
    // Then convert this to a map of node to parent node. Use this map to exclude certain edges in the edge map, 
    // thus making the perimeter nodes directional.
    // This works for everything except for the bottom corner node. This one fails because my standard BFS is
    // shortest-path, but the bottom corner node can be reached in two ways. So I handle the bottom corner 
    // individually.
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(*start);
    let mut verboten = VertexMap::new();
    while let Some(current) = q.pop_front() {
        let neighbors: Vec<usize> = initial[&current].iter()
            .filter(|&(neighbor, _)| !verboten.contains_key(neighbor) && initial[neighbor].len() != 4)
            .map(|(neighbor, _)| *neighbor)
            .collect();
        for neighbor in neighbors {
            if neighbor != 0 {
                verboten.insert(neighbor, current);
            }
            q.push_back(neighbor);
        }
    }

    let bottom_corner = initial.values().last().unwrap().first().unwrap().0;

    let edges: Vec<Vec<(usize, usize)>> = initial.into_iter()
        .map(|(pos, edges)| {
            edges.into_iter()
                .filter(|&(neighbor, _)| {
                    if pos == bottom_corner {
                        neighbor == *end
                    } else {
                        verboten.get(&pos) != Some(&neighbor)
                    }
                })
                .collect()
        })
        .collect();
    
    find_longest_trail(&edges, *start, 0, *end, 0)
}

#[test]
fn default() {
    let input = get_input(23, 23).unwrap();
    let input = parse_input(&input);
    assert_eq!(2210, part1(&input));
    assert_eq!(6522, part2(&input));
}

// Input parsed (113μs)
// 1. 2210 (873μs)
// 2. 6522 (38ms)
// Total: 39ms
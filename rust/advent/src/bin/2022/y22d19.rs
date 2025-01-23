use std::collections::VecDeque;

use advent::utilities::get_input::get_input;
use lazy_regex::regex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use utilities::structs::{indexer::Indexer, stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = (Vec<Blueprint>, Indexer<&'a str>);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 19).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    robot_costs: [[usize; 4]; 4],
    max_material: [usize; 3],
}

#[derive(Debug, Clone, Copy)]
struct State {
    minute: usize,
    resources: [usize; 4],
    robots: [usize; 4],
}

impl State {
    fn new() -> Self {
        State { minute: 0, resources: [0; 4], robots: [0; 4] }
    }
    
    fn max_bound(&self, minutes: usize, resource: usize) -> usize {
        let current_amount = self.resources[resource];
        let current_robot_num = self.robots[resource];
        let max_future: usize = (0..minutes - self.minute)
            .map(|minute| minute + current_robot_num)
            .sum();
        current_amount + max_future
    }
    
    fn min_bound(&self, minutes: usize, resource: usize) -> usize {
        let current_amount = self.resources[resource];
        let current_robot_num = self.robots[resource];
        current_amount + (minutes - self.minute) * current_robot_num
    }
    
    fn next_states(
        &self, 
        blueprint: &Blueprint, 
        minutes: usize, 
        cutoff: &Vec<usize>
    ) -> Vec<State> {
        (0..4)
            .filter_map(|robot_type| {
                if Some(&self.robots[robot_type]) == blueprint.max_material.get(robot_type) { 
                    return None; 
                }
                let remaining_minutes = minutes.checked_sub(self.minute)?;
                remaining_minutes.checked_sub(cutoff[robot_type])?;
                let build_time: usize = self.build_time(blueprint, robot_type);
                remaining_minutes.checked_sub(build_time)?;

                let mut new_robots = self.robots;
                new_robots[robot_type] += 1;

                let mut new_resources = self.resources;
                let costs = blueprint.robot_costs[robot_type];

                for (component, cost) in new_resources.iter_mut().enumerate() {
                    *cost = *cost + self.robots[component] * build_time - costs[component];
                }
                
                Some(State {
                    minute: self.minute + build_time,
                    resources: new_resources,
                    robots: new_robots,
                })
            })
            .collect()
    }
    
    fn build_time(&self, blueprint: &Blueprint, robot_type: usize) -> usize {
        blueprint.robot_costs[robot_type].iter().enumerate()
            .map(|(component, &cost)| {
                let resources_available = self.resources[component];
                if cost <= resources_available {
                    1
                } else {
                    let robots_available = self.robots[component];
                    if robots_available == 0 {
                        usize::MAX
                    } else {
                        ((cost - resources_available) as f64 
                            / robots_available as f64).ceil() as usize + 1
                    }
                }
            })
            .max()
            .unwrap()
    }
}

fn parse_input(input: &str) -> Input {
    let rx = regex!(r"Each ([a-z]+) robot costs (\d+) ([a-z]+)(?: and (\d+) ([a-z]+))?. ?");
    let mut indexer = Indexer::new();
    let blueprints = input.lines().enumerate()
        .map(|(idx, spec)| {
            let mut robot_costs = [[0; 4]; 4];
            for caps in rx.captures_iter(spec) {
                let robot = caps.get(1).unwrap().as_str();
                let robot = indexer.get_or_assign_index(robot);
                let robot = robot_costs.get_mut(robot).unwrap();

                let cost1: usize = caps.get(2).unwrap().as_str().parse().unwrap();
                let resource1 = caps.get(3).unwrap().as_str();
                let resource1 = indexer.get_or_assign_index(resource1);

                robot[resource1] = cost1;
                
                if let Some(cost2) = caps.get(4) {
                    let cost2: usize = cost2.as_str().parse().unwrap();
                    let resource2 = caps.get(5).unwrap().as_str();
                    let resource2 = indexer.get_or_assign_index(resource2);

                    robot[resource2] = cost2;
                }
            }
            let max_material: [usize; 3] = std::array::from_fn(|resource| {
                robot_costs.iter()
                .map(|resources| resources[resource])
                .max()
                .unwrap_or_default()
            });
            Blueprint { id: idx + 1, robot_costs, max_material }
        })
        .collect();
    (blueprints, indexer)
}

fn find_resource(
    blueprint: &Blueprint, 
    resource: usize, 
    initial_state: State, 
    minutes: usize
) -> Output {
    let cutoff: Vec<usize> = (0..4)
        .map(|robot_type| {
            if robot_type == resource {
                1
            } else if blueprint.robot_costs[resource][robot_type] > 0 {
                3
            } else {
                5
            }
        })
        .collect();
    let mut max_geodes = 0;
    let mut q = VecDeque::new();
    q.push_back(initial_state);
    while let Some(state) = q.pop_front() {
        if state.max_bound(minutes, resource) < max_geodes { continue; }
        let min_geodes = state.min_bound(minutes, resource);
        if min_geodes > max_geodes {
            max_geodes = min_geodes;
        }
        for next in state.next_states(blueprint, minutes, &cutoff) {
            q.push_back(next);
        }
    }
    max_geodes
}

fn part1((blueprints, indexer): &Input) -> Output {
    let mut initial_state = State::new();
    initial_state.robots[indexer.get_index(&"ore").unwrap()] = 1;
    blueprints.par_iter()
        .map(|blueprint| {
            blueprint.id * find_resource(
                blueprint, 
                indexer.get_index(&"geode").unwrap(), 
                initial_state, 
                24
            )
        })
        .sum()
}

fn part2((blueprints, indexer): &Input) -> Output {
    let mut initial_state = State::new();
    initial_state.robots[indexer.get_index(&"ore").unwrap()] = 1;
    blueprints[..3].par_iter()
        .map(|blueprint| {
            find_resource(
                blueprint, 
                indexer.get_index(&"geode").unwrap(), 
                initial_state, 
                32
            )
        })
        .reduce(|| 1, |acc, minutes| acc * minutes)
}

#[test]
fn default() {
    let input = get_input(22, 19).unwrap();
    let input = parse_input(&input);
    assert_eq!(1427, part1(&input));
    assert_eq!(4400, part2(&input));
}

// Input parsed (429μs)
// 1. 1427 (2ms)
// 2. 4400 (24ms)
// Total: 27ms
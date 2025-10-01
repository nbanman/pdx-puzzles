use advent::utilities::get_input::get_input;
use indexmap::IndexSet;
use std::{cmp::Reverse, collections::BinaryHeap};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = (Steps, Steps, Vec<usize>);
type Steps = [Vec<usize>; 26];

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 7).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut next_steps: [Vec<usize>; 26] = std::array::from_fn(|_| Vec::new());
    let mut preceding_steps: [Vec<usize>; 26] = std::array::from_fn(|_| Vec::new());
    for line in input.lines() {
        let line = line.as_bytes();
        next_steps[line[5] as usize - 65].push(line[36] as usize - 65);
        preceding_steps[line[36] as usize - 65].push(line[5] as usize - 65);
    }
    let starts = (0..26)
        .filter(|&i| preceding_steps[i].is_empty() && !next_steps.is_empty())
        .collect();
    (next_steps, preceding_steps, starts)
}

#[derive(Clone, Copy, Debug)]
enum WorkerStatus {
    Idle,
    Working(usize),
}

#[derive(Clone, Copy, Debug)]
struct Worker {
    status: WorkerStatus,
    ready: usize,
}

impl Worker {
    fn is_finished(&self, t: usize) -> bool {
        matches!(self.status, WorkerStatus::Working(_)) && self.ready == t
    }

    fn assign(&mut self, step: usize, t: usize, offset: usize) {
        self.status = WorkerStatus::Working(step);
        self.ready = t + offset + step + 1;
    }
}

fn solve(input: &Input, number_of_workers: usize, time_offset: usize) -> (String, usize) {
    let (next_steps, preceding_steps, starts) = input;
    let mut queue: BinaryHeap<Reverse<usize>> =
        starts.iter().map(|&start| Reverse(start)).collect();

    // tracks the letters that have been delivered
    let mut steps = IndexSet::with_capacity(26);

    // represents all the workers available to perform  steps. They begin idle.
    let idle_worker = Worker {
        status: WorkerStatus::Idle,
        ready: 0,
    };
    let mut worker_pool = vec![idle_worker; number_of_workers];

    // This sequence starts at second 0 and keeps adding one second.
    // Each second, it harvests completed letters from workers, adding them to the steps
    // It then assigns available letters to idle workers.
    // It terminates when steps contains all the letters, returning the # of seconds.
    for t in 0.. {
        for worker in worker_pool.iter_mut().filter(|w| w.is_finished(t)) {
            let WorkerStatus::Working(product) = worker.status else {
                panic!("Finished worker has no deliverable");
            };
            steps.insert(product);
            worker.status = WorkerStatus::Idle;

            // take the steps that are potentially available now that the worker has completed the step
            // check that all other preceding steps have already been added, then add to queue
            if let Some(next) = next_steps.get(product) {
                for &next_step in next {
                    if preceding_steps[next_step]
                        .iter()
                        .all(|&it| steps.contains(&it))
                    {
                        queue.push(Reverse(next_step));
                    }
                }
            }
        }

        for worker in worker_pool.iter_mut() {
            if matches!(worker.status, WorkerStatus::Idle) {
                if let Some(Reverse(step)) = queue.pop() {
                    worker.assign(step, t, time_offset);
                } else {
                    break;
                }
            }
        }

        if steps.len() == 26 {
            let letters: String = steps
                .iter()
                .map(|&step| (step as u8 + 65) as char)
                .collect();

            return (letters, t);
        }
    }
    unreachable!()
}

fn part1(input: &Input) -> String {
    solve(input, 1, 0).0
}

fn part2(input: &Input) -> usize {
    solve(input, 5, 60).1
}

#[test]
fn default() {
    let input = get_input(18, 7).unwrap();
    let input = parse_input(&input);
    assert_eq!("ABGKCMVWYDEHFOPQUILSTNZRJX".to_string(), part1(&input));
    assert_eq!(898, part2(&input));
}

// Input parsed (19μs)
// 1. ABGKCMVWYDEHFOPQUILSTNZRJX (31μs)
// 2. 898 (17μs)
// Total: 69μs

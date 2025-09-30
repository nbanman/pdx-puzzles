use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{grid::Grid2, stopwatch::{ReportDuration, Stopwatch}}};

type Input = FxHashMap<usize, Grid2<bool>>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 4).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut guards = FxHashMap::default();

    // Parse input to list of log entries sorted by dateTime
    let log: Vec<((usize, usize, usize, usize, usize), Option<usize>)> = input.lines()
        .map(|line| {
            let (timestamp, entry) = line.split_once("] ").unwrap();
            let date: (usize, usize, usize, usize, usize) = timestamp.get_numbers().collect_tuple().unwrap();
            let id = entry.get_numbers().next();
            (date, id)
        })
        .sorted_unstable_by_key(|(a, _)| *a)
        .collect();
    
    let mut index = 0;

    // while loop b/c index can be advanced more than once per iteration
    while index < log.len() {
        // get id of guard on duty and set various variables to track the sleep log of the particular day
        let (_, id) = log[index];
        let id = id.unwrap();
        let mut hour = vec![false; 60];
        let mut minute = 0;
        let mut asleep = false;
        index += 1;

        // nested while loop populates the hour array with sleep info
        loop {
            let ((_, _, _, _, next_minute), next_id) = log[index];
            if next_id.is_some() { break; }
            for min in minute..next_minute {
                hour[min] = asleep;
            }
            minute = next_minute;
            asleep = !asleep;
            index += 1;
            if index == log.len() { break; }
        }

        // fills out the rest of the hour array
        for min in minute..60 {
            hour[min] = asleep;
        }

        // adds the hour to the map
        guards.entry(id).or_insert(Vec::new()).push(hour);
    }
    guards.into_iter()
        .map(|(id, timetable)| (id, timetable.try_into().unwrap()))
        .collect()
}

fn part1(guards: &Input) -> Output {
    guards.iter()
        .max_by_key(|(_, days)| {
            days.iter().filter(|&&it| it).count()
        })
        .map(|(&id, day)| {
            // finds the minute where the guard is most often asleep
            let most_asleep = day.columns().enumerate()
                .max_by_key(|(_, record)| {
                    record.iter().filter(|&&&it| it).count()
                })
                .map(|(min, _)| min)
                .unwrap();

            // returns the id * the minute
            id * most_asleep
        })
        .unwrap()
}

fn part2(guards: &Input) -> Output {
    guards.iter()
        // for each guard...
        .map(|(&id, days)| {
            // ...make a table of each minute and how many times asleep...
            days.columns().enumerate()
                .map(|(minute, record)| {
                    (minute, record.iter().filter(|&&&it| it).count())
                })
                // ...then grab the minute that guard was most often asleep...
                .max_by_key(|(_, asleep)| *asleep)
                // ...and package it all together
                .map(|(min, asleep)| (id, min, asleep))
                .unwrap()
        })
        // find guard who had the minute that was most asleep
        .max_by_key(|(_, _, asleep)| *asleep)
        // multiply that minute by the guard's ID
        .map(|(id, min, _)| id * min)
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(18, 4).unwrap();
    let input = parse_input(&input);
    assert_eq!(19025, part1(&input));
    assert_eq!(23776, part2(&input));
}

// Input parsed (328μs)
// 1. 19025 (10μs)
// 2. 23776 (35μs)
// Total: 376μs

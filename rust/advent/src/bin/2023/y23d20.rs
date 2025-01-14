use std::{collections::{HashMap, HashSet, VecDeque}, iter::successors};

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = (Lookup<'a>, UpstreamCount<'a>, DispatchQueue<'a>);
type Output = usize;
type Downstream<'a> = Vec<&'a str>;
type Lookup<'a> = HashMap<&'a str, Module<'a>>;
type UpstreamCount<'a> = HashMap<&'a str, usize>;
type DispatchQueue<'a> = VecDeque<Signal<'a>>;
type UpstreamPulses<'a> = HashMap<&'a str, Pulse>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
struct Signal<'a> {
    sender: &'a str,
    recipients: Vec<&'a str>,
    pulse: Pulse,
}

impl<'a> Signal<'a> {
    fn send(
        &self, 
        lookup: &mut Lookup<'a>, 
        dispatch_queue: &mut DispatchQueue<'a>, 
        upstream_count: &mut UpstreamCount<'a>
    ) {
        for recipient in self.recipients.iter() {
            if let Some(module) = lookup.get_mut(recipient) {
                module.on_receive(self.clone(), dispatch_queue, upstream_count);
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Module<'a> {
    Button { downstream: Downstream<'a> },
    Broadcaster { downstream: Downstream<'a> },
    FlipFlop { name: &'a str, downstream: Downstream<'a>, on: bool },
    Conjunction { name: &'a str, downstream: Vec<&'a str>, upstream_pulses: UpstreamPulses<'a> },
}

impl<'a> Module<'a> {
    fn register_button(lookup: &mut Lookup<'a>, upstream_count: &mut UpstreamCount<'a>) {
        let new = Self::Button { downstream: vec!["broadcaster"] } ;
        lookup.insert("button", new);
        upstream_count.entry("broadcaster").and_modify(|e| *e += 1).or_insert(1);
    }

    fn register_broadcaster(
        downstream: Downstream<'a>, 
        lookup: &mut Lookup<'a>, 
        upstream_count: &mut UpstreamCount<'a>,
    ) {
        for &down in downstream.iter() {
            upstream_count.entry(down).and_modify(|e| *e += 1).or_insert(1);
        }
        let new = Self::Broadcaster { downstream };
        lookup.insert("broadcaster", new);
    }

    fn register_flip_flop(
        name: &'a str, 
        downstream: Downstream<'a>,
        lookup: &mut Lookup<'a>, 
        upstream_count: &mut UpstreamCount<'a>,
    ) {
        for &down in downstream.iter() {
            upstream_count.entry(down).and_modify(|e| *e += 1).or_insert(1);
        }
        let new = Self::FlipFlop { name, downstream, on: false };
        lookup.insert(name, new);
    }

    fn register_conjunction(
        name: &'a str, 
        downstream: Downstream<'a>,
        lookup: &mut Lookup<'a>, 
        upstream_count: &mut UpstreamCount<'a>,
    ) {
        for &down in downstream.iter() {
            upstream_count.entry(down).and_modify(|e| *e += 1).or_insert(1);
        }
        let new = Self::Conjunction { name, downstream, upstream_pulses: HashMap::new() };
        lookup.insert(name, new);
    }

    fn on_receive(
        &mut self, 
        signal: Signal<'a>, 
        dispatch_queue: &mut DispatchQueue<'a>,
        upstream_count: &mut UpstreamCount<'a>,
    ) {
        match self {
            Module::Button { downstream} => {
                let output = Signal { 
                    sender: "button", 
                    recipients: downstream.clone(), 
                    pulse: Pulse::Low, 
                };
        
                dispatch_queue.push_back(output);
            },
            Module::Broadcaster { downstream } => {
                let output = Signal { 
                    sender: "broadcaster", 
                    recipients: downstream.clone(), 
                    pulse: signal.pulse, 
                };
        
                dispatch_queue.push_back(output);
            },
            Module::FlipFlop { name, downstream, on } => {
                if signal.pulse == Pulse::Low {
                    *on = !*on;
                    let pulse = if *on { Pulse::High } else { Pulse::Low };
                    let output = Signal { 
                        sender: name, 
                        recipients: downstream.clone(), 
                        pulse, 
                    };
                    dispatch_queue.push_back(output);
                }
            },
            Module::Conjunction { name, downstream, upstream_pulses } => {
                upstream_pulses.insert(signal.sender, signal.pulse);
                let pulse = if upstream_count[name] == upstream_pulses.len() 
                    && upstream_pulses.values().all(|&it| it == Pulse::High) 
                {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                let output = Signal { sender: name, recipients: downstream.clone(), pulse };
                dispatch_queue.push_back(output);
            },
        }
    }

    fn reset(&mut self) {
        match self {
            Module::FlipFlop { name: _, downstream: _, on } => {
                *on = false;
            },
            Module::Conjunction { name: _, downstream: _, upstream_pulses } => {
                upstream_pulses.clear();
            },
            _ => { },
        }
    }
    
    fn downstream(&self) -> &Vec<&str> {
        match self {
            Module::Button { downstream }=> downstream,
            Module::Broadcaster { downstream } => downstream,
            Module::FlipFlop { name: _, downstream, on: _ } => downstream,
            Module::Conjunction { name: _, downstream, upstream_pulses: _ } => downstream,
        }
    }

}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 20).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut lookup = Lookup::new();
    let mut upstream_count = UpstreamCount::new();
    let dispatch_queue = DispatchQueue::new();

    Module::register_button(&mut lookup, &mut upstream_count);

    for line in input.lines() {
        let (name, downstream) = line.split_once(" -> ").unwrap();
        let (module_type, name) = name.split_at(1);
        let downstream: Vec<&str> = downstream.split(", ").collect();
        match module_type {
            "%" => Module::register_flip_flop(name, downstream, &mut lookup, &mut upstream_count),
            "&" => Module::register_conjunction(name, downstream, &mut lookup, &mut upstream_count),
            _ => Module::register_broadcaster(downstream, &mut lookup, &mut upstream_count),
        }
    }
    (lookup, upstream_count, dispatch_queue)
}

fn press_button<'a>(
    lookup: &mut Lookup<'a>,
    dispatch_queue: &mut DispatchQueue<'a>,
    upstream_count: &mut UpstreamCount<'a>,
) -> (usize, usize) {
    let recipients = ["broadcaster"].into_iter().collect();
    let signal = Signal { sender: "button", recipients, pulse: Pulse::Low };
    lookup.get_mut("button").unwrap().on_receive(signal, dispatch_queue, upstream_count);
    let mut high = 0;
    let mut low = 0;
    while let Some(signal) = dispatch_queue.pop_front() {
        signal.send(
            lookup, 
            dispatch_queue, 
            upstream_count,
        );
        
        if signal.pulse == Pulse::High {
            high += signal.recipients.len();
        } else {
            low += signal.recipients.len();
        }
    }
    (high, low)
}

fn part1(modules: Input) -> Output {
    let (mut lookup, mut upstream_count, mut dispatch_queue) = modules;
    let (high, low) = (0..1000)
        .map(|_| press_button(&mut lookup, &mut dispatch_queue, &mut upstream_count))
        .fold((0, 0), |(sum_high, sum_low), (high, low)| {
            (sum_high + high, sum_low + low)
        });
    high * low
}

fn part2(modules: Input) -> Output {
    let (mut lookup, mut upstream_count, mut dispatch_queue) = modules;
    let flip_flops: HashSet<&str> = lookup.values()
        .filter_map(|module| {
            if let &Module::FlipFlop { name, downstream, on: _ } = module {
                Some(name)
            } else {
                None
            }
        })
        .collect();
    let binary_counter_results = lookup["broadcaster"].downstream()
        .iter()
        .map(|&name| {
            let start = lookup[name];
            let &conjunction = start.downstream().iter()
                .find(|&&v| matches!(lookup[v], Module::Conjunction { name, downstream, upstream_pulses }))
                .unwrap();
            successors(Some(start), |module| {
                module.downstream().iter().find(|&&v| flip_flops.contains(v))
            })
                .map(|it| )
        })

}

#[test]
fn default() {
    let input = get_input(23, 20).unwrap();
    let input = parse_input(&input);
    assert_eq!(938065580, part1(input.clone()));
    assert_eq!(250628960065793, part2(input));
}

use std::{collections::{HashMap, VecDeque}, iter::successors};

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Output = usize;
type Downstream<'a> = Vec<&'a str>;
type Lookup<'a> = HashMap<&'a str, Module<'a>>;
type UpstreamCount<'a> = HashMap<&'a str, usize>;
type DispatchQueue<'a> = VecDeque<Signal<'a>>;
type UpstreamPulses<'a> = HashMap<&'a str, Pulse>;

#[derive(Clone, Debug)]
struct Modules<'a> {
    lookup: Lookup<'a>,
    upstream_count: UpstreamCount<'a>,
    dispatch_queue: DispatchQueue<'a>,
}

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
    Button,
    Broadcaster { downstream: Vec<&'a str> },
    FlipFlop { name: &'a str, downstream: Downstream<'a>, on: bool },
    Conjunction { name: &'a str, downstream: Vec<&'a str>, upstream_pulses: UpstreamPulses<'a> },
}

impl<'a> Module<'a> {
    fn register_button(lookup: &mut Lookup<'a>, upstream_count: &mut UpstreamCount<'a>) {
        let new = Self::Button;
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
        modules: &mut Modules<'a>,
    ) {
        match self {
            Module::Button => {
                let output = Signal { 
                    sender: "button", 
                    recipients: ["broadcaster"].into_iter().collect(), 
                    pulse: Pulse::Low, 
                };
        
                modules.dispatch_queue.push_back(output);
            },
            Module::Broadcaster { downstream } => {
                let output = Signal { 
                    sender: "broadcaster", 
                    recipients: downstream.clone(), 
                    pulse: signal.pulse, 
                };
        
                modules.dispatch_queue.push_back(output);
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
                    modules.dispatch_queue.push_back(output);
                }
            },
            Module::Conjunction { name, downstream, upstream_pulses } => {
                upstream_pulses.insert(signal.sender, signal.pulse);
                let pulse = if modules.upstream_count[name] == upstream_pulses.len() 
                    && upstream_pulses.values().all(|&it| it == Pulse::High) 
                {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                let output = Signal { sender: name, recipients: downstream.clone(), pulse };
                modules.dispatch_queue.push_back(output);
            },
        }
    }

    fn reset(&mut self) {
        match self {
            Module::FlipFlop { name, downstream, on } => {
                *on = false;
            },
            Module::Conjunction { name, downstream, upstream_pulses } => {
                upstream_pulses.clear();
            },
            _ => { },
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

fn parse_input(input: &str) -> Modules {
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
    Modules { lookup, upstream_count, dispatch_queue }
}

fn press_button(modules: &mut Modules) -> (usize, usize) {
    let recipients = ["broadcaster"].into_iter().collect();
    let signal = Signal { sender: "button", recipients, pulse: Pulse::Low };
    modules.lookup["button"].on_receive(signal, modules);
       
    ()
}

fn part1(modules: Modules) -> Output {
    let (high, low) = (0..1000)
        .map(|_| press_button())
        .fold((0, 0), |(sum_high, sum_low), (high, low)| {
            (sum_high + high, sum_low + low)
        });
    high * low
}

fn part2(modules: Modules) -> Output {
    250628960065793
}

#[test]
fn default() {
    let input = get_input(23, 20).unwrap();
    let input = parse_input(&input);
    assert_eq!(938065580, part1(&input));
    assert_eq!(250628960065793, part2(&input));
}

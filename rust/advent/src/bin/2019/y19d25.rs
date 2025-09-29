use advent::utilities::{get_input::get_input, intcode::IntCode};
use lazy_regex::regex;
use utilities::parsing::get_numbers::ContainsNumbers;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 25).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: &str) -> usize {
    let mut ic: IntCode = input.into();

    // Initial Hull breach information needed to start the second pathfinding, so run the program and save
    // a copy of the output.
    let (_, initial_hull_breach) = ic.run_while_able();

    // Run a DFS that traverses entire map, picking up all items, savescumming to avoid fatal items.
    explore(
        false,
        String::new(),
        "Outer Space".to_string(),
        Some(initial_hull_breach.clone()),
        &mut ic,
        0,
    );


    // Second DFS to get to security. Reset the output for the second traversal, now that all items
    // have been picked up.
    let plate_direction = explore(
        true,
        String::new(),
        "Outer Space".to_string(),
        Some(initial_hull_breach),
        &mut ic,
        0,
    );

    // get list of all items...
    let inventory = execute("inv", &mut ic);

    // ...and parse them
    let inventory: Vec<String> = output_to_string(&inventory)
        .lines()
        .filter(|line| !line.is_empty() && line.starts_with('-'))
        .map(|line| line.chars().skip(2).collect::<String>())
        .collect();

    // drop all items
    for item in inventory.iter() {
        let command = format!("drop {}", item);
        execute(&command, &mut ic);
    }

    // DFS tries all combinations until the answer is provided
    match get_passcode(&inventory, &plate_direction, 0, &mut ic) {
        ScaleReport::Passcode(v) => v,
        _ => unreachable!()
    }
}

fn get_passcode(
    inventory: &[String],
    direction: &str,
    index: usize,
    ic: &mut IntCode
) -> ScaleReport {
    // step on plate and get report
    let step = step(&direction, ic);

    // if "heavy" or the answer, return it. otherwise continue
    if !matches!(step, ScaleReport::Light) { return step; }

    // go through all items in the list and pick them up, then call the function recursively. if the result
    // of the recursive call is too heavy, drop that item. If the result of the recursive all is the answer,
    // return it.
    for i in index..inventory.len() {
        let command = format!("take {}", inventory[i]);
        execute(&command, ic);
        match get_passcode(&inventory, direction, i + 1, ic) {
            ScaleReport::Heavy => {
                let command = format!("drop {}", inventory[i]);
                execute(&command, ic);
            },
            ScaleReport::Passcode(v) => { return ScaleReport::Passcode(v) },
            ScaleReport::Light => {},
        }
    }
    ScaleReport::Heavy
}

enum ScaleReport {
    Heavy,
    Light,
    Passcode(usize),
}

fn step(direction: &str, ic: &mut IntCode) -> ScaleReport {
    ic.input_ascii(direction);
    ic.input(10);
    let (_, output) = ic.run_while_able();
    let report = output_to_string(&output);
    if report.contains("lighter") {
        ScaleReport::Heavy
    } else if report.contains("heavier") {
        ScaleReport::Light
    } else {
        ScaleReport::Passcode(report.get_numbers().next().unwrap())
    }
}

fn explore(
    stop_at_security: bool,
    command: String,
    previous_location: String,
    previous_output: Option<Vec<i64>>,
    ic: &mut IntCode,
    inception: usize,
) -> String {
    let inception = inception + 1;
    let mut output = execute(&command, ic);
    if let Some(mut previous_output) = previous_output {
        previous_output.extend_from_slice(&output);
        output = previous_output;
    }
    let (current_location, doors, items) = parse(&output);
    
    // returns early if hits pressure plate
    if previous_location == current_location {
        return if stop_at_security { command } else { String::new() };
    }

    // picks up items, undos action if fatal
    if !stop_at_security {
        for item in items {
            let save = ic.clone();
            let command = format!("take {item}");
            let mut output = execute(&command, ic);
            output.extend(execute("", ic));
            let output = output_to_string(&output);
            if !output.contains("Unrecognized") {
                ic.restore(save);
            }
        }
    }

    // moves to next spot
    for door in doors.into_iter().filter(|door| door != reverse(&command)) {
        let end_command = explore(
            stop_at_security,
            door,
            current_location.clone(),
            None,
            ic,
            inception
        );
        if !end_command.is_empty() {
            return end_command;
        }
    }

    // moves back out
    if !command.is_empty() {
        execute(reverse(&command), ic);
    }
    String::new()
}

fn reverse(direction: &str) -> &'static str {
    match direction {
        "north" => "south",
        "south" => "north",
        "east" => "west",
        "west" => "east",
        _ => "none",
    }
}

fn execute(command: &str, ic: &mut IntCode) -> Vec<i64> {
    ic.input_ascii(&command);
    ic.input(10);
    let (_, output) = ic.run_while_able_protected(300);
    output
}

fn output_to_string(output: &[i64]) -> String {
    output.iter().map(|&i| i as u8 as char).collect()
}

fn parse(output: &[i64]) -> (String, Vec<String>, Vec<String>) {
    let output = output_to_string(output);

    let location_rx: &lazy_regex::Lazy<regex::Regex> = regex!(r"== ((?:\w+ ?)+) ==");
    let doors_rx = regex!(r"Doors here lead:\n((?:- \w+\n)+)");
    let itemized_rx = regex!(r"Items here:\n((?:- [a-z ]+\n)+)");
    let split_rx = regex!(r"\w+(?: \w+)?");

    let get_group = |rx: &lazy_regex::Lazy<regex::Regex>| {
        rx.captures(&output)
            .map(|cap| cap.get(1).unwrap().as_str().to_string())
            .unwrap_or_default()
    };
    let get_itemized = |s: &str| {
        split_rx.find_iter(s).map(|cap| cap.as_str().to_string()).collect::<Vec<_>>()
    };

    let location = get_group(location_rx);
    let doors = get_group(doors_rx);
    let doors = get_itemized(&doors);
    let items = get_group(itemized_rx);
    let items: Vec<String> = get_itemized(&items);

    (location, doors, items)
}

#[test]
fn default() {
    let input = get_input(19, 25).unwrap();
    assert_eq!(16810049, part1(&input));
}

// Input parsed (30μs)
// 1. 16810049 (12ms)
// Total: 12ms
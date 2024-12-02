use std::iter::successors;

use everybody_codes::utilities::inputs::get_inputs;
use itertools::Itertools;
use utilities::{enums::cardinals::Cardinal, parsing::try_get::TryGet, structs::{coord::Coord, stopwatch::{ReportDuration, Stopwatch}}};

type Pos = Coord<isize, 2>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_inputs(24, 7);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: &str) -> String {
    let knights = parse_knights(input);
    let race = TRACK1.as_bytes().iter()
        .map(power_of);
    knights.into_iter()
        .map(|(knight, plan)| {
            (knight, get_power(&plan, race.clone()))
        })
        .sorted_unstable_by_key(|(_, power)| *power)
        .rev()
        .map(|(knight, _)| knight)
        .join("")
}

fn part2(input: &str) -> String {
    let knights = parse_knights(input);
    let track = parse_track(TRACK2);
    let race = get_race(&track, 10);
    knights.into_iter()
        .map(|(knight, plan)| (knight, get_power(&plan, race.clone())))
        .sorted_unstable_by_key(|(_, power)| *power)
        .rev()
        .map(|(knight, _)| knight)
        .join("")
}

fn part3(input: &str) -> usize {
    let (_, opponent_plan) = parse_knights(input)[0].clone();
    let track = parse_track(TRACK3);
    // I was too lazy to do an lcm function, but the lcm is just 11 * track size, so 11 laps
    // is what it takes to cycle. It would work anyway even if there was a lower lcm; it 
    // would just not be as efficient.
    let laps = 11;
    let race = get_race(&track, laps);
    let opponent_power = get_power(&opponent_plan, race.clone());
    let plans: Vec<Vec<isize>> = get_plans();
    plans.iter()
        .filter(|&plan| {
            get_power(plan, race.clone()) > opponent_power
        })
        .count()
}

fn get_plans() -> Vec<Vec<isize>> {
    let mut permutations = Vec::with_capacity(9240);
    let mut working = Vec::with_capacity(11);
    let mut store = [3, 3, 5];
    traverse(&mut permutations, &mut working, &mut store);
    permutations
}

fn traverse(
    permutations: &mut Vec<Vec<isize>>,
    working: &mut Vec<isize>,
    store: &mut [usize; 3],
) {
    for value in 0..=2 {
        if store[value] > 0 {
            working.push(value as isize - 1);
            store[value] -= 1;
            if working.len() == 11 {
                permutations.push(working.clone());
            } else {
                traverse(permutations, working, store);
            }
            working.pop();
            store[value] += 1;
        }
    }
}

fn parse_knights(input: &str) -> Vec<(&str, Vec<isize>)> {
    input.lines()
        .map(|line| {
            let (knight, power) = line.split_once(':').unwrap();
            let power = power.split(',')
                .map(|c| power_of(&c.as_bytes()[0]))
                .collect();
            (knight, power)
        })
        .collect()
}

fn parse_track(track: &'static str) -> Vec<isize> {
    let width = track.find('\n').unwrap();
    let track: Vec<Vec<u8>> = track.lines()
        .map(|line| {
            format!("{:<width$}", line, width = width)
                .as_bytes()
                .to_vec()
        })
        .collect();
    let turns = [Cardinal::straight, Cardinal::left, Cardinal::right];

    let go = 
        |(pos, dir): (Pos, Cardinal)| -> Option<(Pos, Cardinal)>
    {
        turns.iter()
            .filter_map(|turn| {
                let new_dir = turn(&dir);
                let new_pos = pos.move_direction(new_dir, 1)?;
                let (x, y) = new_pos.destructured();
                let row = track.try_get(y)?;
                let &cell = row.try_get(x)?;
                if cell != b' ' {
                    Some((new_pos, new_dir))
                } else {
                    None
                }
            })
            .next()
    };

    successors(Some((Pos::new2d(1, 0), Cardinal::East)), |&prev| go(prev))
        .take_while_inclusive(|&(pos, _)| {
            let (x, y) = pos.destructured();
            track[y as usize][x as usize] != b'S'
        })
        .map(|(pos, _)| {
            let (x, y) = pos.destructured();
            power_of(&track[y as usize][x as usize])
        })
        .collect()
}

fn get_race(track: &[isize], laps: usize) -> impl Iterator<Item = isize> + '_ + Clone {
    track.iter().copied().cycle().take(laps * track.len())
}

fn get_power(plan: &[isize], race: impl Iterator<Item = isize>) -> isize {
        let plan = plan.iter().copied().cycle();
        race.zip(plan)
            .scan(10isize, |state, (track, device)| {
                let adjust = if track == 0 { device } else { track };
                *state += adjust;
                Some(*state)
            })
            .sum()
}

fn power_of(byte: &u8) -> isize {
    match &byte {
        b'+' => 1,
        b'-' => -1,
        _ => 0,
    }
}



const TRACK1: &str = "==========";
const TRACK2: &str = r"S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--
-                                                                     -
=                                                                     =
+                                                                     +
=                                                                     +
+                                                                     =
=                                                                     =
-                                                                     -
--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-";
const TRACK3: &str = r"S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-";
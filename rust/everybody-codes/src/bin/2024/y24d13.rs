use std::{cmp::{min, Reverse}, collections::{BinaryHeap, HashSet}};

use everybody_codes::utilities::inputs::get_inputs;
use utilities::{enums::cardinals::Cardinal, structs::stopwatch::{ReportDuration, Stopwatch}};

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_inputs(24, 13);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!("1. {} ({})", solve(&input1), stopwatch.lap().report());
    println!("2. {} ({})", solve(&input2), stopwatch.lap().report());
    println!("3. {} ({})", solve(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pub time: usize,
    pub pos: usize,
}

fn solve(chamber: &str) -> usize {
    let width = chamber.find('\n').unwrap() + 1;
    let start_pos = chamber.find('E').unwrap();
    let chamber = chamber.as_bytes();

    // start Dijkstra
    let mut q: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let mut visited = HashSet::new();
    let start = State { time: 0, pos: start_pos };
    q.push(Reverse(start));
    let moves = [Cardinal::North, Cardinal::South, Cardinal::East, Cardinal::West];
    
    while let Some(Reverse(state)) = q.pop() {
        // base case, path found; return time
        if chamber[state.pos] == b'S' { return state.time; }
        
        // add the state to visited list, or skip processing the state if previously visited
        if !visited.insert(state.pos) { continue; }

        // process state into new states by moving to adjacent spaces, determining if it is a 
        // valid move, then computing cost to move to the valid spaces. Finally, add them
        // to the queue.
        moves.iter()
            .filter_map(|dir| {
                // move around chamber, return None if invalid position
                let neighbor_pos= match dir {
                    Cardinal::North => state.pos.checked_sub(width),
                    Cardinal::East => Some(state.pos + 1),
                    Cardinal::South => Some(state.pos + width),
                    Cardinal::West => Some(state.pos - 1),
                }?;

                // return None if a '#' or '\n' is hit.
                if !(chamber[neighbor_pos] as char).is_ascii_alphanumeric() { return None; };
                
                let neighbor_floor = (chamber[neighbor_pos] as char)
                    .to_digit(10)
                    .unwrap_or_default() as i8;
                let current_floor = (chamber[state.pos] as char)
                    .to_digit(10)
                    .unwrap_or_default() as i8;
                let time_cost = (1 + min(
                    (neighbor_floor - current_floor).rem_euclid(10),
                    (current_floor + 10 - neighbor_floor).rem_euclid(10)
                )) as usize;
                Some(State { time: state.time + time_cost, pos: neighbor_pos }) 
            })
            .for_each(|next| q.push(Reverse(next))); // push neighbor states to queue
    }
    unreachable!("Queue empty, but S never reached!");
}

#[test]
fn tests() {
    let tests = ["#######
#6769##
S50505E
#97434#
#######", "SSSSSSSSSSS
S674345621S
S###6#4#18S
S53#6#4532S
S5450E0485S
S##7154532S
S2##314#18S
S971595#34S
SSSSSSSSSSS"];
    assert_eq!(28, solve(tests[0]));
    assert_eq!(14, solve(tests[1]));
    // assert_eq!(X, solve(tests[2]));
}
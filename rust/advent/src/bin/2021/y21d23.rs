use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<String>;
type Output = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    hallway: [Amphipod; 7],
    rooms: Vec<Room>,
}

impl State {
    fn is_finished(&self) -> bool {
        self.hallway.iter().all(|&amphipod| amphipod == Amphipod::E)
            && self.rooms.iter().all(|room| room.is_finished())
    }

    fn get_edges(&self) -> Vec<(usize, State)> {
        // find open rooms and get corresponding values
        for room in self.rooms.iter().filter(|room| room.is_open()) {
            // look left
            let mut steps = 1;
            for hall_spot in (0..=room.hall_index()).rev() {
                // look in hall first
                match self.hallway[hall_spot] {
                    amp if amp == room.id => {
                        return vec![self.edge_from_hall_to_room(hall_spot, room, steps)];
                    }
                    Amphipod::E => {} // keep going
                    _ => {
                        break;
                    } // left blocked; go right
                }
                // look in room (if available)
                if let Some(room_spot) = hall_spot.checked_sub(2) {
                    let top = self.rooms[room_spot].top_amphipod();
                    if top == room.id {
                        return vec![self.edge_from_room_to_room(room_spot, room, steps)];
                    }
                }
                steps = if hall_spot == 1 { steps + 1 } else { steps + 2 };
            }
            // look right
            steps = 1;
            for hall_spot in room.hall_index() + 1..7 {
                // look in hall first
                match self.hallway[hall_spot] {
                    amp if amp == room.id => {
                        return vec![self.edge_from_hall_to_room(hall_spot, room, steps)];
                    },
                    Amphipod::E => {}
                    _ => { break; } // right blocked; go to pop
                }
                // look in room
                let room_spot = hall_spot - 1;
                if room_spot < self.rooms.len() {
                    let top = self.rooms[room_spot].top_amphipod();
                    if top == room.id {
                        return vec![self.edge_from_room_to_room(room_spot, room, steps)];
                    }
                }
                steps = if hall_spot == 5 { steps + 1 } else { steps + 2 };
            }
        }

        // move to popping...
        let mut edges = Vec::new();
        for room in self.rooms.iter().filter(|room| room.is_mixed()) {
            // look left
            let mut steps = 1;
            for hall_spot in (0..=room.hall_index()).rev() {
                if self.hallway[hall_spot] == Amphipod::E {
                    edges.push(self.edge_from_room_to_hall(hall_spot, room, steps));
                } else {
                    break;
                }
                steps = if hall_spot == 1 { steps + 1 } else { steps + 2 };
            }
            // look right
            steps = 1;
            for hall_spot in room.hall_index() + 1..7 {
                if self.hallway[hall_spot] == Amphipod::E {
                    edges.push(self.edge_from_room_to_hall(hall_spot, room, steps));
                } else {
                    break;
                }
                steps += if hall_spot == 5 { 1 } else { 2 };
            }
        }
        edges
    }

    fn edge_from_hall_to_room(
        &self,
        hall_spot: usize,
        room: &Room,
        steps: usize,
    ) -> (usize, State) {
        let mut new_hallway = self.hallway;
        new_hallway[hall_spot] = Amphipod::E;
        let new_rooms = Self::new_rooms(&self, room.id, Room::add_amphipod);
        let new_state = State { hallway: new_hallway, rooms: new_rooms };
        let weight = (steps + room.openings()) * room.id.energy();
        (weight, new_state)
    }

    fn edge_from_room_to_room(
        &self,
        room_spot: usize,
        room: &Room,
        steps: usize,
    ) -> (usize, State) {
        let other = &self.rooms[room_spot];
        let new_rooms = (0..self.rooms.len())
            .map(|idx| {
                match idx {
                    i if i == room.id.ordinal() => room.add_amphipod(),
                    i if i == room_spot => other.remove_amphipod(),
                    _ => self.rooms[idx].clone(),
                }
            })
            .collect_vec();
        let new_state = State { rooms: new_rooms, ..*self };
        let weight = (steps + other.openings() + 1 + room.openings() + 1) * room.id.energy();
        (weight, new_state)
    }

    fn edge_from_room_to_hall(&self, hall_spot: usize, room: &Room, steps: usize) -> (usize, State) {
        let new_amphipod = room.top_amphipod();
        let new_state = {
            let new_hallway = self.new_hallway(hall_spot, new_amphipod);
            let new_rooms = self.new_rooms(room.id, Room::remove_amphipod);
            State { hallway: new_hallway, rooms: new_rooms }
        };
        let weight = (steps + room.openings() + 1) * new_amphipod.energy();
        (weight, new_state)
        
    }

    fn new_hallway(&self, index: usize, amphipod: Amphipod) -> [Amphipod; 7] {
        let mut new_hallway = self.hallway;
        new_hallway[index] = amphipod;
        new_hallway
    }

    fn new_rooms<F>(&self, id: Amphipod, action: F) -> Vec<Room>
    where
        F: Fn(&Room) -> Room,
    {
        (0..self.rooms.len())
            .map(|idx| {
                if idx == id.ordinal() {
                    action(&self.rooms[idx])
                } else {
                    self.rooms[idx].clone()
                }
            })
            .collect()
    }
}

impl Ord for State {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
    E,
}

impl Amphipod {
    const ENTRIES: [Self; 5] = [Self::A, Self::B, Self::C, Self::D, Self::E];
    fn energy(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1_000,
            Amphipod::E => 0,
        }
    }

    fn ordinal(&self) -> usize {
        match self {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
            Amphipod::E => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Room {
    id: Amphipod,
    spots: Vec<Amphipod>,
}

impl Room {
    fn is_finished(&self) -> bool {
        self.spots.iter().all(|&it| it == self.id)
    }

    fn is_open(&self) -> bool {
        self.spots
            .iter()
            .all(|&it| it == Amphipod::E || it == self.id)
            && self.spots.iter().any(|&it| it == Amphipod::E)
    }

    fn hall_index(&self) -> usize {
        self.id.ordinal() + 1
    }

    fn top_amphipod(&self) -> Amphipod {
        *self
            .spots
            .iter()
            .find(|&&spot| spot != Amphipod::E)
            .unwrap_or(&Amphipod::E)
    }

    fn is_mixed(&self) -> bool {
        self.spots.iter().any(|&it| ![Amphipod::E, self.id].contains(&it))
    }

    fn add_amphipod(&self) -> Room {
        let last_empty_index = self.spots.iter().rposition(|&it| it == Amphipod::E).unwrap();
        let mut spots = self.spots.clone();
        spots[last_empty_index] = self.id;
        Room { spots, ..*self }
    }

    fn remove_amphipod(&self) -> Room {
        let first_occupied_index = self.spots.iter()
            .position(|&it| it != Amphipod::E)
            .unwrap();
        let mut spots = self.spots.clone();
        spots[first_occupied_index] = Amphipod::E;
        Room { spots, ..*self }
    }

    fn openings(&self) -> usize {
        self.spots.iter().filter(|&&it| it == Amphipod::E).count()
    }
}
fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 23).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .chunks(4)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect()
}

fn total_energy(strings: Vec<String>) -> Output {
    let start = (0, get_state(strings));

    let mut q: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();
    q.push(Reverse(start));

    let mut weights: FxHashMap<State, usize> = FxHashMap::default();
    let mut visited: FxHashSet<State> = FxHashSet::default();

    while let Some(Reverse((weight, state))) = q.pop() {
        if visited.contains(&state) {
            continue;
        }
        if state.is_finished() {
            return weight;
        }

        for (edge_weight, edge_state) in state.get_edges() {
            let potential_weight = weight + edge_weight;
            if let Some(current_weight) = weights.get_mut(&edge_state) {
                if potential_weight < *current_weight {
                    *current_weight = potential_weight;
                    q.push(Reverse((potential_weight, edge_state)));
                }
            } else {
                weights.insert(edge_state.clone(), potential_weight);
                q.push(Reverse((potential_weight, edge_state)));
            }
        }
        visited.insert(state);
    }
    unreachable!("Queue ran out of edges before solution was found!")
}

fn get_state(strings: Vec<String>) -> State {
    let len = strings.len();
    let collated = (0..len * 4)
        .map(|i| match strings[i % len].as_bytes()[i / len] {
            b'A' => Amphipod::A,
            b'B' => Amphipod::B,
            b'C' => Amphipod::C,
            b'D' => Amphipod::D,
            _ => unreachable!(),
        })
        .chunks(len);
    let rooms = collated
        .into_iter()
        .enumerate()
        .map(|(idx, chunk)| {
            let id = Amphipod::ENTRIES[idx];
            Room {
                id,
                spots: chunk.collect(),
            }
        })
        .collect();
    State {
        hallway: [Amphipod::E; 7],
        rooms,
    }
}

fn part1(strings: Input) -> Output {
    total_energy(strings)
}

fn part2(strings: Input) -> Output {
    let (first, last) = strings.into_iter().collect_tuple().unwrap();
    let expanded_strings = vec![first, "DCBA".to_string(), "DBAC".to_string(), last];
    total_energy(expanded_strings)
}

#[test]
fn default() {
    let input = get_input(21, 23).unwrap();
    let input = parse_input(&input);
    assert_eq!(14148, part1(input.clone()));
    assert_eq!(43814, part2(input));
}

// Input parsed (19μs)
// 1. 14148 (44ms)
// 2. 43814 (153ms)
// Total: 198ms
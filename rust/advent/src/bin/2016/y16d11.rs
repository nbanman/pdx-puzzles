use advent::utilities::get_input::get_input;
use itertools::Itertools;
use lazy_regex::regex;
use rustc_hash::FxHashSet;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 11).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq,)]
enum ItemType {
    Microchip,
    Generator,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq,)]
struct Item<'a> {
    name: &'a str,
    item_type: ItemType,
}

#[derive(Debug, Clone, Eq)]
struct Floor<'a> {
    items: Vec<Item<'a>>,
    microchips: Vec<&'a str>,
    generators: Vec<&'a str>,
}

impl<'a> Floor<'a> {
    fn new(items: Vec<Item<'a>>) -> Self {
        let microchips = items.iter()
            .filter(|item| item.item_type == ItemType::Microchip)
            .map(|item| item.name)
            .collect();
        let generators = items.iter()
            .filter(|item| item.item_type == ItemType::Generator)
            .map(|item| item.name)
            .collect();
        Self { items, microchips, generators }
    }
    
    fn is_valid(&self) -> bool {
        self.microchips.is_empty() || self.generators.is_empty()
            || self.microchips.iter().all(|&chip| self.generators.contains(&chip))
    }
}

impl<'a> PartialEq for Floor<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.microchips.len() == other.microchips.len() && self.generators.len() == other.generators.len()
    }
}

impl<'a> std::hash::Hash for Floor<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.microchips.len().hash(state);
        self.generators.len().hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct FloorState<'a> {
    elevator: usize,
    floors: [Floor<'a>; 4],
}

impl<'a> FloorState<'a> {
    fn is_valid(&self) -> bool {
        self.floors.iter().all(|floor| floor.is_valid())
    }

    fn is_solved(&self) -> bool {
        self.floors.iter().dropping_back(1).all(|floor| floor.items.is_empty())
    }

    fn next(&self) -> Vec<Self> {
        let mut valid_moves = Vec::new();
        let potential_floors = match self.elevator {
            0 => vec![1],
            1 => vec![0, 2],
            2 => vec![1, 3],
            _ => vec![2],
        };

        // Gets all potential loads the elevator can carry by getting combinations of all the items on the floor
        // then checking to see if the remaining items on the floor cause any chips to fry.
        let mut potential_loads = Vec::new();

        let cur_floor = &self.floors[self.elevator];

        for (index, item) in cur_floor.items.iter().enumerate() {
            potential_loads.push(Floor::new(vec!(*item)));
            potential_loads.extend(
                cur_floor.items.iter().skip(index + 1).map(|it| {
                    Floor::new(vec![*item, *it])
                })
            );
        }
        
        // Nested loop that tries all valid loads on all potential floors. If the floor with the new load is valid,
        // add it to the list of potential states.
        for potential_floor in potential_floors {
            for potential_load in potential_loads.iter() {
                let fry_load: Vec<_> = cur_floor.items.iter()
                    .filter(|&it| !potential_load.items.contains(it))
                    .copied()
                    .collect();
                let fry_load = Floor::new(fry_load);
                if fry_load.is_valid() {
                    let new_floors: [Floor<'a>; 4] = std::array::from_fn(|i| {
                        match i {
                            i if i == self.elevator => fry_load.clone(),
                            i if i == potential_floor => {
                                let added = self.floors[i].items.iter()
                                    .chain(potential_load.items.iter())
                                    .copied()
                                    .collect_vec();
                                Floor::new(added)
                                
                            }
                            _ => self.floors[i].clone(),
                        }
                    });
                    let new_state = Self { elevator: potential_floor, floors: new_floors };
                    if new_state.is_valid() {
                        valid_moves.push(new_state);
                    }
                }
            }
        }
        valid_moves
    }
}

fn parse_floors(input: &str, part_2: bool) -> FloorState<'_> {
    let rx = regex!(r"(?<material>\w+)(?: |-compatible )(?<itemType>generator|microchip)");
    let floors = input.lines().collect_vec();
    let floors: [Floor; 4] = std::array::from_fn(|index| {
        let mut items = Vec::new();
        for caps in rx.captures_iter(floors[index]) {
            let name = caps.name("material").unwrap().as_str();
            let item_type = match caps.name("itemType").unwrap().as_str() {
                "generator" => ItemType::Generator,
                "microchip" => ItemType::Microchip,
                _ => unreachable!()
            };
            items.push(Item { name, item_type });
        }
        if index == 0 && part_2 {
            items.push(Item { name: "elerium", item_type: ItemType::Generator });
            items.push(Item { name: "elerium", item_type: ItemType::Microchip });
            items.push(Item { name: "dilithium", item_type: ItemType::Generator });
            items.push(Item { name: "dilithium", item_type: ItemType::Microchip });
        }
        Floor::new(items)
    });
    FloorState { elevator: 0, floors }
}

fn solve_floors(initial_state: FloorState) -> usize {
    let mut steps = 0;
    // cache used to prune previously visited states.
    let mut visited = FxHashSet::default();
    visited.insert(initial_state.clone());
    let mut todo: Vec<FloorState> = Vec::new();
    let mut next: Vec<FloorState> = Vec::new();
    next.push(initial_state);
    loop {
        steps += 1;
        std::mem::swap(&mut todo, &mut next);
        for state in todo.drain( .. ) {
            for neighbor in state.next() {
                if !visited.contains(&neighbor) {
                    if neighbor.is_solved() {
                        return steps
                    }
                    visited.insert(neighbor.clone());
                    next.push(neighbor);
                }
            }
        }
    }
}

fn part1(input: Input) -> Output {
    solve_floors(parse_floors(input, false))
}

fn part2(input: Input) -> Output {
    solve_floors(parse_floors(input, true))
}

#[test]
fn default() {
    let input = get_input(16, 11).unwrap();
    assert_eq!(47, part1(&input));
    assert_eq!(71, part2(&input));
}

// Input parsed (20μs)
// 1. 47 (21ms)
// 2. 71 (113ms)
// Total: 135ms

use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Plants = Vec<Plant>;

#[derive(Debug)]
struct Plant {
    thickness: i64,
    branches: Branches,
}

impl Plant {
    fn get_energy(&self, plants: &[Plant], plant_energy: &mut Vec<Option<i64>>) -> i64 {
        match &self.branches {
            Branches::Free => 1,
            Branches::ToPlants(branches) => {
                let mut energy = 0;
                for branch in branches {
                    let branch_energy = plant_energy[branch.to_plant];
                    let source_energy = if let Some(branch_energy) = branch_energy {
                        branch_energy
                    } else {
                        let branch_energy = plants[branch.to_plant].get_energy(plants, plant_energy);
                        plant_energy[branch.to_plant] = Some(branch_energy);
                        branch_energy
                    };
                    energy += source_energy * branch.thickness;
                }
                if energy >= self.thickness {
                    energy
                } else {
                    0
                }
            },
        }
    }
}

impl From<&str> for Plant {
    fn from(description: &str) -> Self {
        let (plant, branches) = description.split_once("\n").unwrap();
        let thickness: i64 = plant.get_numbers().skip(1).next().unwrap();
        let branches = Branches::from(branches);
        Self { thickness, branches }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Branches {
    Free,
    ToPlants(Vec<Branch>)
}

impl From<&str> for Branches {
    fn from(value: &str) -> Self {
        if value.starts_with("- f") {
            Self::Free
        } else {
            let branches = value.get_numbers().tuples()
                .map(|(plant, thickness)| Branch { thickness, to_plant: plant as usize - 1 })
                .collect();
            Self::ToPlants(branches)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Branch {
    thickness: i64,
    to_plant: usize,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 18);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn get_plants(notes: Input) -> Plants {
    notes.split("\n\n").map(Plant::from).collect()
}

fn part1(notes: Input) -> i64 {
    let plants = get_plants(notes);
    let mut plant_energy: Vec<Option<i64>> = vec![None; plants.len()];
    plants.last().map(|plant| plant.get_energy(&plants, &mut plant_energy)).unwrap()
}

fn part2(notes: Input) -> i64 {
    let (plants, test_cases) = notes.split_once("\n\n\n").unwrap();
    let plants = get_plants(plants);
    let last = plants.last().unwrap();
    test_cases.lines()
        .map(|line| {
            let mut plant_energy: Vec<Option<i64>> = line.get_numbers()
                .map(|n| Some(n))
                .collect();
            plant_energy.resize(plants.len(), None);
            last.get_energy(&plants, &mut plant_energy)
        })
        .sum()
}

fn part3(notes: Input) -> i64 {
    let (plants, test_cases) = notes.split_once("\n\n\n").unwrap();
    let plants = get_plants(plants);
    let last = plants.last().unwrap();

    let mut plant_energy: Vec<Option<i64>> = vec![None; plants.len()];

    let phase_2_start = plants.iter()
        .position(|plant| {
            matches!(plant.branches, Branches::ToPlants(_))
        })
        .unwrap();
    for i in 0..phase_2_start {
        plant_energy[i] = Some(0);
    }
    
    let positive_branch_plants = plants.iter()
        .filter_map(|plant| {
            match &plant.branches {
                Branches::Free => None,
                Branches::ToPlants(branches) => Some(branches),
            }
        })
        .take_while(|branches| branches.len() > 2)
        .flat_map(|branches| {
            branches
                .iter()
                .filter(|branch| branch.thickness > 0)
                .map(|branch| branch.to_plant)
        })
        .unique()
        .sorted_unstable();

    for plant_idx in positive_branch_plants {
        plant_energy[plant_idx] = Some(1);
    }

    let optimum = last.get_energy(&plants, &mut plant_energy);

    test_cases.lines()
        .map(|line| {
            let mut plant_energy: Vec<Option<i64>> = line.get_numbers()
                .map(|n| Some(n))
                .collect();
            plant_energy.resize(plants.len(), None);
            let energy = last.get_energy(&plants, &mut plant_energy);
            if energy == 0 {
                0
            } else {
                optimum - energy
            }
        })
        .sum()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 18);
    assert_eq!(2067316, part1(&input1));
    assert_eq!(15481956620, part2(&input2));
    assert_eq!(485271, part3(&input3));
}

// Input parsed (44μs)
// 1. 2067316 (16μs)
// 2. 15481956620 (86μs)
// 3. 485271 (148μs)
// Total: 298μs

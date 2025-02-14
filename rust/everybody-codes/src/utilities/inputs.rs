use std::{fs::File, io::Read};

use itertools::Itertools;

pub fn get_inputs(year: u16, day: u8) -> (String, String, String) {
    let year = if year > 2000 { year - 2000 } else { year };
    (1..=3)
        .map(|part| get_input(year, day, part))
        .collect_tuple()
        .unwrap()
}

pub fn get_input(year: u16, day: u8, quest: u8) -> String {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to find manifest directory");
    let path = manifest_dir + &format!(
        "/../../inputs/everybody_codes/20{}/y{}d{:02}q{}.txt", 
        year,
        year, 
        day, 
        quest
    );
    let mut file = File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
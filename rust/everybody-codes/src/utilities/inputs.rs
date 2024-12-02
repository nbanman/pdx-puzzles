use std::{fs::File, io::Read};

use itertools::Itertools;

pub fn get_inputs(year: u16, day: u8) -> (String, String, String) {
    let year = if year > 2000 { year - 2000 } else { year };
    (1..=3)
        .map(|part| get_input(year, day, part))
        .collect_tuple()
        .unwrap()
}

pub fn get_input(year: u16, day: u8, part: u8) -> String {
    let path = format!(
        "./everybody-codes/inputs/2024/everybody_codes_e20{}_q{:02}_p{}.txt", 
        year, 
        day, 
        part
    );
    let mut file = File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
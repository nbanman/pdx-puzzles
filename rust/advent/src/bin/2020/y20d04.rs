use advent::utilities::get_input::get_input;
use lazy_regex::regex;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = Vec<PassPort<'a>>;
type Output = usize;
type PassPort<'a> = Vec<PassPortField<'a>>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 4).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug)]
struct PassPortField<'a> {
    field: FieldType,
    info: &'a str,
}

#[derive(Debug)]
enum FieldType {
    Byr, Iyr, Eyr, Hgt, Hcl, Ecl, Pid,
}

impl<'a> PassPortField<'a> {
    fn new(info: &'a str) -> Option<Self> {
        let (field, info) = info.split_once(':').unwrap();
        let field = match field {
            "byr" => Some(FieldType::Byr),
            "iyr" => Some(FieldType::Iyr),
            "eyr" => Some(FieldType::Eyr),
            "hgt" => Some(FieldType::Hgt),
            "hcl" => Some(FieldType::Hcl),
            "ecl" => Some(FieldType::Ecl),
            "pid" => Some(FieldType::Pid),
            "cid" => None,
            _ => panic!("Unknown field \"{}\".", field)
        }?;
        Some(PassPortField { info, field })
    }

    fn is_valid(&self) -> bool {
        match self.field {
            FieldType::Byr => {
                if let Ok(info) = self.info.parse() {
                    (1920..=2002).contains(&info)
                } else { false }
            },
            FieldType::Iyr => {
                if let Ok(info) = self.info.parse() {
                    (2010..=2020).contains(&info)
                } else { false }
            }
            FieldType::Eyr => {
                if let Ok(info) = self.info.parse() {
                    (2020..=2030).contains(&info)
                } else { false }
            }
            FieldType::Hgt => {
                let (amt, is_cm) = self.info.split_at(self.info.len() - 2);
                if let Ok(amt) = amt.parse() {
                    (is_cm == "cm" && (150..=193).contains(&amt))
                        || (is_cm == "in" && (59..=76).contains(&amt))
                } else { false }
            }
            FieldType::Hcl => {
                let hcl_rx = regex!(r"#[a-f\d]{6}");
                hcl_rx.is_match(self.info)
            }
            FieldType::Ecl => {
                let ecl_rx = regex!(r"amb|blu|brn|gry|grn|hzl|oth");
                ecl_rx.is_match(self.info)
            }
            FieldType::Pid => {
                self.info.len() == 9 && self.info.parse::<u32>().is_ok()
            }
        }
    }
}

fn parse_input(input: &'_ str) -> Input<'_> {
    input.split("\n\n")
        .map(|raw_passport_data| {
            raw_passport_data.split_whitespace()
                .filter_map(PassPortField::new)
                .collect::<Vec<_>>()
        })
        .filter(|passport| passport.len() == 7)
        .collect()
}

fn part1(passports: &Input) -> Output {
    passports.len()
}

fn part2(passports: &Input) -> Output {
    passports.iter()
        .filter(|passport| passport.iter().all(|field| field.is_valid()))
        .count()
}

#[test]
fn default() {
    let input = get_input(20, 4).unwrap();
    let input = parse_input(&input);
    assert_eq!(242, part1(&input));
    assert_eq!(186, part2(&input));
}

// Input parsed (159μs)
// 1. 242 (6μs)
// 2. 186 (492μs)
// Total: 662μs
use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::Coord,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Pos = Coord<isize, 2>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(24, 12);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!(
        "1. {} ({})",
        attack_ruins(&input1),
        stopwatch.lap().report()
    );
    println!(
        "2. {} ({})",
        attack_ruins(&input2),
        stopwatch.lap().report()
    );
    println!(
        "3. {} ({})",
        missile_defense(&input3),
        stopwatch.lap().report()
    );
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Eq, PartialEq)]
pub struct Ruins {
    pub target: Target,
    pub pos: Pos,
}

impl PartialOrd for Ruins {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.pos.x().partial_cmp(&other.pos.x()) {
            Some(core::cmp::Ordering::Equal) => other.pos.y().partial_cmp(&self.pos.y()),
            ord => ord,
        }
    }
}

impl Ord for Ruins {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.pos.x().cmp(&other.pos.x()) {
            std::cmp::Ordering::Equal => other.pos.y().cmp(&self.pos.y()),
            ord => ord,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Block,
    Rock,
}

impl Target {
    pub fn value(&self) -> usize {
        match self {
            Target::Block => 1,
            Target::Rock => 2,
        }
    }
}

fn attack_ruins(input: &str) -> usize {
    let (catapults, towers) = get_battlefield(input);
    let mut score = 0;
    for target in towers {
        let ranking = catapults
            .iter()
            .filter_map(|catapult| get_ranking(catapult, &target))
            .next()
            .unwrap();
        score += ranking;
    }
    score
}

fn get_ranking(catapult: &Pos, ruin: &Ruins) -> Option<usize> {
    let diff = ruin.pos - *catapult;
    let adjusted_x = diff.x() + diff.y();
    if adjusted_x % 3 != 0 {
        None
    } else {
        let power = adjusted_x / 3;
        Some(score(catapult.y(), power, &ruin.target))
    }
}

fn get_battlefield(input: &str) -> (Vec<Pos>, Vec<Ruins>) {
    let mut tower = Vec::new();
    let mut catapults = Vec::new();

    // go through each line from ground up and add things as they come along
    for (y, line) in input.lines().rev().enumerate() {
        for (x, &c) in line.as_bytes().iter().enumerate() {
            match c {
                b'A' | b'B' | b'C' => {
                    catapults.push(Pos::new2d(x as isize, y as isize));
                }
                b'T' => {
                    let pos = Pos::new2d(x as isize, y as isize);
                    let ruin = Ruins {
                        target: Target::Block,
                        pos,
                    };
                    tower.push(ruin);
                }
                b'H' => {
                    let pos = Pos::new2d(x as isize, y as isize);
                    let ruin = Ruins {
                        target: Target::Rock,
                        pos,
                    };
                    tower.push(ruin);
                }
                _ => {}
            }
        }
    }
    catapults.sort_unstable();
    tower.sort_unstable();
    (catapults, tower)
}

fn missile_defense(input: &str) -> usize {
    let meteors: Vec<Pos> = input
        .get_numbers()
        .tuples::<(isize, isize)>()
        .map(Pos::from)
        .collect();
    meteors
        .iter()
        .map(|meteor| {
            (0..3)
                .filter_map(|catapult| intercept(catapult, meteor))
                .min()
                .unwrap()
        })
        .sum()
}

fn intercept(catapult: isize, meteor: &Pos) -> Option<usize> {
    // if the x-coordinate is odd, the missile and the meteor will pass by each other every
    // time since we only measure at discrete integers of t. Solve this by launching at t=1,
    // which makes it even.
    let meteor_t1 = if meteor.x() & 1 == 1 {
        *meteor - 1
    } else {
        *meteor
    };

    // the highest altitude will always be half the total x distance, so that's where the
    // missile should hit
    let x = meteor_t1.x() / 2;
    // Step 1: if xm - ym + offset == 0, then you get it on the upswing, and power is xm / 2
    if meteor_t1.x() - meteor_t1.y() + catapult == 0 {
        let power = x;
        let score = score(catapult + 1, power, &Target::Block);
        return Some(score);
    }

    // Step 2: y == bc == p + offset equation
    // at x, y will have dropped by x amount
    let y = meteor_t1.y() - x;

    let power = y - catapult;

    // power calculation only valid for t=power to t=power * 2 inclusive. Before that, it
    // hasn't reached full height. After that, it's dropping.
    if (power..=power * 2).contains(&x) {
        let score = score(catapult + 1, power, &Target::Block);
        return Some(score);
    }

    // Step 3: Apply formula from pts 1 + 2.

    // The formula is based on a distance x, but assumes that the launcher and the meteor are
    // the same height. To adjust for that, add the height difference.
    let adjusted_x = x + (y - catapult);

    // Power is that adjusted distance, divided by 3, because for 1/3 of the time, the
    // projectile will be gaining, 1/3 staying the same, and 1/3 dropping. So if it doesn't
    // divide evenly that means that the projectile will not be y at x.
    // y also has to be lower than x, because otherwise the shot will never get high enough
    // to hit y at x
    if adjusted_x % 3 != 0 || y - catapult > x {
        None
    } else {
        let power = adjusted_x / 3;
        let score = score(catapult + 1, power, &Target::Block);
        Some(score)
    }
}

fn score(catapult: isize, power: isize, target: &Target) -> usize {
    ((catapult) * power * target.value() as isize) as usize
}

//725208, with t1

#[test]
fn tests() {
    let tests = [
        ".............
.C...........
.B......T....
.A......T.T..
=============",
        ".............
.C...........
.B......H....
.A......T.H..
=============",
        "6 5
6 7
10 5",
        "6 5
6 7
10 5, 7, 8",
    ];
    assert_eq!(13, attack_ruins(tests[0]));
    assert_eq!(22, attack_ruins(tests[1]));
    assert_eq!(11, missile_defense(tests[2]));
    missile_defense(tests[3]);
}

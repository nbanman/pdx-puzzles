use everybody_codes::utilities::inputs::get_story_inputs;
use itertools::Itertools;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use utilities::parsing::get_numbers::ContainsNumbers;
use utilities::structs::grid::{Grid, Grid2, GridIterator};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_story_inputs(25, 2, 3);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> usize {
    let mut dice = parse_dice(input).unwrap();
    let mut points = 0;
    let mut rolls = 0;
    while points < 10_000 {
        rolls += 1;
        points += dice
            .iter_mut()
            .map(|die| die.spin(rolls))
            .sum::<isize>();
    }
    rolls
}

fn part2(input: Input) -> String {
    let (dice, track) = input.split_once("\n\n").unwrap();
    let mut dice = parse_dice(dice).unwrap();
    let track: Vec<_> = track.as_bytes().iter().map(|&b| (b - 48) as isize ).collect();
    dice.iter_mut()
        .map(|die| {
            let turns = die.race(&track);
            (die, turns)
        })
        .sorted_by_key(|(_die, turns)| *turns)
        .map(|(die, _turns)| (b'0' + die.id as u8) as char)
        .join(",")
}

fn part3(input: Input) -> usize {
    let (dice, grid) = input.split_once("\n\n").unwrap();
    let mut dice = parse_dice(dice).unwrap();
    let grid = Grid::try_from(grid).unwrap();
    let grid_width = grid.width();
    let grid: Grid<isize, 2> =  grid.iter()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .try_collect_grid(grid_width)
        .unwrap();

    dice.iter_mut()
        .par_bridge()
        .map(|die| die.bfs(&grid))
        .reduce(|| vec![0; grid.len()], |mut acc, die_rolls| {
            for (x, y) in acc.iter_mut().zip(die_rolls) {
                *x += y;
            }
            acc
        })
        .iter()
        .filter(|v| **v != 0)
        .count()
}

fn parse_dice(input: Input) -> Result<Vec<Die>, String> {
    input.lines()
        .map(Die::try_from)
        .collect::<Result<Vec<_>, _>>()
}
struct Die {
    id: usize,
    faces: Vec<isize>,
    face_index: usize,
    seed: usize,
    pulse: usize,
    track_index: usize,
}

impl Die {
    fn spin(&mut self, roll_number: usize) -> isize {
        let spin = roll_number * self.pulse;
        self.face_index = (self.face_index + spin) % self.faces.len();
        self.pulse = (self.pulse + spin) % self.seed + 1 + roll_number + self.seed;
        self.faces[self.face_index]
    }

    fn race_turn(&mut self, turn: usize, track: &[isize]) -> bool {
        if track[self.track_index] == self.spin(turn) {
            self.track_index += 1;
        }
        self.track_index == track.len()
    }

    fn race(&mut self, track: &[isize]) -> usize {
        (1..)
            .find(|turn| self.race_turn(*turn, track))
            .expect("infinite sequence so will never return None")
    }
    fn bfs(&mut self, grid: &Grid2<isize>) -> Vec<usize> {
        let mut current_turn = 1;
        let mut result = self.spin(current_turn);

        let mut todo = Vec::with_capacity(100_000);
        let mut next = Vec::with_capacity(100_000);
        let mut visited = vec![0; grid.len()];

        for (pos, value) in grid.iter().enumerate() {
            if *value == result {
                todo.push(pos);
                visited[pos] = current_turn;
            }
        }

        while !todo.is_empty() {
            // rerolls the next result once per turn
            current_turn += 1;
            result = self.spin(current_turn);

            for pos in todo.drain(..) {
                let neighbors = grid
                    .adjacent(pos, false)
                    .expect("all positions should be in grid")
                    .map(|neighbor| (neighbor.index, neighbor.value.clone()))
                    .chain(std::iter::once((pos, grid[pos])));

                for (n_pos, n_value) in neighbors {
                    if n_value == result && visited[n_pos] != current_turn {
                        visited[n_pos] = current_turn;
                        next.push(n_pos);
                    }
                }
            }
            (todo, next) = (next, todo);
        }
        visited
    }
}

impl TryFrom<&str> for Die {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (id, faces, seed) = value
            .split_whitespace()
            .collect_tuple()
            .ok_or(format!("Invalid input: {}", value))?;

        let id = id.split_once(':')
            .ok_or(format!("Invalid id input: {}", value))?
            .0
            .parse::<usize>()
            .map_err(|_| format!("Invalid id input: {}", value))?;

        let faces = faces.get_numbers().collect::<Vec<isize>>();
        let seed = seed.split_once('=')
            .ok_or(format!("Invalid seed input: {}", seed))?
            .1
            .parse::<usize>()
            .map_err(|_| format!("Invalid seed input: {}", seed))?;
        Ok(Die {
            id,
            faces,
            face_index: 0,
            seed,
            pulse: seed,
            track_index: 0,
        })
    }
}

#[test]
fn default() {
    let (input1, input2, input3) = get_story_inputs(25, 2, 3);
    assert_eq!(637, part1(&input1));
    assert_eq!("2,7,3,9,6,8,1,4,5".to_string(), part2(&input2));
    assert_eq!(154381, part3(&input3));
}

#[test]
fn examples() {
    let inputs = [
        r"1: faces=[1,2,3,4,5,6] seed=7
2: faces=[-1,1,-1,1,-1] seed=13
3: faces=[9,8,7,8,9] seed=17",
        r"1: faces=[1,2,3,4,5,6,7,8,9] seed=13
2: faces=[1,2,3,4,5,6,7,8,9] seed=29
3: faces=[1,2,3,4,5,6,7,8,9] seed=37
4: faces=[1,2,3,4,5,6,7,8,9] seed=43

51257284",
        r"1: faces=[1,2,3,4,5,6,7,8,9] seed=13

1523758297
4822941583
7627997892
4397697132
1799773472",
        r"1: faces=[1,2,3,4,5,6,7,8,9] seed=339211
2: faces=[1,2,3,4,5,6,7,8,9] seed=339517
3: faces=[1,2,3,4,5,6,7,8,9] seed=339769
4: faces=[1,2,3,4,5,6,7,8,9] seed=339049
5: faces=[1,2,3,4,5,6,7,8,9] seed=338959
6: faces=[1,2,3,4,5,6,7,8,9] seed=340111
7: faces=[1,2,3,4,5,6,7,8,9] seed=339679
8: faces=[1,2,3,4,5,6,7,8,9] seed=339121
9: faces=[1,2,3,4,5,6,7,8,9] seed=338851

94129478611916584144567479397512595367821487689499329543245932151
45326719759656232865938673559697851227323497148536117267854241288
44425936468288462848395149959678842215853561564389485413422813386
64558359733811767982282485122488769592428259771817485135798694145
17145764554656647599363636643624443394141749674594439266267914738
89687344812176758317288229174788352467288242171125512646356965953
72436836424726621961424876248346712363842529736689287535527512173
18295771348356417112646514812963612341591986162693455745689374361
56445661964557624561727322332461348422854112571195242864151143533
77537797151985578367895335725777225518396231453691496787716283477
37666899356978497489345173784484282858559847597424967325966961183
26423131974661694562195955939964966722352323745667498767153191712
99821139398463125478734415536932821142852955688669975837535594682
17768265895455681847771319336534851247125295119363323122744953158
25655579913247189643736314385964221584784477663153155222414634387
62881693835262899543396571369125158422922821541597516885389448546
71751114798332662666694134456689735288947441583123159231519473489
94932859392146885633942828174712588132581248183339538341386944937
53828883514868969493559487848248847169557825166338328352792866332
54329673374115668178556175692459528276819221245996289611868492731
97799599164121988455613343238811122469229423272696867686953891233
56249752581283778997317243845187615584225693829653495119532543712
39171354221177772498317826968247939792845866251456175433557619425
56425749216121421458547849142439211299266255482219915528173596421
48679971256541851497913572722857258171788611888347747362797259539
32676924489943265499379145361515824954991343541956993467914114579
45733396847369746189956225365375253819969643711633873473662833395
42291594527499443926636288241672629499242134451937866578992236427
47615394883193571183931424851238451485822477158595936634849167455
16742896921499963113544858716552428241241973653655714294517865841
57496921774277833341488566199458567884285639693339942468585269698
22734249697451127789698862596688824444191118289959746248348491792
28575193613471799766369217455617858422158428235521423695479745656
74234343226976999161289522983885254212712515669681365845434541257
43457237419516813368452247532764649744546181229533942414983335895"];
    assert_eq!(844, part1(inputs[0]));
    assert_eq!("1,3,4,2".to_string(), part2(inputs[1]));
    assert_eq!(33, part3(inputs[2]));
    assert_eq!(1125, part3(inputs[3]));
}

// Input parsed (185μs)
// 1. 637 (18μs)
// 2. 2,7,3,9,6,8,1,4,5 (564μs)
// 3. 154381 (30ms)
// Total: 31ms
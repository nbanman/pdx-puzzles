use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 9).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> Output {
    let mut fragmented = Vec::new();
    for (idx, n) in input.chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate() {
        let value = if idx & 1 == 0 { Some(idx / 2) } else { None };
        for _ in 0..n {
            fragmented.push(value);
        }
    }
    let mut left = 0;
    let mut right = fragmented.len() - 1;
    let blocks = fragmented.iter().filter(|it| it.is_some()).count();
    let mut defragged = Vec::new();
    while left < blocks {
        while let Some(block) = fragmented[left] {
            if left < right { 
                defragged.push(block);
                left += 1;
            } else {
                break;
            }
        }
        while let None = fragmented[right] { right -= 1; }
        left += 1;
        defragged.push(fragmented[right].unwrap());
        right -= 1;
    }
    defragged.iter().enumerate().fold(0, |state, (idx, &i)| {
        state + idx * i
    })
}

#[derive(Debug)]
enum Block<'a> {
    DataBlock(&'a Data),
    SpaceBlock(&'a Space),
}

impl Block<'_> {
    fn checksum(&self) -> usize {
        match self {
            Block::DataBlock(data) => data.checksum(),
            Block::SpaceBlock(space) => space.checksum(),
        }
    }
}

#[derive(Debug)]
struct Data {
    index: usize,
    size: usize,
    value: usize,
}

impl Data {
    fn checksum(&self) -> usize { self.checksum_from_index(self.index) }

    fn checksum_from_index(&self, index: usize) -> usize {
        (index..index + self.size)
            .map(|idx| idx * self.value)
            .sum()
    }
}


#[derive(Debug)]
struct Space {
    index: usize,
    size: usize,
    data: Vec<Data>
}

impl Space {
    fn checksum(&self) -> usize {
        let mut checksum = 0;
        let mut index = self.index;
        for datum in self.data.iter() {
            checksum += datum.checksum_from_index(index);
            index += datum.size
        }
        checksum
    }
}

fn part2(input: Input) -> Output {
    let mut index = 0;
    let mut spaces = Vec::new();
    let mut data = Vec::new();
    for (order, size) in input.chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate() {
        let size = size as usize;
        if size > 0 {
            if order & 1 == 0 {
                let datum = Data {
                    index,
                    size,
                    value: order / 2,
                };
                data.push(datum);
            } else {
                let space = Space { index, size, data: Vec::new() };
                spaces.push(space);
            }
        }
        index += size
    }
    let mut space_ref: Vec<&mut Space> = spaces.iter_mut().collect();
    for data_idx in (0..data.len()).rev() {
        if let Some(space_idx) = space_ref.iter().position(|space| 
            space.size >= data[data_idx].size && space.index < data[data_idx].index
        ) {
            let datum = data.remove(data_idx);
            space_ref[space_idx].size -= datum.size;
            space_ref[space_idx].data.push(datum);
            if space_ref[space_idx].size == 0 {
                space_ref.remove(space_idx);
            }
        }
    }

    data.iter().map(Block::DataBlock)
        .chain(spaces.iter().map(Block::SpaceBlock))
        .map(|block| block.checksum())
        .sum()
}

#[test]
fn default() {
    let input = get_input(24, 9).unwrap();
    assert_eq!(6390180901651, part1(&input));
    assert_eq!(6412390114238, part2(&input));
}

#[test]
fn examples() {
    let inputs = [r"", ];
    assert_eq!(1, part1(&inputs[0]));
    // assert_eq!(Y, part2(&input));
}
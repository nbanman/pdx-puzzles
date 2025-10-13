use itertools::Itertools;

pub fn dense_hash(lengths: &[usize]) -> String {
    let ring: Vec<usize> = (0..256).collect();   
    let shift_sum = lengths.iter().sum::<usize>() * 64;
    let skip_sum = lengths.len() * 64;
    let mut reshifted = (0..64).fold(ring, |acc, i| knot_hash(acc, lengths, i * lengths.len()));
    let reshifted_len = reshifted.len();
    let total_skips = (1..skip_sum).sum::<usize>();
    reshifted.rotate_right((total_skips + shift_sum) % reshifted_len);

    let dense_hash = reshifted.into_iter().chunks(16).into_iter()
        .map(|chunk| {
            let reduction = chunk.reduce(|acc, i| acc ^ i).unwrap();
            format!("{:02x}", reduction)
        })
        .collect();
    dense_hash
}

pub fn knot_hash(ring: Vec<usize>, lengths: &[usize], skip: usize) -> Vec<usize> {
    lengths.iter().enumerate().fold(ring, |acc, (index, &length)| {
        let reverse_part: Vec<usize> = acc[..length].iter().rev().copied().collect();
        let mut knot: Vec<usize> = reverse_part.iter()
            .chain(acc.iter().skip(reverse_part.len()))
            .copied()
            .collect();
        let knot_len = knot.len();
        knot.rotate_left((length + skip + index) % knot_len);
        knot
    })
}

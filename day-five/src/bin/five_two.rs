fn main() {
    let mut input = include_str!("input.txt").split_terminator("\n\n");

    let seeds: Vec<u64> = input
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();
    let maps = input
        .map(|chunk| {
            chunk
                .lines()
                .skip(1) // can skip mapping names as they are always in order
                .map(|line| {
                    let data: Vec<u64> = line.split(' ').map(|n| n.parse().unwrap()).collect();
                    (data[0] as i64 - data[1] as i64, data[1]..data[1] + data[2])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let solution = seeds
        .chunks(2)
        .flat_map(<&[u64; 2]>::try_from)
        .flat_map(|&[start, len]| {
            // for each seed range, generate all matching ranges for each level
            maps.iter().fold(vec![start..(start + len)], |acc, map| {
                acc.into_iter()
                    .flat_map(|input_rng| {
                        // build set of reachable ranges from this layer for input ranges
                        let mut matching_ranges = map
                            .iter()
                            .filter_map(|(offset, src_rng)| {
                                if src_rng.end <= input_rng.start || input_rng.end <= src_rng.start
                                {
                                    None // does not overlap
                                } else {
                                    Some(
                                        (src_rng.start.max(input_rng.start) as i64 + offset) as u64
                                            ..(src_rng.end.min(input_rng.end) as i64 + offset)
                                                as u64,
                                    )
                                }
                            })
                            .collect::<Vec<_>>();

                        // pass input range through if no ranges overlap
                        if matching_ranges.is_empty() {
                            matching_ranges.push(input_rng)
                        }

                        matching_ranges
                    })
                    .collect()
            })
        })
        .map(|min_range| min_range.start)
        .min()
        .unwrap();

    println!("{solution}")
}

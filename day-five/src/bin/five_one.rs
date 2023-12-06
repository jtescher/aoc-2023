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
                    (data[0], data[1]..data[1] + data[2])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let solution = seeds
        .into_iter()
        .map(|seed| {
            // traverse each mapping in order to get to location value
            maps.iter().fold(seed, |src, map| {
                map.iter()
                    .find_map(|(dest, range)| {
                        if range.contains(&src) {
                            Some(dest + (src - range.start))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(src)
            })
        })
        .min()
        .unwrap();

    println!("{solution}")
}

fn main() {
    let mut input = include_str!("input.txt").lines();

    let solution: usize = input
        .next()
        .unwrap()
        .split_whitespace()
        .flat_map(|n| n.parse().ok())
        .zip(
            input
                .next()
                .unwrap()
                .split_whitespace()
                .flat_map(|n| n.parse().ok()),
        )
        .map(|(time, distance): (usize, usize)| {
            let low_root = (time as f64 - (((time.pow(2) - 4 * distance) as f64).sqrt())) / 2.0;
            let high_root = (time as f64 + (((time.pow(2) - 4 * distance) as f64).sqrt())) / 2.0;
            (high_root.ceil() - low_root.floor()) as usize - 1
        })
        .product();

    println!("{solution}")
}

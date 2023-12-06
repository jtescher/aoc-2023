fn main() {
    let mut input = include_str!("input.txt").lines();
    let time: usize = input
        .next()
        .unwrap()
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: usize = input
        .next()
        .unwrap()
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap();

    let low_root = (time as f64 - (((time.pow(2) - 4 * distance) as f64).sqrt())) / 2.0;
    let high_root = (time as f64 + (((time.pow(2) - 4 * distance) as f64).sqrt())) / 2.0;
    let solution = (high_root.ceil() - low_root.floor()) as usize - 1;

    println!("{solution}")
}

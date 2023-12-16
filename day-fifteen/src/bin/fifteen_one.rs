fn main() {
    let input = include_str!("input.txt");

    let solution = input
        .trim()
        .split_terminator(',')
        .map(|s| s.chars().fold(0, |acc, c| ((acc + c as u64) * 17) % 256))
        .sum::<u64>();

    println!("{solution}")
}

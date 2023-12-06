use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let solution: u32 = input
        .lines()
        .filter_map(|line| {
            let (correct, guesses) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let correct = correct
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<HashSet<u32>>();
            let count = guesses
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .filter(|num| correct.contains(num))
                .count();

            if count > 0 {
                Some(2u32.pow((count - 1) as u32))
            } else {
                None
            }
        })
        .sum();

    println!("{solution}")
}

use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    let solution = input
        .lines()
        .rev()
        .fold(HashMap::new(), |mut acc, line| {
            let (card_num, card_data) = line.trim_start_matches("Card ").split_once(':').unwrap();
            let card_num = card_num.trim().parse::<usize>().unwrap();
            let (correct, guesses) = card_data.split_once('|').unwrap();

            let correct = correct
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<HashSet<usize>>();
            let hits = guesses
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .filter(|num| correct.contains(num))
                .count();
            let total = (1..=hits)
                .map(|num| *acc.get(&(card_num + num)).unwrap())
                .sum::<usize>()
                + 1;

            acc.insert(card_num, total);
            acc
        })
        .values()
        .sum::<usize>();

    println!("{solution}")
}

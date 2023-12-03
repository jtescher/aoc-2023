use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let (mut stars, numbers) = input.lines().enumerate().fold(
        (HashMap::new(), vec![]),
        |(mut stars, mut numbers), (row, line)| {
            let mut num_range: Option<(usize, Vec<char>)> = None;
            for (col, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    if let Some((_, chars)) = &mut num_range {
                        chars.push(c);
                    } else {
                        num_range = Some((col, vec![c]))
                    }
                } else {
                    if let Some((col, chars)) = num_range.take() {
                        numbers.push((row, col, chars));
                    }
                    if c == '*' {
                        stars.insert((row, col), HashSet::new());
                    }
                }
            }
            if let Some((col, chars)) = num_range.take() {
                numbers.push((row, col, chars));
            }

            (stars, numbers)
        },
    );

    for (row, col, chars) in numbers {
        for col in col..col + chars.len() {
            stars
                .entry((row.saturating_sub(1), col.saturating_sub(1)))
                .and_modify(|e| {
                    e.insert(chars.clone());
                });
            stars.entry((row.saturating_sub(1), col)).and_modify(|e| {
                e.insert(chars.clone());
            });
            stars
                .entry((row.saturating_sub(1), col + 1))
                .and_modify(|e| {
                    e.insert(chars.clone());
                });
            stars.entry((row, col.saturating_sub(1))).and_modify(|e| {
                e.insert(chars.clone());
            });
            stars.entry((row, col + 1)).and_modify(|e| {
                e.insert(chars.clone());
            });
            stars
                .entry((row + 1, col.saturating_sub(1)))
                .and_modify(|e| {
                    e.insert(chars.clone());
                });
            stars.entry((row + 1, col)).and_modify(|e| {
                e.insert(chars.clone());
            });
            stars.entry((row + 1, col + 1)).and_modify(|e| {
                e.insert(chars.clone());
            });
        }
    }

    let solution: u32 = stars
        .into_values()
        .filter_map(|numbers| {
            if numbers.len() == 2 {
                Some(
                    numbers
                        .into_iter()
                        .map(|chars| {
                            chars
                                .into_iter()
                                .rev()
                                .enumerate()
                                .map(|(i, c)| 10u32.pow(i as u32) * (c as u32 - '0' as u32))
                                .sum::<u32>()
                        })
                        .product::<u32>(),
                )
            } else {
                None
            }
        })
        .sum();

    println!("{solution}")
}

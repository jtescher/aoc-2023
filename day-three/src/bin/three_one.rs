use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let (symbols, numbers) = input.lines().enumerate().fold(
        (HashSet::new(), vec![]),
        |(mut symbols, mut numbers), (row, line)| {
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
                    if c != '.' {
                        symbols.insert((row, col));
                    }
                }
            }
            if let Some((col, chars)) = num_range.take() {
                numbers.push((row, col, chars));
            }

            (symbols, numbers)
        },
    );

    let solution: u32 = numbers
        .into_iter()
        .filter_map(|(row, col, chars)| {
            if (col..col + chars.len()).any(|col| {
                symbols.contains(&(row.saturating_sub(1), col.saturating_sub(1)))
                    || symbols.contains(&(row.saturating_sub(1), col))
                    || symbols.contains(&(row.saturating_sub(1), col + 1))
                    || symbols.contains(&(row, col.saturating_sub(1)))
                    || symbols.contains(&(row, col + 1))
                    || symbols.contains(&(row + 1, col.saturating_sub(1)))
                    || symbols.contains(&(row + 1, col))
                    || symbols.contains(&(row + 1, col + 1))
            }) {
                Some(
                    chars
                        .into_iter()
                        .rev()
                        .enumerate()
                        .map(|(i, c)| 10u32.pow(i as u32) * (c as u32 - '0' as u32))
                        .sum::<u32>(),
                )
            } else {
                None
            }
        })
        .sum();

    println!("{solution}")
}

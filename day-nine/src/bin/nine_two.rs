use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    let solution = input
        .lines()
        .map(|line| {
            let mut rows = vec![line
                .split_whitespace()
                .flat_map(FromStr::from_str)
                .collect::<Vec<_>>()];

            while !rows.last().unwrap().iter().all(|&x| x == 0) {
                let deltas = rows[rows.len() - 1]
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect();
                rows.push(deltas);
            }

            let mut last = 0;
            for row in rows.iter_mut().rev().skip(1) {
                last = row[0] - last;
                row.insert(0, last);
            }

            rows[0][0]
        })
        .sum::<i64>();

    println!("{solution}")
}

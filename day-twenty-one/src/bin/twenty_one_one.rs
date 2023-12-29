use std::collections::{HashSet, VecDeque};

fn main() {
    let mut start = (0, 0);
    let grid = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        start = (row, col);
                        '.'
                    } else {
                        c
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut solution = HashSet::new();
    let mut seen: HashSet<_> = [start].into_iter().collect();
    let mut q: VecDeque<_> = [(start.0, start.1, 64)].into_iter().collect();

    while let Some((row, col, steps)) = q.pop_front() {
        if steps % 2 == 0 {
            solution.insert((row, col));
        }
        if steps == 0 {
            continue;
        }

        for (dy, dx) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let row = (row as isize + dy) as usize;
            let col = (col as isize + dx) as usize;

            if row < grid.len()
                && col < grid[0].len()
                && grid[row][col] == '.'
                && seen.insert((row, col))
            {
                q.push_back((row, col, steps - 1));
            }
        }
    }

    println!("{}", solution.len())
}

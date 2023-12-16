use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let mut grid = input.lines().map(|line| line.chars().collect()).collect();

    cycle_n(&mut grid, 1_000_000_000);

    let solution = (0..grid[0].len())
        .map(|col| {
            (0..grid.len())
                .filter(|&row| grid[row][col] == 'O')
                .map(|row| grid.len() - row)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{solution}")
}

fn cycle_n(grid: &mut Vec<Vec<char>>, n: usize) {
    let mut cycle_cache = HashMap::new();
    let mut i = 0;
    while i < n {
        i += 1;

        // north
        for col in 0..grid[0].len() {
            let mut next_slot = 0;

            for row in 0..grid.len() {
                if grid[row][col] == 'O' {
                    grid[row][col] = '.';
                    grid[next_slot][col] = 'O';
                    next_slot += 1;
                } else if grid[row][col] == '#' {
                    next_slot = row + 1;
                }
            }
        }

        // west
        for row in 0..grid.len() {
            let mut next_slot = 0;

            for col in 0..grid[0].len() {
                if grid[row][col] == 'O' {
                    grid[row][col] = '.';
                    grid[row][next_slot] = 'O';
                    next_slot += 1;
                } else if grid[row][col] == '#' {
                    next_slot = col + 1;
                }
            }
        }

        // south
        for col in 0..grid[0].len() {
            let mut next_slot = grid.len() - 1;

            for row in (0..grid.len()).rev() {
                if grid[row][col] == 'O' {
                    grid[row][col] = '.';
                    grid[next_slot][col] = 'O';
                    next_slot = next_slot.saturating_sub(1);
                } else if grid[row][col] == '#' {
                    next_slot = row.saturating_sub(1);
                }
            }
        }

        // east
        for row in 0..grid.len() {
            let mut next_slot = grid[0].len() - 1;

            for col in (0..grid[0].len()).rev() {
                if grid[row][col] == 'O' {
                    grid[row][col] = '.';
                    grid[row][next_slot] = 'O';
                    next_slot = next_slot.saturating_sub(1);
                } else if grid[row][col] == '#' {
                    next_slot = col.saturating_sub(1);
                }
            }
        }

        if let Some(prev_i) = cycle_cache.get(grid) {
            let steps_per_cycle = i - prev_i;
            let cycles = (n - i) / steps_per_cycle;
            i += cycles * steps_per_cycle;
        }

        cycle_cache.insert(grid.clone(), i);
    }
}

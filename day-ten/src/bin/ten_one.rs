use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let mut start = (0, 0);
    let mut grid = input
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    if c == 'S' {
                        start = (row_idx, col_idx);
                    }
                    c
                })
                .collect()
        })
        .collect::<Vec<_>>();

    let mut prev = start;
    let mut current = start_loop(&mut grid, start);
    let mut grid_loop: HashSet<_> = [(current), (prev)].into_iter().collect();
    while current != start {
        let next = next_position(&grid, current, prev);
        grid_loop.insert(next);
        prev = current;
        current = next;
    }

    println!("{}", grid_loop.len() / 2)
}

fn next_position(
    grid: &[Vec<char>],
    mut current: (usize, usize),
    prev: (usize, usize),
) -> (usize, usize) {
    let c = grid[current.0][current.1];
    match c {
        '-' => current.1 = (current.1 as isize + (current.1 as isize - prev.1 as isize)) as usize,
        '|' => current.0 = (current.0 as isize + (current.0 as isize - prev.0 as isize)) as usize,
        'F' | '7' => {
            if current.0 == prev.0 {
                current.0 += 1;
            } else if c == 'F' {
                current.1 += 1;
            } else {
                current.1 -= 1;
            }
        }
        'L' | 'J' => {
            if current.0 == prev.0 {
                current.0 -= 1;
            } else if c == 'L' {
                current.1 += 1;
            } else {
                current.1 -= 1;
            }
        }
        _ => unreachable!(),
    }

    current
}

fn start_loop(grid: &mut [Vec<char>], (row, col): (usize, usize)) -> (usize, usize) {
    let up = matches!(grid[row.saturating_sub(1)][col], '|' | '7' | 'F');
    let down = matches!(grid[row + 1][col], '|' | 'L' | 'J');
    let left = matches!(grid[row][col.saturating_sub(1)], '-' | 'F' | 'L');
    let right = matches!(grid[row][col + 1], '-' | 'J' | '7');

    let (s_char, next) = match (up, down, left, right) {
        (_, _, true, true) => ('-', (row, col + 1)),
        (_, true, _, true) => ('F', (row, col + 1)),
        (_, true, true, _) => ('7', (row + 1, col)),
        (true, _, _, true) => ('L', (row, col + 1)),
        (true, _, true, _) => ('J', (row - 1, col)),
        (true, true, _, _) => ('|', (row + 1, col)),
        _ => unreachable!(),
    };

    grid[row][col] = s_char;

    next
}

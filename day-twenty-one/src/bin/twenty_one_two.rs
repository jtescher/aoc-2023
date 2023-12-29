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

    let steps = 26501365;
    let size = grid.len();
    let grid_width = steps / size - 1;
    let odd = (grid_width / 2 * 2 + 1).pow(2);
    let even = ((grid_width + 1) / 2 * 2).pow(2);

    // assuming grid is square...
    assert_eq!(grid.len(), grid[0].len());
    // assuming start is at the center of the grid...
    assert!(start.0 == size / 2 && start.1 == size / 2);
    // assuming number of steps is enough to exactly reach the end of a grid
    assert!(steps % size == size / 2);

    let odd_points = n_points(start, size * 2 + 1, &grid);
    let even_points = n_points(start, size * 2, &grid);

    let corner_t = n_points((size - 1, start.1), size - 1, &grid);
    let corner_r = n_points((start.0, 0), size - 1, &grid);
    let corner_b = n_points((0, start.1), size - 1, &grid);
    let corner_l = n_points((start.0, size - 1), size - 1, &grid);

    let small_tr = n_points((size - 1, 0), size / 2 - 1, &grid);
    let small_tl = n_points((size - 1, size - 1), size / 2 - 1, &grid);
    let small_br = n_points((0, 0), size / 2 - 1, &grid);
    let small_bl = n_points((0, size - 1), size / 2 - 1, &grid);

    let large_tr = n_points((size - 1, 0), size * 3 / 2 - 1, &grid);
    let large_tl = n_points((size - 1, size - 1), size * 3 / 2 - 1, &grid);
    let large_br = n_points((0, 0), size * 3 / 2 - 1, &grid);
    let large_bl = n_points((0, size - 1), size * 3 / 2 - 1, &grid);

    let solution = odd * odd_points
        + even * even_points
        + corner_t
        + corner_r
        + corner_b
        + corner_l
        + (grid_width + 1) * (small_tr + small_tl + small_br + small_bl)
        + grid_width * (large_tr + large_tl + large_br + large_bl);

    println!("{solution}")
}

fn n_points(start: (usize, usize), step_count: usize, grid: &[Vec<char>]) -> usize {
    let mut plots = HashSet::new();
    let mut seen: HashSet<_> = [start].into_iter().collect();
    let mut q: VecDeque<_> = [(start.0, start.1, step_count)].into_iter().collect();

    while let Some((row, col, steps)) = q.pop_front() {
        if steps % 2 == 0 {
            plots.insert((row, col));
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

    plots.len()
}

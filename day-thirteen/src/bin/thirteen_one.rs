fn main() {
    let input = include_str!("input.txt");

    let solution = input
        .split_terminator("\n\n")
        .filter_map(|chunk| {
            let grid = chunk
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            find_horizontal_mirror(&grid)
                .map(|n| n * 100)
                .or_else(|| find_horizontal_mirror(&rotate(&grid)))
        })
        .sum::<usize>();

    println!("{solution}")
}

fn find_horizontal_mirror(grid: &[Vec<char>]) -> Option<usize> {
    (0..grid.len() - 1)
        .find(|&i| {
            let max_offset = i.min(grid.len() - i - 2);
            for offset in 0..=max_offset {
                if grid[i - offset] != grid[i + 1 + offset] {
                    return false;
                }
            }
            true
        })
        .map(|i| i + 1)
}

fn rotate<T: Copy>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut res = vec![Vec::with_capacity(grid.len()); grid[0].len()];

    for row in grid.iter() {
        for (c_idx, &el) in row.iter().enumerate() {
            res[c_idx].push(el)
        }
    }

    res
}

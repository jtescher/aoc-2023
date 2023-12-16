fn main() {
    let input = include_str!("input.txt");
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let solution = (0..grid[0].len())
        .map(|col| {
            let mut next_slot = 0;

            (0..grid.len()).fold(0, |mut sum, row| {
                if grid[row][col] == 'O' {
                    sum += grid.len() - next_slot;
                    next_slot += 1;
                } else if grid[row][col] == '#' {
                    next_slot = row + 1;
                }

                sum
            })
        })
        .sum::<usize>();

    println!("{solution}")
}

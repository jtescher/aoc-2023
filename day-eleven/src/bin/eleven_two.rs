fn main() {
    let input = include_str!("input.txt");
    let mut row_zeroes = vec![];
    let mut column_zeroes = vec![0; input.lines().next().unwrap().chars().count()];
    let mut galaxies = vec![];

    let mut empty_rows_total = 0;
    for (row_idx, line) in input.lines().enumerate() {
        let mut seen_row = false;
        for (col_idx, c) in line.chars().enumerate() {
            if c == '#' {
                seen_row = true;
                column_zeroes[col_idx] = 1;
                galaxies.push([row_idx, col_idx]);
            }
        }
        empty_rows_total += usize::from(!seen_row);
        row_zeroes.push(empty_rows_total);
    }

    let mut empty_cols_total = 0;
    for count in column_zeroes.iter_mut() {
        empty_cols_total += usize::from(*count == 0);
        *count = empty_cols_total;
    }

    let solution = (0..galaxies.len())
        .map(|i| {
            ((i + 1)..galaxies.len())
                .map(|oi| {
                    let gal_y = galaxies[i][0] + row_zeroes[galaxies[i][0]] * 999_999;
                    let gal_x = galaxies[i][1] + column_zeroes[galaxies[i][1]] * 999_999;

                    let other_y = galaxies[oi][0] + row_zeroes[galaxies[oi][0]] * 999_999;
                    let other_x = galaxies[oi][1] + column_zeroes[galaxies[oi][1]] * 999_999;

                    gal_x.abs_diff(other_x) + gal_y.abs_diff(other_y)
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{solution}")
}

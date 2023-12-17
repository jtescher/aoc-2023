use std::collections::HashMap;

fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let solution = (0..grid.len())
        .map(|r| ((r as i32, 0), Dir::Right))
        .chain((0..grid.len()).map(|r| ((r as i32, (grid[0].len() - 1) as i32), Dir::Left)))
        .chain((0..grid[0].len()).map(|c| ((0 as i32, c as i32), Dir::Down)))
        .chain((0..grid[0].len()).map(|c| (((grid.len() - 1) as i32, c as i32), Dir::Up)))
        .map(|(init, dir)| {
            let mut beams = vec![(init, dir)];
            let mut energized = beams.iter().copied().collect::<HashMap<_, _>>();

            while let Some(mut beam) = beams.pop() {
                while let (Some(next), split) = get_next(&beam, &grid) {
                    if energized.insert(next.0, next.1) == Some(next.1) {
                        break; // found cycle
                    }
                    beam = next;

                    if let Some(other) = split {
                        beams.push(other);
                    }
                }
            }

            energized.len()
        })
        .max()
        .unwrap();

    println!("{solution}")
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn get_next(
    ((row, col), dir): &((i32, i32), Dir),
    grid: &[Vec<char>],
) -> (Option<((i32, i32), Dir)>, Option<((i32, i32), Dir)>) {
    let next = match dir {
        Dir::Right => (*row, *col + 1),
        Dir::Left => (*row, *col - 1),
        Dir::Up => (*row - 1, *col),
        Dir::Down => (*row + 1, *col),
    };
    if next.0 < 0 || next.0 >= grid.len() as i32 || next.1 < 0 || next.1 >= grid[0].len() as i32 {
        return (None, None);
    }

    let (dir, split_dir) = match grid[next.0 as usize][next.1 as usize] {
        '/' => (
            match dir {
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Right => Dir::Up,
                Dir::Up => Dir::Right,
            },
            None,
        ),
        '\\' => (
            match dir {
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
            },
            None,
        ),
        '|' if matches!(dir, Dir::Left | Dir::Right) => (Dir::Down, Some(Dir::Up)),
        '-' if matches!(dir, Dir::Up | Dir::Down) => (Dir::Left, Some(Dir::Right)),
        _ => (*dir, None),
    };

    (Some((next, dir)), split_dir.map(|d| (next, d)))
}

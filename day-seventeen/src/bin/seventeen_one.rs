use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as usize - '0' as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let solution = modified_dijkstra(&grid, (0, 0), (grid.len() - 1, grid[0].len() - 1)).unwrap();

    println!("{solution}")
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (usize, usize),
    dir: (isize, isize),
    steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn modified_dijkstra(
    grid: &[Vec<usize>],
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    let mut pq = BinaryHeap::new();
    let mut seen = HashSet::new();

    pq.push(Reverse(State {
        cost: 0,
        pos: start,
        dir: (0, 0),
        steps: 0,
    }));

    while let Some(Reverse(State {
        cost,
        pos,
        dir,
        steps,
    })) = pq.pop()
    {
        if pos == end {
            return Some(cost);
        }
        if !seen.insert((pos, dir, steps)) {
            continue;
        }

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = (pos.0 as isize + dx) as usize;
            let new_y = (pos.1 as isize + dy) as usize;
            let new_dir = (dx, dy);

            if new_x < grid.len() && new_y < grid[0].len() && new_dir != (-dir.0, -dir.1) {
                let new_steps = if dir == new_dir { steps + 1 } else { 1 };

                if new_steps <= 3 {
                    pq.push(Reverse(State {
                        cost: cost + grid[new_x][new_y],
                        pos: (new_x, new_y),
                        dir: new_dir,
                        steps: new_steps,
                    }));
                }
            }
        }
    }

    None
}

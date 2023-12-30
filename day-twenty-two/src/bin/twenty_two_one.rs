use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn main() {
    let mut bricks = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.replace('~', ",")
                .split_terminator(',')
                .flat_map(FromStr::from_str)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // initial sort by z index
    bricks.sort_unstable_by_key(|brick| brick[2]);

    let overlaps =
        |a: &[u32], b: &[u32]| a[0].max(b[0]) <= a[3].min(b[3]) && a[1].max(b[1]) <= a[4].min(b[4]);

    // move all bricks down to lowest non-overlapping z index
    for i in 0..bricks.len() {
        let max_z = bricks[..i]
            .iter()
            .filter(|brick| overlaps(&bricks[i], brick))
            .fold(1, |max_z, brick| max_z.max(brick[5] + 1));

        bricks[i][5] -= bricks[i][2] - max_z;
        bricks[i][2] = max_z
    }

    // final sort as order may have changed
    bricks.sort_unstable_by_key(|brick| brick[2]);

    let mut k_supports_v: HashMap<_, _> = (0..bricks.len()).map(|i| (i, HashSet::new())).collect();
    let mut v_supports_k = k_supports_v.clone();

    for (j, upper) in bricks.iter().enumerate() {
        for (i, lower) in bricks[..j].iter().enumerate() {
            if overlaps(lower, upper) && upper[2] == lower[5] + 1 {
                k_supports_v.entry(i).or_default().insert(j);
                v_supports_k.entry(j).or_default().insert(i);
            }
        }
    }

    // find all bricks that are supported by more than one
    let solution = (0..bricks.len())
        .filter(|i| k_supports_v[i].iter().all(|j| v_supports_k[j].len() >= 2))
        .count();

    println!("{solution}")
}

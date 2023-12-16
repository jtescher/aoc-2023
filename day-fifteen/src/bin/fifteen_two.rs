fn main() {
    let input = include_str!("input.txt");

    let buckets = input
        .trim()
        .split_terminator(',')
        .fold(vec![Vec::new(); 256], |mut acc, op| {
            if let Some((label, val)) = op.split_once('=') {
                let val = val.parse::<usize>().unwrap();
                let bucket = &mut acc[hash(label)];
                if let Some((_, v)) = bucket.iter_mut().find(|(l, _)| *l == label) {
                    *v = val
                } else {
                    bucket.push((label, val));
                }
            } else {
                acc[hash(op.trim_end_matches('-'))].retain(|(l, _)| !op.starts_with(l));
            }

            acc
        });

    let solution = buckets
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(bi, (_, val))| (bi + 1) * (i + 1) * val)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{solution}")
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

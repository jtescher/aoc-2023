use std::{collections::HashMap, mem};

fn main() {
    let input = include_str!("input.txt");
    let (instructions, network) = input.split_once("\n\n").unwrap();
    let network = network.lines().fold(HashMap::new(), |mut acc, line| {
        let (src, dests) = line.split_once(" = ").unwrap();
        acc.insert(src, dests[1..9].split_once(", ").unwrap());
        acc
    });

    let solution = network
        .keys()
        .filter(|p| p.ends_with('A'))
        .cloned()
        .map(|mut pos| {
            let mut instructions = instructions.chars().cycle();
            let mut steps = 0;
            while !pos.ends_with('Z') {
                let ins = instructions.next().unwrap();
                let (left, right) = network.get(pos).unwrap();
                pos = if ins == 'L' { left } else { right };
                steps += 1;
            }
            steps
        })
        .reduce(least_common_multiple)
        .unwrap();

    println!("{solution}")
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let (instructions, network) = input.split_once("\n\n").unwrap();
    let mut instructions = instructions.chars().cycle();
    let network = network.lines().fold(HashMap::new(), |mut acc, line| {
        let (src, dests) = line.split_once(" = ").unwrap();
        acc.insert(src, dests[1..9].split_once(", ").unwrap());
        acc
    });

    let mut pos = "AAA";
    let mut steps = 0;
    while pos != "ZZZ" {
        let ins = instructions.next().unwrap();
        let (left, right) = network.get(pos).unwrap();
        pos = if ins == 'L' { left } else { right };
        steps += 1;
    }

    println!("{steps}")
}

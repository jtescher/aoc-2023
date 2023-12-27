use std::{collections::HashMap, ops::RangeInclusive};

fn main() {
    let workflows = include_str!("input.txt")
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (name, rules) = line.split_once('{').unwrap();
            let rules = rules
                .trim_end_matches('}')
                .split_terminator(',')
                .map(|rule| {
                    if let Some((cond, res)) = rule.split_once(':') {
                        let operation_i = cond.chars().position(|c| !c.is_alphabetic()).unwrap();
                        let attr = &cond[0..operation_i];
                        let op = &cond[operation_i..operation_i + 1];
                        let val = cond[operation_i + 1..].parse::<u64>().unwrap();
                        (Some((attr, op, val)), res)
                    } else {
                        (None, rule)
                    }
                })
                .collect::<Vec<_>>();
            (name, rules)
        })
        .collect::<HashMap<_, _>>();

    let solution = input_combinations([1..=4000, 1..=4000, 1..=4000, 1..=4000], "in", &workflows)
        .into_iter()
        .map(|ranges| {
            ranges
                .into_iter()
                .map(|range| (range.end() - range.start()) + 1)
                .product::<u64>()
        })
        .sum::<u64>();

    println!("{solution}")
}

fn input_combinations(
    mut ranges: [RangeInclusive<u64>; 4],
    name: &str,
    workflows: &HashMap<&str, Vec<(Option<(&str, &str, u64)>, &str)>>,
) -> Vec<[RangeInclusive<u64>; 4]> {
    if name == "A" {
        return vec![ranges];
    } else if name == "R" {
        return Vec::new();
    }

    workflows[name]
        .iter()
        .flat_map(|(condition, name)| {
            if let Some((attr, op, val)) = condition {
                let i = ["x", "m", "a", "s"].iter().position(|a| a == attr).unwrap();
                let mut inclusive_ranges = ranges.clone();
                if op == &">" {
                    inclusive_ranges[i] = *(ranges[i].start()).max(&(val + 1))..=*(ranges[i].end());
                    ranges[i] = *(ranges[i].start())..=*val
                } else {
                    inclusive_ranges[i] = *(ranges[i].start())..=(*val - 1);
                    ranges[i] = *(ranges[i].start()).max(val)..=*(ranges[i].end())
                }
                input_combinations(inclusive_ranges, name, workflows)
            } else {
                input_combinations(ranges.clone(), name, workflows)
            }
        })
        .collect()
}

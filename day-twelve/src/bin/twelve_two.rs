use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("input.txt");

    let solution = input
        .lines()
        .map(|line| {
            let (conditions, crit) = line.split_once(' ').unwrap();
            let conditions: Vec<_> = conditions.chars().collect();
            let conditions: Vec<_> = conditions
                .iter()
                .chain([&'?'])
                .cycle()
                .take((conditions.len() * 5) + 4)
                .copied()
                .collect();
            let crit: Vec<usize> = crit
                .split_terminator(',')
                .flat_map(FromStr::from_str)
                .collect();
            let criteria: Vec<_> = crit.iter().cycle().take(crit.len() * 5).copied().collect();

            arrangement_count(&conditions, &criteria, 0, &mut HashMap::new())
        })
        .sum::<usize>();

    println!("{solution}")
}

fn arrangement_count(
    conditions: &[char],
    criteria: &[usize],
    current_run: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(&count) = cache.get(&(conditions.len(), criteria.len(), current_run)) {
        return count;
    } else if conditions.is_empty() {
        if criteria.is_empty() && current_run == 0
            || criteria.len() == 1 && current_run == criteria[0]
        {
            return 1; // valid arrangement
        } else {
            return 0; // invalid arrangement
        }
    }

    let count = ['.', '#']
        .into_iter()
        .filter(|&condition| conditions[0] == condition || conditions[0] == '?')
        .map(|condition| match condition {
            '#' if criteria.is_empty() || current_run >= criteria[0] => 0,
            '#' => arrangement_count(&conditions[1..], criteria, current_run + 1, cache),
            '.' if current_run == 0 => arrangement_count(&conditions[1..], criteria, 0, cache),
            '.' if criteria.first() == Some(&current_run) => {
                arrangement_count(&conditions[1..], &criteria[1..], 0, cache)
            }
            _ => 0,
        })
        .sum();

    cache.insert((conditions.len(), criteria.len(), current_run), count);
    count
}

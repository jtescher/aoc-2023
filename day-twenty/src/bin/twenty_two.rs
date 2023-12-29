use std::{
    collections::{HashMap, VecDeque},
    mem,
};

fn main() {
    let mut reverse_mapping = HashMap::<&str, Vec<(bool, &str)>>::new();
    let mut modules = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (module, outputs) = line.split_once(" -> ").unwrap();
            let (m_name, m_type, state) = if let Some(flipflop) = module.strip_prefix('%') {
                (flipflop, '%', vec![(false, "")])
            } else if let Some(conjunction) = module.strip_prefix('&') {
                (conjunction, '&', vec![])
            } else {
                (module, 'b', vec![])
            };
            let outputs = outputs
                .split_terminator(", ")
                .inspect(|o| reverse_mapping.entry(o).or_default().push((false, m_name)))
                .collect::<Vec<_>>();

            (m_name, (m_type, state, outputs))
        })
        .collect::<HashMap<_, _>>();

    // initialize all conjunction states
    for (m_name, (_, state, _)) in modules.iter_mut().filter(|(_, module)| module.0 == '&') {
        *state = reverse_mapping.remove(m_name).unwrap();
    }

    // rx has one '&' input, so track that input's signals for cycles.
    let rx_input = &reverse_mapping["rx"][0].1;
    let mut cycle_lengths = HashMap::new();
    let mut seen = modules
        .iter()
        .filter(|(_, module)| module.2.contains(rx_input))
        .map(|(&name, _)| (name, 0))
        .collect::<HashMap<_, _>>();

    let mut solution: usize = 0;
    'outer: loop {
        solution += 1;
        let mut queue: VecDeque<_> = [(false, "broadcaster", "button")].into_iter().collect();

        while let Some((pulse, dest, src)) = queue.pop_front() {
            let Some((m_type, state, outputs)) = modules.get_mut(dest) else {
                continue;
            };

            if *rx_input == dest && pulse {
                *seen.get_mut(&src).unwrap() += 1;

                if let Some(len) = cycle_lengths.get(src) {
                    assert_eq!(solution, seen[&src] * len);
                } else {
                    cycle_lengths.insert(src, solution);
                }

                if seen.values().all(|&x| x > 0) {
                    solution = cycle_lengths
                        .into_values()
                        .fold(1, |acc, len| acc * (len / gcd(acc, len)));
                    break 'outer;
                }
            }

            let new_pulse = match m_type {
                '%' => {
                    if pulse {
                        None
                    } else {
                        state[0].0 = !state[0].0;
                        Some(state[0].0)
                    }
                }
                '&' => {
                    let prev = state.iter_mut().find(|prev| prev.1 == src).unwrap();
                    prev.0 = pulse;
                    Some(!state.iter().all(|&(is_high, _)| is_high))
                }
                _ => Some(pulse),
            };

            if let Some(new_pulse) = new_pulse {
                outputs
                    .iter()
                    .for_each(|o| queue.push_back((new_pulse, o, dest)));
            }
        }
    }

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

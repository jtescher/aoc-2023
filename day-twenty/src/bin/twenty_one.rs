use std::collections::{HashMap, VecDeque};

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

    let (high, low) = (0..1_000).fold((0, 0), |mut acc, _| {
        let mut queue: VecDeque<_> = [(false, "broadcaster", "button")].into_iter().collect();

        while let Some((pulse, dest, src)) = queue.pop_front() {
            acc = (acc.0 + usize::from(pulse), acc.1 + usize::from(!pulse));

            let Some((m_type, state, outputs)) = modules.get_mut(dest) else {
                continue;
            };

            let new_pulse = match m_type {
                '%' => {
                    if !pulse {
                        state[0].0 = !state[0].0;
                        Some(state[0].0)
                    } else {
                        None
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

        acc
    });

    println!("{}", high * low)
}

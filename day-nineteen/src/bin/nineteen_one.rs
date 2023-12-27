use std::collections::HashMap;

fn main() {
    let (workflows, ratings) = include_str!("input.txt").split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
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

    let solution = ratings
        .lines()
        .map(|line| {
            line.trim_start_matches('{')
                .trim_end_matches('}')
                .split_terminator(',')
                .flat_map(|kv| kv.split_once('='))
                .map(|(key, value)| (key, value.parse().unwrap()))
                .collect()
        })
        .filter(|part| is_accepted("in", part, &workflows))
        .map(|part| part.values().sum::<u64>())
        .sum::<u64>();

    println!("{solution}")
}

fn is_accepted(
    workflow_name: &str,
    part: &HashMap<&str, u64>,
    workflows: &HashMap<&str, Vec<(Option<(&str, &str, u64)>, &str)>>,
) -> bool {
    if workflow_name == "A" {
        return true;
    } else if workflow_name == "R" {
        return false;
    };

    for (condition, name) in &workflows[workflow_name] {
        if !condition.is_some_and(|(attr, op, val)| {
            (op == ">" && part[attr] < val) || (op == "<" && part[attr] > val)
        }) {
            return is_accepted(name, part, workflows);
        }
    }

    panic!("invalid workflow name {workflow_name}")
}

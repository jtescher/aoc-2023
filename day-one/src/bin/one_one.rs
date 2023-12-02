fn main() {
    let input = include_str!("input.txt");
    let solution = input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .filter(|&c| char::is_digit(c, 10))
                .next()
                .unwrap();
            let last = line
                .chars()
                .filter(|&c| char::is_digit(c, 10))
                .rev()
                .next()
                .unwrap();

            format!("{first}{last}").parse::<u64>().unwrap()
        })
        .sum::<u64>();

    println!("{solution}");
}

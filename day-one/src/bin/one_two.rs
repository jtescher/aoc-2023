static NUMBERS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("input.txt");

    let solution = input
        .lines()
        .map(|line| {
            let first = NUMBERS
                .iter()
                .enumerate()
                .flat_map(|(pos, num)| line.find(num).map(|idx| (idx, pos)))
                .max_by_key(|&(idx, _)| idx)
                .map(|(_, num)| num % 9 + 1)
                .expect("should find at least one number");

            let last = NUMBERS
                .iter()
                .enumerate()
                .flat_map(|(pos, num)| line.rfind(num).map(|idx| (idx, pos)))
                .max_by_key(|&(idx, _)| idx)
                .map(|(_, num)| num % 9 + 1)
                .expect("should find at least one number");

            (first * 10) + last
        })
        .sum::<usize>();

    println!("{solution}");
}

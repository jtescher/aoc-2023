fn main() {
    let input = include_str!("input.txt");
    let solution = input
        .lines()
        .map(|line| {
            let first = line.chars().find(char::is_ascii_digit).unwrap() as u32 - '0' as u32;
            let last = line.chars().rev().find(char::is_ascii_digit).unwrap() as u32 - '0' as u32;

            (first * 10) + last
        })
        .sum::<u32>();

    println!("{solution}");
}

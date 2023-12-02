const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let input = include_str!("data.txt");
    let solution: u32 = input
        .lines()
        .filter_map(|line| {
            let (id, rounds) = line
                .trim_start_matches("Game ")
                .split_once(':')
                .expect("data format");
            let id = id.parse::<u32>().expect("numeric game numbers");

            if rounds.split_terminator(';').all(|round| {
                round.split_terminator(',').all(|selection| {
                    let (count, color) = selection
                        .trim()
                        .split_once(' ')
                        .expect("should have number, color pairs");
                    let count = count.parse::<u32>().unwrap();
                    match color {
                        "red" => count <= MAX_RED,
                        "blue" => count <= MAX_BLUE,
                        "green" => count <= MAX_GREEN,
                        other => panic!("expected color, got {other}"),
                    }
                })
            }) {
                Some(id)
            } else {
                None
            }
        })
        .sum();

    println!("{solution}")
}

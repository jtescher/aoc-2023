fn main() {
    let input = include_str!("data.txt");

    let solution: u32 = input
        .lines()
        .map(|line| {
            let (_game, rounds) = line.split_once(':').expect("game n: data format");

            let (min_r, min_g, min_b) =
                rounds
                    .split_terminator(';')
                    .fold((0, 0, 0), |(r, g, b), round| {
                        let (rr, rg, rb) =
                            round
                                .split_terminator(',')
                                .fold((0, 0, 0), |(r, g, b), selection| {
                                    let (count, color) = selection
                                        .trim()
                                        .split_once(' ')
                                        .expect("should have number, color pairs");
                                    let count = count.parse::<u32>().unwrap();
                                    match color {
                                        "red" => (count.max(r), g, b),
                                        "blue" => (r, g, count.max(b)),
                                        "green" => (r, count.max(g), b),
                                        other => panic!("expected color, got {other}"),
                                    }
                                });

                        (r.max(rr), g.max(rg), b.max(rb))
                    });

            min_r * min_g * min_b
        })
        .sum();

    println!("{solution}")
}

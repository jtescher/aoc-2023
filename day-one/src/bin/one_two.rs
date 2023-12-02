fn main() {
    let input = include_str!("input.txt");

    fn first_number(chunk: &str) -> Option<char> {
        let norm = [
            chunk.match_indices("1").next().map(|(i, _)| (i, '1')),
            chunk.match_indices("one").next().map(|(i, _)| (i, '1')),
            chunk.match_indices("2").next().map(|(i, _)| (i, '2')),
            chunk.match_indices("two").next().map(|(i, _)| (i, '2')),
            chunk.match_indices("3").next().map(|(i, _)| (i, '3')),
            chunk.match_indices("three").next().map(|(i, _)| (i, '3')),
            chunk.match_indices("4").next().map(|(i, _)| (i, '4')),
            chunk.match_indices("four").next().map(|(i, _)| (i, '4')),
            chunk.match_indices("5").next().map(|(i, _)| (i, '5')),
            chunk.match_indices("five").next().map(|(i, _)| (i, '5')),
            chunk.match_indices("6").next().map(|(i, _)| (i, '6')),
            chunk.match_indices("six").next().map(|(i, _)| (i, '6')),
            chunk.match_indices("7").next().map(|(i, _)| (i, '7')),
            chunk.match_indices("seven").next().map(|(i, _)| (i, '7')),
            chunk.match_indices("8").next().map(|(i, _)| (i, '8')),
            chunk.match_indices("eight").next().map(|(i, _)| (i, '8')),
            chunk.match_indices("9").next().map(|(i, _)| (i, '9')),
            chunk.match_indices("nine").next().map(|(i, _)| (i, '9')),
        ];

        norm.iter()
            .flatten()
            .min_by_key(|(idx, _)| idx)
            .map(|(_, num)| *num)
    }

    fn last_number(chunk: &str) -> Option<char> {
        let chunk = chunk.chars().rev().collect::<String>();
        let norm = [
            chunk.match_indices("1").next().map(|(i, _)| (i, '1')),
            chunk.match_indices("eno").next().map(|(i, _)| (i, '1')),
            chunk.match_indices("2").next().map(|(i, _)| (i, '2')),
            chunk.match_indices("owt").next().map(|(i, _)| (i, '2')),
            chunk.match_indices("3").next().map(|(i, _)| (i, '3')),
            chunk.match_indices("eerht").next().map(|(i, _)| (i, '3')),
            chunk.match_indices("4").next().map(|(i, _)| (i, '4')),
            chunk.match_indices("ruof").next().map(|(i, _)| (i, '4')),
            chunk.match_indices("5").next().map(|(i, _)| (i, '5')),
            chunk.match_indices("evif").next().map(|(i, _)| (i, '5')),
            chunk.match_indices("6").next().map(|(i, _)| (i, '6')),
            chunk.match_indices("xis").next().map(|(i, _)| (i, '6')),
            chunk.match_indices("7").next().map(|(i, _)| (i, '7')),
            chunk.match_indices("neves").next().map(|(i, _)| (i, '7')),
            chunk.match_indices("8").next().map(|(i, _)| (i, '8')),
            chunk.match_indices("thgie").next().map(|(i, _)| (i, '8')),
            chunk.match_indices("9").next().map(|(i, _)| (i, '9')),
            chunk.match_indices("enin").next().map(|(i, _)| (i, '9')),
        ];

        norm.iter()
            .flatten()
            .min_by_key(|(idx, _)| idx)
            .map(|(_, num)| *num)
    }

    let solution = input
        .lines()
        .map(|line| {
            let first = first_number(line).unwrap();
            let last = last_number(line).unwrap();

            format!("{first}{last}",).parse::<u64>().unwrap()
        })
        .sum::<u64>();

    println!("{solution}");
}

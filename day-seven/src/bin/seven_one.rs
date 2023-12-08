use std::cmp::Ordering;

fn card_strength(card: char) -> u32 {
    match card {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'J' => 4,
        'T' => 5,
        '2'..='9' => 13 - (card as u32 - '2' as u32),
        o => panic!("unexpected char {o}"),
    }
}

fn hand_type(match_count: usize, max_match_count: u32) -> u32 {
    match match_count {
        1 => 1, // five of a kind
        2 => {
            if max_match_count == 4 {
                2 // four of a kind
            } else {
                3 // full house
            }
        }
        3 => {
            if max_match_count == 3 {
                4 // three of a kind
            } else {
                5 // two pair
            }
        }
        4 => 6, // one pair
        5 => 7, // high card
        o => panic!("unexpected card counts {o}"),
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut hands: Vec<_> = input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(hand, bid)| {
            let bid: u32 = bid.parse().unwrap();

            let (hand_strengths, hand_matches) = hand.chars().enumerate().fold(
                ([0; 5], [0; 13]),
                |(mut strengths, mut matches), (idx, card)| {
                    let strength = card_strength(card);
                    strengths[idx] = strength;
                    matches[(strength - 1) as usize] += 1;

                    (strengths, matches)
                },
            );
            let match_count = hand_matches.iter().filter(|&&c| c != 0).count();
            let max_match_count = hand_matches.into_iter().max().unwrap_or(0);

            (hand_type(match_count, max_match_count), hand_strengths, bid)
        })
        .collect();

    hands.sort_unstable_by(|(a_type, a_hand, _), (b_type, b_hand, _)| {
        a_type.cmp(&b_type).then_with(|| {
            a_hand
                .iter()
                .zip(b_hand.iter())
                .find_map(|(a, b)| if a != b { Some(a.cmp(b)) } else { None })
                .unwrap_or(Ordering::Equal)
        })
    });

    let solution = hands
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) * bid as usize)
        .sum::<usize>();

    println!("{solution}")
}

use std::cmp::Ordering;

fn card_strength(card: char) -> u32 {
    match card {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'T' => 4,
        '2'..='9' => 12 - (card as u32 - '2' as u32),
        'J' => 13,
        o => panic!("unexpected char {o}"),
    }
}

fn hand_type(match_count: usize, max_match_count: u32, j_count: u32) -> u32 {
    match match_count {
        1 => 1, // five of a kind
        2 => {
            if max_match_count == 4 {
                if j_count == 4 {
                    1 // four j + other == five of a kind
                } else {
                    2 - j_count // four of a kind
                }
            } else if j_count == 3 || j_count == 2 {
                1 // five of a kind with wilds
            } else {
                3 // full house
            }
        }
        3 => {
            if max_match_count == 3 {
                if j_count == 3 || j_count == 1 {
                    2 // four of a kind
                } else {
                    4 // three of a kind
                }
            } else if j_count == 2 {
                2 // four of a kind
            } else if j_count == 1 {
                3 // full house
            } else {
                5 // two pair
            }
        }
        4 => {
            if j_count == 2 || j_count == 1 {
                4 // three of a kind
            } else {
                6 // one pair
            }
        }
        5 => 7 - j_count, // high card
        o => panic!("unexpected card counts {o}"),
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut hands = input
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
            let hand_type = hand_type(
                hand_matches.iter().filter(|&&c| c != 0).count(),
                hand_matches.into_iter().max().unwrap_or(0),
                hand_matches[12],
            );

            (hand_type, hand_strengths, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_unstable_by(|(a_type, a_hand, _), (b_type, b_hand, _)| {
        a_type.cmp(b_type).then_with(|| {
            a_hand
                .iter()
                .zip(b_hand.iter())
                .find_map(|(a, b)| if a != b { Some(a.cmp(b)) } else { None })
                .unwrap_or(Ordering::Equal)
        })
    });

    let solution = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) * (*bid) as usize)
        .sum::<usize>();

    println!("{solution}")
}

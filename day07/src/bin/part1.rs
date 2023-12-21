fn main() {
    let input = include_str!("input.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug)]
struct CamelHand {
    hand_vec: Vec<char>,
    hand_type: u32,
    bid: u32,
}

struct Card {
    card_type: char,
    strength: u32,
}

fn solution(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    let cards = vec![
        Card {
            card_type: 'A',
            strength: 13,
        },
        Card {
            card_type: 'K',
            strength: 12,
        },
        Card {
            card_type: 'Q',
            strength: 11,
        },
        Card {
            card_type: 'J',
            strength: 10,
        },
        Card {
            card_type: 'T',
            strength: 9,
        },
        Card {
            card_type: '9',
            strength: 8,
        },
        Card {
            card_type: '8',
            strength: 7,
        },
        Card {
            card_type: '7',
            strength: 6,
        },
        Card {
            card_type: '6',
            strength: 5,
        },
        Card {
            card_type: '5',
            strength: 4,
        },
        Card {
            card_type: '4',
            strength: 3,
        },
        Card {
            card_type: '3',
            strength: 2,
        },
        Card {
            card_type: '2',
            strength: 1,
        },
    ];

    let mut all_hands: Vec<CamelHand> = get_hands(lines);

    all_hands.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            let mut a_val_strength = 0;
            let mut b_val_strength = 0;
            for (i, a_val) in a.hand_vec.iter().enumerate() {
                if *a_val == b.hand_vec[i] {
                    continue;
                }

                a_val_strength = cards
                    .iter()
                    .find(|card| card.card_type == *a_val)
                    .unwrap()
                    .strength;
                b_val_strength = cards
                    .iter()
                    .find(|card| card.card_type == b.hand_vec[i])
                    .unwrap()
                    .strength;

                break;
            }

            b_val_strength.cmp(&a_val_strength)
        } else {
            a.hand_type.cmp(&b.hand_type)
        }
    });

    all_hands
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, hand)| {
            acc + (hand.bid * (index as u32 + 1))
        })
}

fn get_hands(lines: Vec<&str>) -> Vec<CamelHand> {
    let mut hands: Vec<CamelHand> = Vec::new();
    for line in lines {
        let splitted = line.split_whitespace().collect::<Vec<_>>();

        let hand_vec = splitted[0].chars().collect::<Vec<_>>();

        let hand_type = get_hand_type(&hand_vec);

        hands.push(CamelHand {
            hand_vec,
            bid: splitted[1].parse::<u32>().unwrap(),
            hand_type,
        })
    }
    hands
}

fn get_hand_type(hand: &Vec<char>) -> u32 {
    let mut hand: Vec<char> = hand.to_vec();
    let hand_type;

    hand.sort_by(|a, b| a.cmp(&b));
    let sorted_copy: Vec<char> = hand.clone();
    hand.dedup();

    hand_type = match hand.len() {
        1 => 1,
        2 => full_or_four(sorted_copy),
        3 => two_or_three(sorted_copy),
        4 => 6,
        5 => 7,
        _ => 10,
    };

    // println!("hand type!:\n{:?}", hand_type);
    hand_type
}

fn full_or_four(hand: Vec<char>) -> u32 {
    let hand_type;

    let filtered = hand
        .iter()
        .filter(|char| **char != hand[0])
        .collect::<Vec<_>>();

    hand_type = match filtered.len() {
        1 => 2,
        2 => 3,
        3 => 3,
        4 => 2,
        _ => 10,
    };

    hand_type
}

fn two_or_three(hand: Vec<char>) -> u32 {
    let hand_type;

    let filtered = hand
        .iter()
        .filter(|char| **char != hand[0])
        .collect::<Vec<_>>();

    hand_type = match filtered.len() {
        2 => 4,
        3 => 5,
        4 => {
            if filtered
                .iter()
                .filter(|c| **c == filtered[0])
                .collect::<Vec<_>>()
                .len()
                == 2
            {
                5
            } else {
                4
            }
        }
        _ => 10,
    };

    hand_type
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483",
        );
        assert_eq!(result, 6440)
    }
}

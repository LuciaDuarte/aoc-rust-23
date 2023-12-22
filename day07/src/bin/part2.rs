fn main() {
    let input = include_str!("input.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug)]
struct CamelHand {
    hand_vec: Vec<char>,
    updated_hand_vec: Vec<char>,
    hand_type: u32,
    bid: u32,
}

struct Card {
    card_type: char,
    strength: u32,
}

fn solution(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    // A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J

    let cards: Vec<Card> = vec![
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
            card_type: 'T',
            strength: 10,
        },
        Card {
            card_type: '9',
            strength: 9,
        },
        Card {
            card_type: '8',
            strength: 8,
        },
        Card {
            card_type: '7',
            strength: 7,
        },
        Card {
            card_type: '6',
            strength: 6,
        },
        Card {
            card_type: '5',
            strength: 5,
        },
        Card {
            card_type: '4',
            strength: 4,
        },
        Card {
            card_type: '3',
            strength: 3,
        },
        Card {
            card_type: '2',
            strength: 2,
        },
        Card {
            card_type: 'J',
            strength: 1,
        },
    ];

    let mut all_hands: Vec<CamelHand> = get_hands(lines, &cards);

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

    println!("All Hands!:\n{:?}", all_hands);

    all_hands
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, hand)| {
            acc + (hand.bid * (index as u32 + 1))
        })
}

fn get_hands(lines: Vec<&str>, cards: &Vec<Card>) -> Vec<CamelHand> {
    let mut hands: Vec<CamelHand> = Vec::new();
    for line in lines {
        let splitted = line.split_whitespace().collect::<Vec<_>>();

        let hand_vec = splitted[0].chars().collect::<Vec<_>>();
        let updated_hand_vec = get_updated_hand(&hand_vec, &cards);

        let hand_type = get_hand_type(&updated_hand_vec);

        hands.push(CamelHand {
            hand_vec: hand_vec.clone(),
            updated_hand_vec,
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

fn get_updated_hand(hand: &Vec<char>, cards: &Vec<Card>) -> Vec<char> {
    if !hand.contains(&'J') {
        return hand.clone();
    }

    let mut updated_hand: Vec<char> = Vec::new();
    let mut hand = hand;
    let mut hand_copy = hand.clone();
    let mut hand_elements: Vec<(char, u32)> = Vec::new();
    hand_copy.sort_by(|a, b| a.cmp(&b));
    hand_copy.dedup();

    if hand_copy.len() == 1 && hand_copy.contains(&'J') {
        return hand.clone();
    }

    hand_copy.iter().for_each(|char: &char| {
        hand_elements.push((
            *char,
            hand.iter()
                .filter(|c| c == &char)
                .collect::<Vec<_>>()
                .len()
                .try_into()
                .unwrap(),
        ))
    });

    hand_elements.sort_by(|a, b| b.1.cmp(&a.1));

    let hand_elements_filtered = hand_elements
        .iter()
        .filter(|el| el.0 != 'J')
        .collect::<Vec<_>>();

    if hand_elements_filtered.len() > 1
        && hand_elements_filtered[0].1 == hand_elements_filtered[1].1
    {
        let first_value = cards
            .iter()
            .find(|card| card.card_type == hand_elements_filtered[0].0)
            .unwrap()
            .strength;

        let second_value = cards
            .iter()
            .find(|card| card.card_type == hand_elements_filtered[1].0)
            .unwrap()
            .strength;

        let highest_card = if first_value > second_value {
            hand_elements_filtered[0].0
        } else {
            hand_elements_filtered[1].0
        };

        updated_hand = hand
            .iter()
            .map(|c| if c == &'J' { highest_card } else { *c })
            .collect();
    } else {
        updated_hand = hand
            .iter()
            .map(|c| {
                if c == &'J' {
                    hand_elements_filtered[0].0
                } else {
                    *c
                }
            })
            .collect();
    }

    updated_hand
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
        assert_eq!(result, 5905)
    }
}

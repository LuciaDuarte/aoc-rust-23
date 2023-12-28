fn main() {
    let input = include_str!("input.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug)]
struct CardWinnings {
    id: i32,
    amount_winnings: i32,
}

fn solution(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();

    let mut card_winnings: Vec<CardWinnings> = Vec::new();

    let mut amount_cards: Vec<(i32, i32)> = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        let split_numbers = line.split("|").collect::<Vec<_>>();
        let winning_numbers_str = split_numbers[0].split(":").collect::<Vec<_>>()[1]
            .split(" ")
            .collect::<Vec<_>>();

        let scratched_numbers_str = split_numbers[1].split(" ").collect::<Vec<_>>();

        amount_cards.push((index as i32 + 1, 1));

        card_winnings.push(CardWinnings {
            id: index as i32 + 1,
            amount_winnings: 0,
        });

        scratched_numbers_str.iter().for_each(|number| {
            if number != &"" {
                if winning_numbers_str.contains(&number) {
                    card_winnings[index].amount_winnings += 1;
                }
            }
        });
    }

    for (index, card) in card_winnings.iter().enumerate() {
        for win in 1..card.amount_winnings + 1 {
            amount_cards[(index as i32 + win as i32) as usize].1 += 1 * amount_cards[index].1;
        }
    }

    println!("Cards:\n{:?}", amount_cards);

    amount_cards.iter().map(|card| card.1).sum::<i32>()
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 30)
    }
}

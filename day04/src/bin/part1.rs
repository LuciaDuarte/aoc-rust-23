fn main() {
    let input = include_str!("input.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();

    let mut values: Vec<i32> = Vec::new();

    for line in lines.iter() {
        let mut card_winnings: Vec<i32> = Vec::new();

        let split_numbers = line.split("|").collect::<Vec<_>>();
        let winning_numbers_str = split_numbers[0].split(":").collect::<Vec<_>>()[1]
            .split(" ")
            .collect::<Vec<_>>();

        let scratched_numbers_str = split_numbers[1].split(" ").collect::<Vec<_>>();

        scratched_numbers_str.iter().for_each(|number| {
            if number != &"" {
                if winning_numbers_str.contains(&number) {
                    let number = number.parse::<i32>().unwrap();
                    card_winnings.push(number);
                }
            }
        });

        let mut value = 0;
        for (index, _) in card_winnings.iter().enumerate() {
            value = if index == 0 { 1 } else { value * 2 };
        }
        values.push(value);
        println!("Value:\n{:?}", value);
    }

    values.iter().sum::<i32>()
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
        assert_eq!(result, 13)
    }
}

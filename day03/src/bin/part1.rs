use regex::Regex;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();

    let number_re: Regex = Regex::new(r"[0-9]+").unwrap();
    let special_re: Regex = Regex::new(r"[^A-z.\s\d][\\\^]?").unwrap();

    let mut parts: Vec<i32> = Vec::new();

    let mut sum: i32 = 0;

    for (index, line) in lines.iter().enumerate() {
        let index = i32::try_from(index).unwrap();
        let numbers: Vec<_> = number_re.find_iter(line.trim()).map(|m| m).collect();

        let mut special_chars: Vec<_> = Vec::new();

        if index < (lines.len() as i32 - 2) as i32 {
            special_chars = special_re
                .find_iter(lines[(index + 1) as usize].trim())
                .map(|m| m)
                .collect();
        }

        for number in numbers {
            let interval = ((number.start() as i32 - 1), (number.end() as i32 + 1));

            for special_char in &special_chars {
                if special_char.start() as i32 >= interval.0
                    && special_char.end() as i32 <= interval.1
                {
                    parts.push(number.as_str().parse::<i32>().unwrap());
                }
            }

            if index > 0 {
                let special_chars: Vec<_> = special_re
                    .find_iter(lines[(index - 1) as usize].trim())
                    .map(|m| m)
                    .collect();

                let interval = ((number.start() as i32 - 1), (number.end() as i32 + 1));

                for special_char in special_chars {
                    if special_char.start() as i32 >= interval.0
                        && special_char.end() as i32 <= interval.1
                    {
                        parts.push(number.as_str().parse::<i32>().unwrap());
                    }
                }
            }

            let special_chars: Vec<_> = special_re
                .find_iter(lines[index as usize].trim())
                .map(|m| m)
                .collect();

            let interval = ((number.start() as i32 - 1), (number.end() as i32 + 1));

            for special_char in special_chars {
                if special_char.start() as i32 >= interval.0
                    && special_char.end() as i32 <= interval.1
                {
                    parts.push(number.as_str().parse::<i32>().unwrap());
                }
            }
        }
    }
    for part in parts.clone() {
        sum += part
    }

    println!("Sum:\n{:?}", parts);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn it_works() {
        let result = part1(
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..",
        );
        assert_eq!(result, 4361)
    }
}

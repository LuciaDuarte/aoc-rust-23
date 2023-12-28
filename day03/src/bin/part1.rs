use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug)]
struct SpecialChar {
    line: i32,
    positions: Vec<i32>,
}

fn solution(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();

    let mut special_chars: Vec<SpecialChar> = Vec::new();

    let mut parts: Vec<i32> = Vec::new();

    // let mut sum: i32 = 0;

    let num_re = Regex::new(r"[0-9]+").unwrap();
    let special_re = Regex::new(r"[^A-z.\s\d][\\\^]?").unwrap();

    for (index, line) in lines.iter().enumerate() {
        let result: Vec<_> = special_re.find_iter(line.trim()).map(|m| m).collect();

        let mut chars_position: Vec<i32> = Vec::new();

        for mat in result {
            chars_position.push(mat.start().try_into().unwrap())
        }

        if chars_position.len() != 0 {
            special_chars.push(SpecialChar {
                line: index.try_into().unwrap(),
                positions: chars_position,
            });
        }
    }

    for (index, line) in lines.iter().enumerate() {
        let numbers: Vec<_> = num_re.find_iter(line.trim()).map(|m| m).collect();

        for number in numbers {
            let interval = ((number.start() as i32 - 1), (number.end() as i32) + 1);

            special_chars
                .iter()
                .filter(|s| {
                    s.line == index.try_into().unwrap()
                        || s.line == (index as i32) - 1
                        || s.line == (index as i32) + 1
                })
                .for_each(|s| {
                    for position in &s.positions {
                        if position >= &interval.0 && position + 1 <= interval.1 {
                            parts.push(number.as_str().parse::<i32>().unwrap())
                        }
                    }
                });
        }
    }

    parts.iter().sum::<i32>()

    // sum
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
            "467..114..
            ...*......
            ..35..598.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..",
        );
        assert_eq!(result, 4326)
    }
}

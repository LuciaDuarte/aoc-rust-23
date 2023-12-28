use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();

    let mut parts_numbers: Vec<i32> = Vec::new();
    let mut parts: Vec<i32> = Vec::new();

    let num_re = Regex::new(r"[0-9]+").unwrap();
    let special_re = Regex::new(r"[*]").unwrap();

    for (index, line) in lines.iter().enumerate() {
        let stars: Vec<_> = special_re.find_iter(line.trim()).map(|m| m).collect();

        if stars.len() == 0 {
            continue;
        }

        for star in stars {
            lines[index - 1..index + 2].iter().for_each(|line| {
                let numbers: Vec<_> = num_re.find_iter(line.trim()).map(|m| m).collect();

                for number in numbers {
                    if star.start() == ((number.start() as i32) - 1) as usize
                        || star.start() == number.end()
                        || star.start() >= (number.start() as i32) as usize
                            && star.end() <= number.end()
                    {
                        parts_numbers.push(number.as_str().parse::<i32>().unwrap())
                    }
                }
            });

            if parts_numbers.len() == 2 {
                parts.push(parts_numbers[0] * parts_numbers[1]);
            }

            parts_numbers.clear();
        }
    }

    parts.iter().sum::<i32>()
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
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
        assert_eq!(result, 467835)
    }
}

use std::i32;

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct CalibrationIndex {
    value: char,
    index: usize,
}

fn part2(input: &str) -> String {
    let input = input.lines().into_iter();

    let mut sum: i32 = 0;

    for value in input {
        let mut calibration_vec: Vec<CalibrationIndex> = Vec::new();

        for (index, char) in value.chars().enumerate() {
            if char.is_numeric() {
                calibration_vec.push({
                    CalibrationIndex {
                        value: char,
                        index: index,
                    }
                });
            }
        }

        let digits_vec = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        for digit in digits_vec {
            let indexes: Vec<_> = value.match_indices(digit).map(|(i, _)| i).collect();

            for i in indexes {
                if validate_digits(digit) != 'x' {
                    calibration_vec.push({
                        CalibrationIndex {
                            value: validate_digits(digit),
                            index: i,
                        }
                    });
                }
            }
        }

        calibration_vec.sort_by(|a, b| a.index.cmp(&b.index));

        let calibration_value: String = calibration_vec[0].value.to_string()
            + &calibration_vec[calibration_vec.len() - 1].value.to_string();
        sum = sum + calibration_value.parse::<i32>().unwrap();
    }
    sum.to_string()
}

fn validate_digits(val: &str) -> char {
    match val {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => 'x',
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn it_works() {
        let result = part2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            sevendxbninefour2fourclmln",
        );
        assert_eq!(result, "355")
    }
}

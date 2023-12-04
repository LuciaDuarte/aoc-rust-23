fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let input = input.lines().into_iter();

    let mut sum: i32 = 0;

    for value in input {
        let mut current_calibration: String = String::from("");

        for char in value.chars() {
            if char.is_numeric() {
                current_calibration.push(char);
                break;
            }
        }
        for char in value.chars().rev() {
            if char.is_numeric() {
                current_calibration.push(char);
                break;
            }
        }

        sum = sum + current_calibration.parse::<i32>().unwrap();
        println!("Sum:\n{:?}", sum);
    }
    sum.to_string()
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(result, "142")
    }
}

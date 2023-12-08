fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let lines = input.lines().into_iter();

    let mut sum: i32 = 0;

    for line in lines {
        let mut max_red: i32 = 0;
        let mut max_green: i32 = 0;
        let mut max_blue: i32 = 0;

        let splitted_game: Vec<_> = line.trim().split(":").collect();

        let rounds: Vec<_> = splitted_game[1].split(";").collect();

        for round in rounds {
            let cubes: Vec<_> = round.split(",").collect();

            for cube in cubes {
                let cube: Vec<_> = cube.trim().split(" ").collect();

                let amount = cube[0].parse::<i32>().unwrap();

                match cube[1] {
                    "red" => {
                        if amount > max_red {
                            max_red = amount;
                        }
                    }
                    "green" => {
                        if amount > max_green {
                            max_green = amount;
                        }
                    }
                    "blue" => {
                        if amount > max_blue {
                            max_blue = amount;
                        }
                    }
                    _ => break,
                }
            }
        }

        sum += max_blue * max_green * max_red;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn it_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286")
    }
}

fn main() {
    let input = include_str!("input.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> i32 {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let lines = input.lines().into_iter();

    let mut sum: i32 = 0;

    for line in lines {
        let mut valid_game: bool = true;
        let splitted_game: Vec<_> = line.trim().split(":").collect();
        let game_id = splitted_game[0].split(" ").collect::<Vec<_>>()[1]
            .parse::<i32>()
            .unwrap();

        let rounds: Vec<_> = splitted_game[1].split(";").collect();

        for round in rounds {
            let cubes: Vec<_> = round.split(",").collect();

            for cube in cubes {
                let cube: Vec<_> = cube.trim().split(" ").collect();

                let amount = cube[0].parse::<i32>().unwrap();

                match cube[1] {
                    "red" => {
                        if amount > MAX_RED {
                            valid_game = false;
                            break;
                        }
                    }
                    "green" => {
                        if amount > MAX_GREEN {
                            valid_game = false;
                            break;
                        }
                    }
                    "blue" => {
                        if amount > MAX_BLUE {
                            valid_game = false;
                            break;
                        }
                    }
                    _ => break,
                }
            }

            if !valid_game {
                break;
            }
        }

        if valid_game {
            sum += game_id;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn it_works() {
        let result = solution(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8)
    }
}

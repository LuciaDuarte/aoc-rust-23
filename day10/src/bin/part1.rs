use std::primitive;

fn main() {
    let input = include_str!("input.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug)]
struct Pipe {
    pipe: char,
    connects_to: Vec<char>,
}

#[derive(Debug, Clone)]
struct Coordinate {
    // horizontal
    x: i32,
    // vertical
    y: i32,
}

fn solution(input: &str) -> i64 {
    let lines: Vec<_> = input.lines().collect();

    let available_pipes: Vec<Pipe> = vec![
        Pipe {
            pipe: '|',
            connects_to: vec!['N', 'S'],
        },
        Pipe {
            pipe: '-',
            connects_to: vec!['W', 'E'],
        },
        Pipe {
            pipe: 'L',
            connects_to: vec!['N', 'E'],
        },
        Pipe {
            pipe: 'J',
            connects_to: vec!['N', 'W'],
        },
        Pipe {
            pipe: '7',
            connects_to: vec!['W', 'S'],
        },
        Pipe {
            pipe: 'F',
            connects_to: vec!['E', 'S'],
        },
    ];

    let grid: Vec<Vec<char>> = generate_grid(lines);

    let starting_pos = find_starting_point(&grid);

    let (mut next_pos, mut from) = check_initial_boundaries(&starting_pos, &grid, &available_pipes);

    let mut steps = 0;
    let mut pipe = '*';

    while pipe != 'S' {
        // println!("next:\n{:?}", next_pos);
        steps += 1;
        (next_pos, from) = find_next(&next_pos, &from, &grid);
        pipe = get_pipe(&next_pos, &grid);
    }
    // 6942
    (steps + 1) / 2
}

fn generate_grid(lines: Vec<&str>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    lines.iter().for_each(|line| {
        grid.push(line.chars().filter(|char| *char != ' ').collect::<Vec<_>>());
    });

    grid
}

fn find_starting_point(grid: &Vec<Vec<char>>) -> Coordinate {
    let mut coordinate: Coordinate = Coordinate { x: 0, y: 0 };

    for (index, row) in grid.iter().enumerate() {
        // println!("row:\n{:?}", row);

        let found_s = row.iter().position(|char| *char == 'S');

        match found_s {
            Some(x) => {
                coordinate = Coordinate {
                    x: x.try_into().unwrap(),
                    y: index.try_into().unwrap(),
                }
            }
            None => continue,
        }
    }

    // println!("coordinate:\n{:?}", coordinate);
    coordinate
}

fn check_north(coordinate: &Coordinate, grid: &Vec<Vec<char>>, pipes: &Vec<Pipe>) -> bool {
    if coordinate.y == 0 {
        return false;
    }

    let north_cell = grid
        .get::<usize>((coordinate.y - 1).try_into().unwrap())
        .unwrap()
        .get::<usize>(coordinate.x.try_into().unwrap())
        .unwrap();

    // println!("north_cell:\n{:?}", north_cell);

    if *north_cell == '.' {
        return false;
    }

    let valid = pipes
        .iter()
        .any(|pipe| pipe.pipe == *north_cell && pipe.connects_to.contains(&'S'));

    // println!("valid:\n{:?}", valid);
    valid
}

fn check_south(coordinate: &Coordinate, grid: &Vec<Vec<char>>, pipes: &Vec<Pipe>) -> bool {
    // println!("grid len:\n{:?}", grid.len());
    if coordinate.y == grid.len() as i32 - 1 {
        return false;
    }

    let south_cell = grid
        .get::<usize>((coordinate.y + 1).try_into().unwrap())
        .unwrap()
        .get::<usize>(coordinate.x.try_into().unwrap())
        .unwrap();

    // println!("south_cell:\n{:?}", south_cell);

    if *south_cell == '.' {
        return false;
    }

    let valid = pipes
        .iter()
        .any(|pipe| pipe.pipe == *south_cell && pipe.connects_to.contains(&'N'));

    // println!("valid:\n{:?}", valid);
    valid
}

fn check_west(coordinate: &Coordinate, grid: &Vec<Vec<char>>, pipes: &Vec<Pipe>) -> bool {
    if coordinate.x == 0 {
        return false;
    }

    let west_cell = grid
        .get::<usize>(coordinate.y.try_into().unwrap())
        .unwrap()
        .get::<usize>((coordinate.x - 1).try_into().unwrap())
        .unwrap();

    // println!("west_cell:\n{:?}", west_cell);

    if *west_cell == '.' {
        return false;
    }

    let valid = pipes
        .iter()
        .any(|pipe| pipe.pipe == *west_cell && pipe.connects_to.contains(&'E'));

    // println!("valid:\n{:?}", valid);
    valid
}

fn check_east(coordinate: &Coordinate, grid: &Vec<Vec<char>>, pipes: &Vec<Pipe>) -> bool {
    if coordinate.y == grid[coordinate.y as usize].len() as i32 - 1 {
        return false;
    }

    let east_cell = grid
        .get::<usize>(coordinate.x.try_into().unwrap())
        .unwrap()
        .get::<usize>((coordinate.y + 1).try_into().unwrap())
        .unwrap();

    // println!("east_cell:\n{:?}", east_cell);

    if *east_cell == '.' {
        return false;
    }

    let valid = pipes
        .iter()
        .any(|pipe| pipe.pipe == *east_cell && pipe.connects_to.contains(&'E'));

    // println!("valid:\n{:?}", valid);
    valid
}

fn check_initial_boundaries(
    coordinate: &Coordinate,
    grid: &Vec<Vec<char>>,
    pipes: &Vec<Pipe>,
) -> (Coordinate, char) {
    let next_coord: Coordinate;

    if check_north(&coordinate, &grid, &pipes) {
        next_coord = Coordinate {
            x: coordinate.x,
            y: coordinate.y - 1,
        };

        return (next_coord, 'S');
    } else if check_south(&coordinate, &grid, &pipes) {
        next_coord = Coordinate {
            x: coordinate.x,
            y: coordinate.y + 1,
        };

        return (next_coord, 'N');
    } else if check_west(&coordinate, &grid, &pipes) {
        next_coord = Coordinate {
            x: coordinate.x - 1,
            y: coordinate.y,
        };

        return (next_coord, 'E');
    } else {
        next_coord = Coordinate {
            x: coordinate.x + 1,
            y: coordinate.y,
        };

        return (next_coord, 'W');
    }

    // check_east(&coordinate, &grid, &pipes);
}

fn find_next(coordinate: &Coordinate, from: &char, grid: &Vec<Vec<char>>) -> (Coordinate, char) {
    let pipe = get_pipe(&coordinate, &grid);
    // println!("from:{:?} pipe: {:?}", from, pipe);
    if *from == 'N' {
        match pipe {
            'L' => (
                Coordinate {
                    x: coordinate.x + 1,
                    y: coordinate.y,
                },
                'W',
            ),
            '|' => (
                Coordinate {
                    x: coordinate.x,
                    y: coordinate.y + 1,
                },
                'N',
            ),
            'J' => (
                Coordinate {
                    x: coordinate.x - 1,
                    y: coordinate.y,
                },
                'E',
            ),
            _ => (Coordinate { x: 0, y: 0 }, '*'),
        }
    } else if *from == 'S' {
        match pipe {
            '7' => (
                Coordinate {
                    x: coordinate.x - 1,
                    y: coordinate.y,
                },
                'E',
            ),
            '|' => (
                Coordinate {
                    x: coordinate.x,
                    y: coordinate.y - 1,
                },
                'S',
            ),
            'F' => (
                Coordinate {
                    x: coordinate.x + 1,
                    y: coordinate.y,
                },
                'W',
            ),
            _ => (Coordinate { x: 0, y: 0 }, '*'),
        }
    } else if *from == 'W' {
        match pipe {
            '-' => (
                Coordinate {
                    x: coordinate.x + 1,
                    y: coordinate.y,
                },
                'W',
            ),
            'J' => (
                Coordinate {
                    x: coordinate.x,
                    y: coordinate.y - 1,
                },
                'S',
            ),
            '7' => (
                Coordinate {
                    x: coordinate.x,
                    y: coordinate.y + 1,
                },
                'N',
            ),
            _ => (Coordinate { x: 0, y: 0 }, '*'),
        }
    } else {
        match pipe {
            '-' => (
                Coordinate {
                    x: coordinate.x - 1,
                    y: coordinate.y,
                },
                'E',
            ),
            'L' => (
                Coordinate {
                    x: coordinate.x,
                    y: coordinate.y - 1,
                },
                'S',
            ),
            'F' => (
                Coordinate {
                    x: coordinate.x,
                    y: coordinate.y + 1,
                },
                'N',
            ),
            _ => (Coordinate { x: 0, y: 0 }, '*'),
        }
    }
}

fn get_pipe(coordinate: &Coordinate, grid: &Vec<Vec<char>>) -> char {
    let pipe = grid
        .get::<usize>(coordinate.y.try_into().unwrap())
        .unwrap()
        .get::<usize>((coordinate.x).try_into().unwrap())
        .unwrap();

    *pipe
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result1 = solution(
            ".....
          .S-7.
          .|.|.
          .L-J.
          .....",
        );
        assert_eq!(result1, 4);
        let result2 = solution(
            "-L|F7
          7S-7|
          L|7||
          -L-J|
          L|-JF",
        );
        assert_eq!(result2, 4);
        let result3 = solution(
            "..F7.
          .FJ|.
          SJ.L7
          |F--J
          LJ...",
        );
        assert_eq!(result3, 8);
        let result4 = solution(
            "7-F7-
          .FJ|7
          SJLL7
          |F--J
          LJ.LJ",
        );
        assert_eq!(result4, 8);
    }
}

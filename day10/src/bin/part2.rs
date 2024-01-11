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

#[derive(Debug, Clone, PartialEq)]
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

    let mut grid: Vec<Vec<char>> = generate_grid(lines);
    let mut grid_coord: Vec<Coordinate> = Vec::new();

    let starting_pos = find_starting_point(&grid);
    grid_coord.push(starting_pos.clone());

    let (mut next_pos, mut from) = check_initial_boundaries(&starting_pos, &grid, &available_pipes);

    let mut pipe = '*';

    while pipe != 'S' {
        grid_coord.push(next_pos.clone());
        (next_pos, from) = find_next(&next_pos, &from, &grid);
        pipe = get_pipe(&next_pos, &grid);
    }

    let updated_grid = remove_trash_pipes(&mut grid, &grid_coord);

    count_in_out(&updated_grid)
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

fn remove_trash_pipes(grid: &mut Vec<Vec<char>>, grid_coord: &Vec<Coordinate>) -> Vec<Vec<char>> {
    for (index, row) in grid.iter_mut().enumerate() {
        for (i, char) in row.iter_mut().enumerate() {
            if *char == 'S' {
                let fist_coor = grid_coord.first().unwrap();
                let second_coord = grid_coord.get(1).unwrap();
                let last_coord = grid_coord.last().unwrap();

                if fist_coor.x == i as i32 && fist_coor.y == index as i32 {
                    *char = 'F';
                } else if second_coord.x == i as i32 && second_coord.y == index as i32 {
                    *char = 'J';
                } else if last_coord.x == i as i32 && last_coord.y == index as i32 {
                    *char = 'L';
                } else {
                    *char = '|';
                }
            }
            if !grid_coord.contains(&Coordinate {
                x: i as i32,
                y: index as i32,
            }) {
                *char = '.';
            }
        }
        // println!("row:\n{:?}", row);
    }

    println!("grid new:\n{:?}", grid);
    grid.clone()
}

fn count_in_out(grid: &Vec<Vec<char>>) -> i64 {
    let mut amount_tiles = 0;

    for (index, row) in grid.iter().enumerate() {
        // println!("row {:?}:\n{:?}", index, row);
        let mut last_valid = row[0];
        let mut is_in = false;
        let mut row_tiles = 0;

        // | is obviously a wall
        // - is not a wall
        // FJ is wall: (F----J) too
        //     |
        //  ---
        //  |
        // L7 is also wall: (L----7) too
        //  |
        //  ---
        //     |
        // F7 is a loop and LJ too:
        //  ----        |   |
        //  |  |        |   |
        //  |  |        -----

        for char in row.iter() {
            if *char == '|' {
                is_in = !is_in;
                amount_tiles += row_tiles;
                row_tiles = 0;
            } else if *char == 'F' || *char == 'L' {
                last_valid = *char;
            } else if (last_valid == 'F' && *char == 'J') || (last_valid == 'L' && *char == '7') {
                is_in = !is_in;
                amount_tiles += row_tiles;
                row_tiles = 0;
                last_valid = *char;
            } else if *char == '.' && is_in {
                row_tiles += 1;
            }
        }

        // println!("end row {:?}: total {:?}", index, amount_tiles);
    }

    amount_tiles
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result1 = solution(
            "...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........",
        );
        assert_eq!(result1, 4);

        let result2 = solution(
            ".F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...",
        );
        assert_eq!(result2, 8);

        let result3 = solution(
            "FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJIF7FJ-
            L---JF-JLJIIIIFJLJJ7
            |F|F-JF---7IIIL7L|7|
            |FFJF7L7F-JF7IIL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L",
        );
        assert_eq!(result3, 10);
    }
}

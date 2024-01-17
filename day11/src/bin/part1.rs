fn main() {
    let input = include_str!("input.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Galaxy {
    id: usize,
    coordinates: Coordinate,
}

#[derive(Debug, Clone)]
struct GalaxyDistance {
    from: usize,
    to: usize,
    shortest_distance: u32,
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

    let universe: Vec<Vec<char>> = generate_grid(lines);

    let (empty_rows, empty_cols) = find_empty(&universe);

    let galaxies_coord: Vec<Galaxy> = find_galaxies(&universe);

    // println!("expanded:\n{:?}", expanded);
    let shortest_distances = find_shortest_path(&galaxies_coord, empty_rows, empty_cols);

    let sum: i64 = shortest_distances
        .iter()
        .map(|galaxy| galaxy.shortest_distance as i64)
        .sum();

    sum
}

fn generate_grid(lines: Vec<&str>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    lines.iter().for_each(|line| {
        grid.push(line.chars().filter(|char| *char != ' ').collect::<Vec<_>>());
    });

    grid
}

fn find_empty(universe: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut expanded = universe.clone();
    let mut rows_i: Vec<usize> = Vec::new();
    let mut cols_i: Vec<usize> = Vec::new();

    for (index, row) in universe.iter().enumerate() {
        let without_galaxies = row.iter().all(|char| *char != '#');

        if without_galaxies {
            rows_i.push(index);
        }
    }

    let mut current_col = 0;
    while current_col < universe[0].len() - 1 {
        // println!("current_col:\n{:?}", current_col);
        for i in 0..universe.len() {
            if universe[i][current_col] == '#' {
                if current_col < universe.len() - 1 {
                    current_col += 1;
                }
                continue;
            }

            if i == universe.len() - 1 {
                cols_i.push(current_col);
                current_col += 1;
            }
        }
    }

    (rows_i, cols_i)
}

// fn expand_universe(universe: &Vec<Vec<char>>) -> Vec<Vec<char>> {
//     let mut expanded = universe.clone();
//     let mut rows_i: Vec<usize> = Vec::new();
//     let mut cols_i: Vec<usize> = Vec::new();

//     for (index, row) in universe.iter().enumerate() {
//         let without_galaxies = row.iter().all(|char| *char != '#');

//         if without_galaxies {
//             rows_i.push(index);
//         }
//     }

//     let mut current_col = 0;
//     while current_col < universe[0].len() - 1 {
//         // println!("current_col:\n{:?}", current_col);
//         for i in 0..universe.len() {
//             // println!("char:\n{:?}", universe[i][current_col]);

//             if universe[i][current_col] == '#' {
//                 if current_col < universe.len() {
//                     current_col += 1;
//                 }
//                 continue;
//             }

//             if i == universe.len() - 1 {
//                 cols_i.push(current_col);
//                 current_col += 1;
//             }
//         }
//     }

//     for (i, row_i) in rows_i.iter().enumerate() {
//         expanded.insert(row_i + i, vec!['.'; universe[0].len()]);
//     }

//     for (i, col_i) in cols_i.iter().enumerate() {
//         for row in expanded.iter_mut() {
//             row.insert(col_i + i, '.');
//         }
//     }

//     expanded
// }

fn find_galaxies(universe: &Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut galaxies_coordinates: Vec<Galaxy> = Vec::new();

    for (index, row) in universe.iter().enumerate() {
        row.iter().enumerate().for_each(|(i, char)| {
            if *char == '#' {
                galaxies_coordinates.push(Galaxy {
                    id: galaxies_coordinates.len() + 1,
                    coordinates: Coordinate {
                        x: i.try_into().unwrap(),
                        y: index.try_into().unwrap(),
                    },
                })
            }
        });
    }

    // println!("galaxies:\n{:?}", galaxies_coordinates);

    galaxies_coordinates
}

fn find_shortest_path(
    galaxies: &Vec<Galaxy>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
) -> Vec<GalaxyDistance> {
    let mut shortest_distances: Vec<GalaxyDistance> = Vec::new();
    println!("empty rows:\n{:?}", empty_rows);
    println!("empty cols:\n{:?}", empty_cols);

    for galaxy in galaxies {
        for other_galaxy in galaxies {
            if galaxy.id >= other_galaxy.id {
                continue;
            }

            shortest_distances.push(GalaxyDistance {
                from: galaxy.id,
                to: other_galaxy.id,
                shortest_distance: calculate_shortest_distance(
                    galaxy,
                    other_galaxy,
                    &empty_rows,
                    &empty_cols,
                ),
            })
        }
    }

    println!("shortest:\n{:?}", shortest_distances.len()); // 92235
    println!("shortest:\n{:?}", shortest_distances.len());
    shortest_distances
}

fn calculate_shortest_distance(
    galaxy: &Galaxy,
    other_galaxy: &Galaxy,
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
) -> u32 {
    println!("galaxy:\n{:?}", galaxy);
    println!("other galaxy: \n{:?}", other_galaxy);

    let mut x = galaxy.coordinates.x.abs_diff(other_galaxy.coordinates.x);
    let mut y = galaxy.coordinates.y.abs_diff(other_galaxy.coordinates.y);
    println!("x:{:?} y:{:?}", x, y);

    for empty_row in empty_rows {
        if other_galaxy.coordinates.y.min(galaxy.coordinates.y) < *empty_row as i32
            && other_galaxy.coordinates.y.max(galaxy.coordinates.y) > *empty_row as i32
        {
            println!("empty row:\n{:?}", empty_row);
            y += 1;
        }
    }

    for empty_col in empty_cols {
        if other_galaxy.coordinates.x.min(galaxy.coordinates.x) < *empty_col as i32
            && other_galaxy.coordinates.x.max(galaxy.coordinates.x) > *empty_col as i32
        {
            println!("empty col:\n{:?}", empty_col);
            x += 1;
        }
    }

    x + y
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
            "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....",
        );
        assert_eq!(result, 374);
    }
}

// 10325819
// 10337801
// 10310424

fn main() {
    let input = include_str!("input.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
struct SeedMap {
    source: (i128, i128),
    destination: i128,
}

fn solution(input: &str) -> i128 {
    // splitting by : to get the groups of maps
    let lines: Vec<_> = input.split(":").collect();

    let mut mappings: Vec<Vec<SeedMap>> = Vec::new();
    let mut seed_map: Vec<SeedMap> = Vec::new();
    let mut locations: Vec<i128> = Vec::new();

    // getting the seeds
    let seeds = lines[1].split("\n").collect::<Vec<_>>()[0]
        .trim()
        .split(" ")
        .collect::<Vec<_>>();
    println!("Seeds:\n{:?}", seeds);

    // removing the first 2 lines: seeds and seed numbers because already have it
    let mut seeds_maps = vec![""; lines.len() - 2];

    seeds_maps.copy_from_slice(&lines[2..lines.len()]);

    for line in seeds_maps.iter() {
        let map_lines: Vec<_> = line
            .split_whitespace()
            // removes the words
            .filter(|m| !m.contains("-") && !m.contains("map"))
            .collect();

        let mut last_current = 2;

        for (index, _) in map_lines.iter().enumerate() {
            if index == map_lines.len() - 1 {
                mappings.push(seed_map.clone());
                seed_map.clear();
                continue;
            }

            if index <= last_current && index != 0 {
                continue;
            }

            last_current = index + 2;

            let destination: i128 = map_lines[index].parse().unwrap();
            let source: i128 = map_lines[index + 1].parse().unwrap();
            let range: i128 = map_lines[index + 2].parse().unwrap();

            seed_map.push(SeedMap {
                source: (source, source + (range - 1)),
                destination: destination,
            });
        }
    }

    // iterate over the seeds and mapping them to their next value
    for seed in &seeds {
        // println!("Seed!:\n{:?}", seed);

        let mut final_value = seed.parse::<i128>().unwrap();
        println!("seed!:\n{:?}", final_value);

        for map in &mappings {
            for val in map {
                if final_value >= val.source.0 && final_value <= val.source.1 {
                    final_value = final_value - val.source.0 + val.destination;
                    break;
                }
            }
        }

        // the location is the final value so keep all the locations
        locations.push(final_value);
    }

    locations.sort();

    println!("Locations:\n{:?}", locations[0]);

    locations[0]
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // destination, source, range
        let result = solution(
            "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48
            
            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15
            
            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4
            
            water-to-light map:
            88 18 7
            18 25 70
            
            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13
            
            temperature-to-humidity map:
            0 69 1
            1 0 69
            
            humidity-to-location map:
            60 56 37
            56 93 4",
        );
        assert_eq!(result, 35)
    }
}

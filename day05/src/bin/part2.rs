fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
struct SeedMap {
    source: (i128, i128),
    destination: i128,
}

fn part2(input: &str) -> i128 {
    // splitting by : to get the groups of maps
    let lines: Vec<_> = input.split(":").collect();

    let mut mappings: Vec<Vec<SeedMap>> = Vec::new();
    let mut seed_map: Vec<SeedMap> = Vec::new();
    let mut locations: Vec<i128> = Vec::new();
    let seeds_range: Vec<(i128, i128)> = get_seeds_range(lines[1]);
    let mut mappings_ranges: Vec<Vec<(i128, i128)>> = Vec::new();

    println!("SeedsRange:\n{:?}", seeds_range);

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
                seed_map.sort_by(|a, b| a.source.0.cmp(&b.source.0));
                mappings.push(seed_map.clone());
                seed_map.clear();
                continue;
            }

            if index <= last_current && index != 0 {
                continue;
            }

            last_current = index + 2;

            let destination: i128 = map_lines[index + 1].parse().unwrap();
            let source: i128 = map_lines[index].parse().unwrap();
            let range: i128 = map_lines[index + 2].parse().unwrap();

            seed_map.push(SeedMap {
                source: (source, source + (range - 1)),
                destination: destination,
            });
        }
    }

    for map in mappings.as_slice() {
        let mut map_range: Vec<(i128, i128)> = Vec::new();
        for val in map {
            map_range.push(val.source)
        }

        if map[0].source.0 != 0 {
            map_range.insert(0, (0, map[0].source.0 - 1))
        }

        mappings_ranges.push(map_range.clone());
        map_range.clear()
    }
    println!("MappingsRange!:\n{:?}", mappings_ranges);

    // this solution does not always work, but I was desperate
    // a lot of things were should be better, but I was desperate
    for location in mappings_ranges.last().unwrap() {
        mappings.reverse();

        let mut final_value = 0;
        let mut is_found = false;
        for i in location.0..=location.1 {
            final_value = i;

            println!("location!:\n{:?}", i);

            for map in &mappings {
                for val in map {
                    if final_value >= val.source.0 && final_value <= val.source.1 {
                        final_value = final_value - val.source.0 + val.destination;
                        break;
                    }
                }
            }
            println!("initial seed!:\n{:?}", final_value);

            let found = seeds_range
                .iter()
                .find(|range| final_value >= range.0 && final_value <= range.1);

            println!("found\n{:?}", found);

            match found {
                Some(_) => {
                    is_found = true;
                    locations.push(i);
                    break;
                }
                None => (),
            }
        }

        if is_found {
            break;
        }
    }

    println!("Locations:\n{:?}", locations);

    locations[0]
}

fn get_seeds_range(seed_line: &str) -> Vec<(i128, i128)> {
    let mut seeds_range: Vec<(i128, i128)> = Vec::new();

    // getting the seeds
    let seeds = seed_line.split("\n").collect::<Vec<_>>()[0]
        .trim()
        .split(" ")
        .map(|n| n.trim().parse::<i128>().unwrap())
        .collect::<Vec<_>>();
    // println!("Seeds:\n{:?}", seeds);

    for (i, _) in seeds.iter().enumerate() {
        if i % 2 == 0 {
            seeds_range.push((seeds[i], seeds[i] + seeds[i + 1] - 1))
        }
    }

    seeds_range.sort_by(|a, b| a.0.cmp(&b.0));
    seeds_range
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // destination, source, range
        let result = part2(
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
        assert_eq!(result, 46)
    }
}

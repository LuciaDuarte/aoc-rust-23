fn main() {
    let input = include_str!("input.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug)]
struct RaceRecords {
    time: u32,
    distance: u32,
}

fn solution(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    let race_records: Vec<RaceRecords> = get_race_records(lines);
    let mut ways_of_winning: Vec<u32> = Vec::new();

    race_records.iter().for_each(|record| {
        let mut would_win = 0;
        for ms in 1..record.time {
            if ms * (record.time - ms) > record.distance {
                would_win += 1;
            }
        }
        ways_of_winning.push(would_win);
    });

    ways_of_winning.iter().fold(1, |acc, c| acc * c)
}

fn get_race_records(lines: Vec<&str>) -> Vec<RaceRecords> {
    let mut race_records: Vec<RaceRecords> = Vec::new();
    for line in lines {
        if line.contains("Time") {
            line.split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .skip(1)
                .for_each(|number| {
                    race_records.push(RaceRecords {
                        time: number.parse::<u32>().unwrap(),
                        distance: 0,
                    })
                });
        } else {
            line.split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .skip(1)
                .enumerate()
                .for_each(|(index, number)| {
                    race_records[index].distance = number.parse::<u32>().unwrap()
                });
        }
    }
    race_records
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
            "Time:      7  15   30
            Distance:  9  40  200",
        );
        assert_eq!(result, 288)
    }
}

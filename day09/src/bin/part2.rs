fn main() {
    let input = include_str!("input.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug)]
struct HistoryReport {
    original_values: Vec<i64>,
    next_sequences: Vec<Vec<i64>>,
    predictions: Vec<i64>,
}

fn solution(input: &str) -> i64 {
    let lines: Vec<_> = input.lines().collect();

    let mut history_report: Vec<HistoryReport> = get_report(lines);

    populate_next_sequences(&mut history_report);

    predict_next_value(&mut history_report);

    // println!("report:\n{:?}", history_report);

    let sum = history_report
        .iter()
        .map(|report| report.predictions.last().unwrap())
        .sum();

    sum
}

fn predict_next_value(report: &mut Vec<HistoryReport>) {
    report.iter_mut().for_each(|report_value| {
        let mut all_sequences = report_value.next_sequences.clone();
        all_sequences.reverse();
        all_sequences.push(report_value.original_values.clone());

        all_sequences
            .iter()
            .skip(1)
            .enumerate()
            .for_each(|(i, sequence)| {
                if i == 0 {
                    report_value.predictions.push(*sequence.first().unwrap());
                } else {
                    let last_val = report_value.predictions.last().unwrap();
                    report_value
                        .predictions
                        .push(sequence.first().unwrap() - last_val);
                }
            })
    })
}

fn populate_next_sequences(report: &mut Vec<HistoryReport>) {
    report
        .iter_mut()
        .for_each(|report_value: &mut HistoryReport| {
            let mut sequence = Vec::new();

            report_value
                .original_values
                .iter()
                .enumerate()
                .for_each(|(i, val)| {
                    if report_value.original_values.len() - 1 > i {
                        sequence.push(report_value.original_values[i + 1] - val);
                    }
                });

            report_value.next_sequences.push(sequence);
        });

    report.iter_mut().for_each(|report_value| {
        while !report_value
            .next_sequences
            .last()
            .unwrap()
            .iter()
            .all(|val| *val == 0)
        {
            let last_sequence = report_value.next_sequences.last().unwrap();

            let mut sequence = Vec::new();

            last_sequence.iter().enumerate().for_each(|(i, val)| {
                if last_sequence.len() - 1 > i {
                    sequence.push(last_sequence[i + 1] - val);
                }
            });

            report_value.next_sequences.push(sequence);
        }
    })
}

fn get_report(lines: Vec<&str>) -> Vec<HistoryReport> {
    let mut report = Vec::new();

    lines.iter().for_each(|line| {
        let values: Vec<_> = line.split_whitespace().collect();

        report.push(HistoryReport {
            original_values: values.iter().map(|v| v.parse().unwrap()).collect(),
            next_sequences: vec![],
            predictions: vec![],
        })
    });

    report
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
            "0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45",
        );
        assert_eq!(result, 2);
    }
}

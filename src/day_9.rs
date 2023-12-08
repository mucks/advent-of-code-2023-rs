fn solve_seq(seq: &[isize], task: usize) -> isize {
    let mut seqs = vec![seq.to_vec()];

    let mut last_seq = seq.to_vec();
    let mut new_seq = vec![];

    loop {
        new_seq = vec![];
        for i in 0..last_seq.len() - 1 {
            let diff = last_seq[i + 1] - last_seq[i];
            new_seq.push(diff);
        }

        if new_seq.iter().all(|s| s == &0) {
            break;
        }

        seqs.push(new_seq.clone());
        last_seq = new_seq.clone();
    }

    if task == 1 {
        for i in (1..seqs.len()).rev() {
            *seqs[i - 1].last_mut().unwrap() += *seqs[i].last().unwrap();
        }

        let result = *seqs.first().unwrap().last().unwrap();

        return result;
    }

    for i in (1..seqs.len()).rev() {
        *seqs[i - 1].first_mut().unwrap() -= *seqs[i].first().unwrap();
    }

    let result = *seqs.first().unwrap().first().unwrap();

    result
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|c| c.parse().ok())
                .collect()
        })
        .collect()
}

fn task_1(input: &str) -> isize {
    let sequences = parse_input(input);
    sequences.iter().map(|s| solve_seq(s, 1)).sum()
}

fn task_2(input: &str) -> isize {
    let sequences = parse_input(input);
    sequences.iter().map(|s| solve_seq(s, 2)).sum()
}

#[cfg(test)]
mod tests {
    use super::{task_1, task_2};

    fn input() -> String {
        std::fs::read_to_string("./data/day_9.txt").unwrap()
    }
    fn sample_input() -> String {
        std::fs::read_to_string("./data/day_9_sample.txt").unwrap()
    }

    #[test]
    fn test_day_9_task_1() {
        let sample_result = task_1(&sample_input());
        assert_eq!(sample_result, 114);

        let result = task_1(&input());
        println!("the solution for day 9, task 1 is: {result}",);

        assert_eq!(result, 1819125966);
    }

    #[test]
    fn test_day_9_task_2() {
        let sample_result = task_2(&sample_input());
        assert_eq!(sample_result, 2);

        let result = task_2(&input());
        println!("the solution for day 9, task 2 is: {result}",);
    }
}

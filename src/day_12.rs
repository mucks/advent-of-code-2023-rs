fn solve_spring_line(mut springs: Vec<char>, mut arrng: Vec<usize>, unfold: bool) -> usize {
    if unfold {
        let springs_copy = springs.clone();
        let arrng_copy = arrng.clone();
        for i in 0..5 {
            springs.push('?');
            springs.extend(&springs_copy);
            arrng.extend(&arrng_copy);
        }

        println!("springs: {springs:?}");
        println!("arrng: {arrng:?}");
    }

    let mut pairs = vec![];
    let mut pair = vec![];
    for c in &springs {
        if c == &'#' || c == &'?' {
            pair.push(c);
        } else {
            if !pair.is_empty() {
                pairs.push(pair);
                pair = vec![];
            }
        }
    }

    println!("pairs: {pairs:?}");

    let qms: Vec<usize> = springs
        .iter()
        .enumerate()
        .filter(|(i, c)| c == &&'?')
        .map(|(i, _)| i)
        .collect();

    let mut springs_as_bools: Vec<bool> = springs.iter().map(|s| s == &'#').collect();

    let mut arrng_matches = 0;

    let total_combinations: usize = 2usize.pow(qms.len() as u32);
    println!("total combinations: {total_combinations}");

    for (mut i) in 0..total_combinations {
        if i % 1000000 == 0 {
            println!("i: {i}");
        }
        let combination = (0..qms.len())
            .map(|_| {
                let bit = i & 1 == 1;
                i >>= 1;
                bit
            })
            .collect::<Vec<bool>>();

        arrng_matches += solve_combination(&arrng, &qms, &mut springs_as_bools, &combination);
    }

    arrng_matches
}

fn solve_combination(
    arrng: &[usize],
    qms: &[usize],
    springs: &mut [bool],
    combination: &[bool],
) -> usize {
    for (qmi, comb) in qms.iter().zip(combination) {
        springs[*qmi] = *comb;
    }

    let mut broken_pairs = vec![];
    let mut broken_pair = 0;
    for s in springs {
        if *s {
            broken_pair += 1;
        } else if broken_pair > 0 {
            broken_pairs.push(broken_pair);
            broken_pair = 0;
        }
    }
    if broken_pair > 0 {
        broken_pairs.push(broken_pair);
    }

    return (arrng == broken_pairs) as usize;
}

fn generate_combinations(length: usize) -> Vec<Vec<bool>> {
    let total_combinations = 1 << length; // Calculate the total number of combinations

    (0..total_combinations)
        .map(|mut i| {
            (0..length)
                .map(|_| {
                    let bit = i & 1 == 1;
                    i >>= 1;
                    bit
                })
                .collect::<Vec<bool>>()
        })
        .collect()
}

fn task(input: &str, unfold: bool) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let springs: Vec<char> = split.next().unwrap().chars().collect();
        let arrangments: Vec<usize> = split
            .next()
            .unwrap()
            .split(",")
            .filter_map(|c| c.to_string().parse().ok())
            .collect();

        sum += solve_spring_line(springs, arrangments, unfold);
    }

    sum
}
fn task_1(input: &str) -> usize {
    task(input, false)
}

fn task_2(input: &str) -> usize {
    task(input, true)
}

#[cfg(test)]
mod tests {
    use super::{task_1, task_2};

    fn input() -> String {
        std::fs::read_to_string("./data/day_12.txt").unwrap()
    }
    fn sample_input() -> String {
        std::fs::read_to_string("./data/day_12_sample.txt").unwrap()
    }

    #[test]
    fn test_day_12_task_1() {
        let sample_result = task_1(&sample_input());
        assert_eq!(sample_result, 21);

        let result = task_1(&input());
        println!("the solution for day 12, task 1 is: {result}",);
    }

    #[test]
    fn test_day_12_task_2() {
        let sample_result = task_2(&sample_input());
        assert_eq!(sample_result, 21);

        // let result = task_1(&input());
        // println!("the solution for day 12, task 2 is: {result}",);
    }
}

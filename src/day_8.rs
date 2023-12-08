use std::collections::HashMap;

fn parse_input(input: &str) -> (String, HashMap<String, (String, String)>) {
    let lines: Vec<&str> = input.lines().collect();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    let instr = lines[0].to_string();

    let mut map_start = "";

    for i in 2..lines.len() {
        let split: Vec<&str> = lines[i].split("=").collect();
        let key = split[0].trim();

        let val: Vec<&str> = split[1].split(",").collect();

        if i == 2 {
            map_start = key;
        }

        map.insert(
            key.into(),
            (
                val[0].trim().replace("(", "").into(),
                val[1].trim().replace(")", "").into(),
            ),
        );
    }

    (instr, map)
}

fn task_1(input: &str) -> usize {
    let (instr, map) = parse_input(input);

    let mut map_current = "AAA";
    let mut step_count = 0;

    while (map_current != "ZZZ") {
        for c in instr.chars() {
            if c == 'L' {
                map_current = &map.get(map_current).unwrap().0;
            } else if c == 'R' {
                map_current = &map.get(map_current).unwrap().1;
            } else {
                panic!("invalid instruction! {c}");
            }

            // println!("map_current: {map_current}");

            step_count += 1;
        }
    }

    step_count
}

fn task_2(input: &str) -> usize {
    let (instr, map) = parse_input(input);

    let mut step_count = 0;

    let start: Vec<&String> = map
        .keys()
        .collect::<Vec<&String>>()
        .into_iter()
        .filter(|k| k.ends_with("A"))
        .collect();

    let end: Vec<&String> = map
        .keys()
        .collect::<Vec<&String>>()
        .into_iter()
        .filter(|k| k.ends_with("Z"))
        .collect();

    let mut current = start;
    let mut current_steps = vec![];

    let empty = "---".to_string();

    while (!current.iter().all(|c| c == &&empty)) {
        for c in instr.chars() {
            for i in 0..current.len() {
                if current[i] == &empty {
                    continue;
                }
                if current[i].ends_with("Z") {
                    current_steps.push(step_count);
                    current[i] = &empty;
                    continue;
                }
                if c == 'L' {
                    current[i] = &map.get(current[i].as_str()).unwrap().0;
                } else if c == 'R' {
                    current[i] = &map.get(current[i].as_str()).unwrap().1;
                } else {
                    panic!("invalid instruction! {c}");
                }
            }
            step_count += 1;
        }
    }

    lcm_of_array(&current_steps).unwrap()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn lcm_of_array(numbers: &[usize]) -> Option<usize> {
    if numbers.is_empty() {
        return None;
    }

    let mut result = numbers[0];

    for &num in &numbers[1..] {
        result = lcm(result, num);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::{task_1, task_2};

    fn input() -> String {
        std::fs::read_to_string("./data/day_8.txt").unwrap()
    }
    fn sample_input() -> String {
        std::fs::read_to_string("./data/day_8_sample.txt").unwrap()
    }
    fn sample_input_2() -> String {
        std::fs::read_to_string("./data/day_8_sample_2.txt").unwrap()
    }

    #[test]
    fn test_day_8_task_1() {
        let sample_result = task_1(&sample_input());
        assert_eq!(sample_result, 6);

        let result = task_1(&input());
        println!("the solution for day 8, task 1 is: {result}",);
    }

    #[test]
    fn test_day_8_task_2() {
        let sample_result = task_2(&sample_input_2());
        assert_eq!(sample_result, 6);

        let result = task_2(&input());
        println!("the solution for day 8, task 2 is: {result}",);
    }
}

const MAX_NUMBER_WORD_LENGTH: usize = 6;
const MIN_NUMBER_WORD_LENGTH: usize = 3;
const NUMBER_WORDS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn task_1(input: &str) -> u32 {
    let lines = input.split("\n");

    let mut solution = 0;

    for line in lines {
        let mut nums = vec![];
        for c in line.chars() {
            if let Ok(i) = c.to_string().parse::<u32>() {
                nums.push(i);
            }
        }

        if nums.is_empty() {
            continue;
        }

        let s = format!("{}{}", nums[0], nums[nums.len() - 1]);
        let sum: u32 = s.parse().unwrap_or(0);
        solution += sum;
    }

    solution
}

fn task_2(input: &str) -> u32 {
    let lines = input.split("\n");

    let mut solution = 0;

    for line in lines {
        let mut nums = vec![];

        let chars: Vec<char> = line.chars().collect();

        for i in 0..chars.len() {
            let c = chars[i];

            if let Ok(num) = c.to_string().parse::<usize>() {
                nums.push(num);
            } else {
                for word_len in MIN_NUMBER_WORD_LENGTH..MAX_NUMBER_WORD_LENGTH {
                    if chars.len() >= i + word_len {
                        let pos = NUMBER_WORDS
                            .iter()
                            .position(|n| n == &chars[i..i + word_len].iter().collect::<String>());

                        if let Some(p) = pos {
                            nums.push(p + 1);
                            continue;
                        }
                    }
                }
            }
        }

        if nums.is_empty() {
            continue;
        }

        let s = format!("{}{}", nums[0], nums[nums.len() - 1]);
        let sum: u32 = s.parse().unwrap_or(0);
        solution += sum;
    }

    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> String {
        std::fs::read_to_string("./data/day_1.txt").unwrap()
    }

    #[test]
    fn test_day_1_task_1() {
        println!("the solution for day 1, task 1 is: {}", task_1(&input()));
    }
    #[test]
    fn test_day_1_task_2() {
        println!("the solution day 1, task 2 is: {}", task_2(&input()));
    }
}

use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Field {
    Number(u32),
    Gear,
    Symbol,
    Dot,
}

impl Field {
    fn is_num(&self) -> bool {
        if let Field::Number(_) = &self {
            true
        } else {
            false
        }
    }
}

fn task_1(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();

    let y_len = lines.len();

    if lines.is_empty() {
        panic!("no input given");
    }

    let x_len = lines[0].len();

    let mut matrix: Vec<Vec<Field>> = vec![];

    // load all the characters
    for y in 0..y_len {
        matrix.push(
            lines[y]
                .chars()
                .map(|c| match c {
                    '.' => Field::Dot,
                    c if c.to_string().parse::<u32>().is_ok() => {
                        Field::Number(c.to_string().parse().unwrap())
                    }
                    _ => Field::Symbol,
                })
                .collect(),
        );
    }

    let mut numbers_near_symbols = vec![];

    for y in 0..y_len {
        for x in 0..x_len {
            if let Field::Number(n) = matrix[y][x] {
                if (x + 1 < x_len && matrix[y][x + 1] == Field::Symbol)
                    || (x > 0 && matrix[y][x - 1] == Field::Symbol)
                    || (y + 1 < y_len && matrix[y + 1][x] == Field::Symbol)
                    || (y > 0 && matrix[y - 1][x] == Field::Symbol)
                    || (x + 1 < x_len && y + 1 < y_len && matrix[y + 1][x + 1] == Field::Symbol)
                    || (x + 1 < x_len && y > 0 && matrix[y - 1][x + 1] == Field::Symbol)
                    || (x > 0 && y + 1 < y_len && matrix[y + 1][x - 1] == Field::Symbol)
                    || (x > 0 && y > 0 && matrix[y - 1][x - 1] == Field::Symbol)
                {
                    let mut number_parts = VecDeque::new();

                    for i in (0..x).rev() {
                        if let Field::Number(np) = matrix[y][i] {
                            number_parts.push_front(np);
                            matrix[y][i] = Field::Dot;
                        } else {
                            break;
                        }
                    }

                    for i in x..x_len {
                        if let Field::Number(np) = matrix[y][i] {
                            number_parts.push_back(np);
                            matrix[y][i] = Field::Dot;
                        } else {
                            break;
                        }
                    }

                    let num: u32 = number_parts
                        .iter()
                        .flat_map(|n| n.to_string().chars().collect::<Vec<char>>())
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    numbers_near_symbols.push(num);
                }
            }
        }
    }

    numbers_near_symbols.iter().sum()
}

fn find_num_coords(
    matrix: &Vec<Vec<Field>>,
    x: usize,
    y: usize,
    x_len: usize,
    y_len: usize,
) -> Option<(usize, usize)> {
    if (x + 1 < x_len && matrix[y][x + 1].is_num()) {
        return Some((x + 1, y));
    }
    if (x > 0 && matrix[y][x - 1].is_num()) {
        return Some((x - 1, y));
    }
    if (y + 1 < y_len && matrix[y + 1][x].is_num()) {
        return Some((x, y + 1));
    }
    if (y > 0 && matrix[y - 1][x].is_num()) {
        return Some((x, y - 1));
    }
    if (x + 1 < x_len && y + 1 < y_len && matrix[y + 1][x + 1].is_num()) {
        return Some((x + 1, y + 1));
    }
    if (x + 1 < x_len && y > 0 && matrix[y - 1][x + 1].is_num()) {
        return Some((x + 1, y - 1));
    }
    if (x > 0 && y + 1 < y_len && matrix[y + 1][x - 1].is_num()) {
        return Some((x - 1, y + 1));
    }
    if (x > 0 && y > 0 && matrix[y - 1][x - 1].is_num()) {
        return Some((x - 1, y - 1));
    }

    return None;
}

fn task_2(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();

    let y_len = lines.len();

    if lines.is_empty() {
        panic!("no input given");
    }

    let x_len = lines[0].len();

    let mut matrix: Vec<Vec<Field>> = vec![];

    // load all the characters
    for y in 0..y_len {
        matrix.push(
            lines[y]
                .chars()
                .map(|c| match c {
                    c if c.to_string().parse::<u32>().is_ok() => {
                        Field::Number(c.to_string().parse().unwrap())
                    }
                    '*' => Field::Gear,
                    _ => Field::Dot,
                })
                .collect(),
        );
    }

    let mut gear_ratios = vec![];

    for y in 0..y_len {
        for x in 0..x_len {
            if matrix[y][x] == Field::Gear {
                let mut gear_numbers = vec![];

                while let Some((x, y)) = find_num_coords(&matrix, x, y, x_len, y_len) {
                    let mut number_parts = VecDeque::new();

                    for i in (0..x).rev() {
                        if let Field::Number(np) = matrix[y][i] {
                            number_parts.push_front(np);
                            matrix[y][i] = Field::Dot;
                        } else {
                            break;
                        }
                    }

                    for i in x..x_len {
                        if let Field::Number(np) = matrix[y][i] {
                            number_parts.push_back(np);
                            matrix[y][i] = Field::Dot;
                        } else {
                            break;
                        }
                    }

                    if number_parts.is_empty() {
                        continue;
                    }

                    let num: u32 = number_parts
                        .iter()
                        .flat_map(|n| n.to_string().chars().collect::<Vec<char>>())
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    gear_numbers.push(num);
                }

                if gear_numbers.len() == 2 {
                    println!("gears: {:?}", gear_numbers);
                    gear_ratios.push(gear_numbers[0] * gear_numbers[1]);
                }
            }
        }
    }

    gear_ratios.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day_3::{task_1, task_2};

    fn input() -> String {
        std::fs::read_to_string("./data/day_3.txt").unwrap()
    }
    fn sample_input() -> String {
        std::fs::read_to_string("./data/day_3_sample.txt").unwrap()
    }

    #[test]
    fn test_day_3_task_1() {
        println!("the solution for day 3, task 1 is: {}", task_1(&input()));
    }

    #[test]
    fn test_day_3_task_2() {
        println!("the solution for day 3, task 2 is: {}", task_2(&input()));
    }
}

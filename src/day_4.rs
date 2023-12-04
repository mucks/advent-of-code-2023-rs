use std::collections::BTreeMap;

fn task_1(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut total_points = 0;

    for line in lines {
        let game = line.split(":").collect::<Vec<&str>>()[1]
            .trim()
            .split("|")
            .collect::<Vec<&str>>();

        let winning_numbers: Vec<u32> = game[0]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let my_numbers: Vec<u32> = game[1]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let mut win_count = 0;

        for n in my_numbers {
            if winning_numbers.contains(&n) {
                win_count += 1;
            }
        }

        if win_count >= 1 {
            total_points += 2u32.pow(win_count - 1);
        }
    }

    total_points
}

fn task_2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut total_cards = 0;
    let mut cards_win_map: BTreeMap<u32, u32> = BTreeMap::new();

    for line in lines {
        let game_parts = line.split(":").collect::<Vec<&str>>();
        let game = game_parts[1].trim().split("|").collect::<Vec<&str>>();

        let game_index = game_parts[0]
            .split_whitespace()
            .filter_map(|g| g.parse().ok())
            .collect::<Vec<u32>>()[0];

        let winning_numbers: Vec<u32> = game[0]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let my_numbers: Vec<u32> = game[1]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let mut win_count = 0;

        for n in my_numbers {
            if winning_numbers.contains(&n) {
                win_count += 1;
            }
        }

        total_cards += 1;
        cards_win_map.insert(game_index, win_count);
    }

    let cards_win_map_size = cards_win_map.len();

    let mut cards_to_process = vec![];

    for (k, v) in &cards_win_map {
        if v > &0 {
            for i in 0..*v {
                cards_to_process.push(k + i + 1);
                total_cards += 1;
            }
        }
    }

    while !cards_to_process.is_empty() {
        let mut indexes_to_remove = vec![];

        for (i, c) in cards_to_process.clone().iter().enumerate() {
            let opt_v = cards_win_map.get(&c).clone();
            if let Some(v) = opt_v {
                if v > &&0 {
                    for i in 0..*v {
                        let new_card = c + i + 1;

                        if new_card as usize <= cards_win_map_size {
                            cards_to_process.push(new_card);
                            total_cards += 1;
                        }
                    }
                }
                indexes_to_remove.push(i);
            }
        }

        for i in indexes_to_remove {
            cards_to_process[i] = 0;
        }

        cards_to_process = cards_to_process
            .iter()
            .filter(|c| c > &&0)
            .cloned()
            .collect();
    }

    total_cards
}

#[cfg(test)]
mod tests {
    use crate::day_4::{task_1, task_2};

    fn input() -> String {
        std::fs::read_to_string("./data/day_4.txt").unwrap()
    }
    fn sample_input() -> String {
        std::fs::read_to_string("./data/day_4_sample.txt").unwrap()
    }

    #[test]
    fn test_day_4_task_1() {
        let sample_result = task_1(&sample_input());
        assert_eq!(sample_result, 13);

        let result = task_1(&input());
        println!("the solution for day 4, task 1 is: {result}",);
    }

    #[test]
    fn test_day_4_task_2() {
        let sample_result = task_2(&sample_input());
        assert_eq!(sample_result, 30);

        let result = task_2(&input());
        println!("the solution for day 4, task 2 is: {result}",);
    }
}

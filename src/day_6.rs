use std::env::join_paths;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn collect_races(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();

    let mut races = vec![];

    let times: Vec<usize> = lines[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let distances: Vec<usize> = lines[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        })
    }
    races
}

fn calculate_race_win_combinations(race: &Race) -> usize {
    let first_step_size = match race.time / 100 == 0 {
        true => 1,
        false => race.time / 100,
    };

    let mut first_win_condition = 0;

    // get first step
    for j in (0..race.time).step_by(first_step_size) {
        let mut speed = 0;
        let mut travel_distance = 0;

        for i in 0..race.time {
            if i <= j {
                speed += 1;
            } else {
                travel_distance += speed;
            }
        }

        if travel_distance > race.distance {
            first_win_condition = j;
            break;
        }
    }

    println!("first_step_size: {first_step_size}");

    // get lowest win condition
    let mut step_size = first_step_size;
    let mut lowest_win_condition = first_win_condition;

    while (step_size > 1) {
        step_size /= 10;
        if step_size == 0 {
            step_size = 1;
        }

        for j in (0..lowest_win_condition).step_by(step_size).rev() {
            let mut speed = 0;
            let mut travel_distance = 0;

            for i in 0..race.time {
                if i <= j {
                    speed += 1;
                } else {
                    travel_distance += speed;
                }
            }

            if travel_distance > race.distance {
                lowest_win_condition = j;
                println!("lowest win condition: {:?}", lowest_win_condition);
            } else {
                break;
            }
        }
    }

    let mut last_win_condition = 0;

    // get last win condition
    for j in (0..race.time).step_by(first_step_size).rev() {
        let mut speed = 0;
        let mut travel_distance = 0;

        for i in 0..race.time {
            if i <= j {
                speed += 1;
            } else {
                travel_distance += speed;
            }
        }

        if travel_distance > race.distance {
            last_win_condition = j;
            break;
        }
    }

    // get highest win condition
    let mut step_size = first_step_size;
    let mut highest_win_condition = last_win_condition;

    while (step_size > 1) {
        step_size /= 10;
        if step_size == 0 {
            step_size = 1;
        }

        for j in (highest_win_condition..race.time).step_by(step_size) {
            let mut speed = 0;
            let mut travel_distance = 0;

            for i in 0..race.time {
                if i <= j {
                    speed += 1;
                } else {
                    travel_distance += speed;
                }
            }

            if travel_distance > race.distance {
                highest_win_condition = j;
                println!("highest win condition: {}", highest_win_condition);
            } else {
                break;
            }
        }
    }

    (lowest_win_condition..highest_win_condition).len() + 1
}

fn task_1(input: &str) -> usize {
    let races = collect_races(input);
    println!("{races:?}");

    let mut win_combinations = vec![];

    for race in races {
        win_combinations.push(calculate_race_win_combinations(&race));
    }

    println!("win_combinations: {win_combinations:?}");

    win_combinations.iter().fold(1, |a, b| a * b)
}

fn combined_race(input: &str) -> Race {
    let races = collect_races(input);

    let race_time: usize = races
        .iter()
        .fold("".to_string(), |a, b| format!("{}{}", a, b.time))
        .parse()
        .unwrap();
    let race_distance: usize = races
        .iter()
        .fold("".to_string(), |a, b| format!("{}{}", a, b.distance))
        .parse()
        .unwrap();

    Race {
        time: race_time,
        distance: race_distance,
    }
}

fn task_2(input: &str) -> usize {
    let race = combined_race(input);
    println!("race: {race:?}");

    calculate_race_win_combinations(&race)
}

#[cfg(test)]
mod tests {
    use crate::day_6::{task_1, task_2};

    fn input() -> String {
        std::fs::read_to_string("./data/day_6.txt").unwrap()
    }
    fn sample_input() -> String {
        std::fs::read_to_string("./data/day_6_sample.txt").unwrap()
    }

    #[test]
    fn test_day_6_task_1() {
        let sample_result = task_1(&sample_input());
        assert_eq!(sample_result, 288);

        let result = task_1(&input());
        println!("the solution for day 6, task 1 is: {result}",);
    }

    #[test]
    fn test_day_6_task_2() {
        let sample_result = task_2(&sample_input());
        assert_eq!(sample_result, 71503);

        let result = task_2(&input());
        println!("the solution for day 6, task 2 is: {result}",);
    }
}

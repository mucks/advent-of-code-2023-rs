fn task_1(input: &str, red: u32, green: u32, blue: u32) -> u32 {
    let lines = input.split("\n");

    let mut valid_game_id_sum = 0u32;

    for line in lines {
        let game: Vec<&str> = line.split(":").collect();

        if game.len() != 2 {
            continue;
        }

        let game_title = game[0];

        let game_id: u32 = game_title.replace("Game ", "").parse().unwrap();

        let game_content = game[1];

        let rounds = game_content.split(";");

        let mut valid_game = true;

        for round in rounds {
            let params = round.split(",");

            let mut valid_round = true;

            for param in params {
                let param_parts: Vec<&str> = param.split_whitespace().collect();

                if param_parts.len() != 2 {
                    continue;
                }

                let count: u32 = param_parts[0].trim().parse().unwrap();

                let color = param_parts[1];

                let valid_count = match color.trim() {
                    "green" => count <= green,
                    "blue" => count <= blue,
                    "red" => count <= red,
                    _ => false,
                };

                if !valid_count {
                    valid_round = false;
                }
            }

            if !valid_round {
                valid_game = false;
            }
        }

        if valid_game {
            valid_game_id_sum += game_id;
        }
    }

    valid_game_id_sum
}

fn task_2(input: &str) -> u32 {
    let lines = input.split("\n");

    let mut power_of_min_cubes = 0;

    for line in lines {
        let game: Vec<&str> = line.split(":").collect();

        if game.len() != 2 {
            continue;
        }

        let game_title = game[0];

        let game_id: u32 = game_title.replace("Game ", "").parse().unwrap();

        let game_content = game[1];

        let rounds = game_content.split(";");

        let mut valid_game = true;

        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for round in rounds {
            let params = round.split(",");

            let mut valid_round = true;

            for param in params {
                let param_parts: Vec<&str> = param.split_whitespace().collect();

                if param_parts.len() != 2 {
                    continue;
                }

                let count: u32 = param_parts[0].trim().parse().unwrap();

                let color = param_parts[1];

                match color.trim() {
                    "green" => {
                        if count >= min_green {
                            min_green = count
                        }
                    }
                    "blue" => {
                        if count >= min_blue {
                            min_blue = count
                        }
                    }
                    "red" => {
                        if count >= min_red {
                            min_red = count
                        }
                    }
                    _ => (),
                };
            }
        }

        power_of_min_cubes += min_red * min_green * min_blue;
    }

    power_of_min_cubes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> String {
        std::fs::read_to_string("./data/day_2.txt").unwrap()
    }

    #[test]
    fn test_day_1_task_1() {
        println!(
            "the solution for day 2, task 1 is: {}",
            task_1(&input(), 12, 13, 14)
        );
    }

    #[test]
    fn test_day_1_task_2() {
        println!("the solution day 2, task 2 is: {}", task_2(&input()));
    }
}

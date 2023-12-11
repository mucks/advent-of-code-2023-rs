use std::cmp::{max, min};

type CharSpace = Vec<Vec<char>>;

fn parse_space(input: &str) -> CharSpace {
    let lines: Vec<&str> = input.lines().collect();

    let mut space = vec![];

    let y_len = lines.len();
    let x_len = lines[0].len();

    for y in 0..y_len {
        let mut space_row = vec![];
        let line: Vec<char> = lines[y].chars().collect();

        for x in 0..x_len {
            space_row.push(line[x]);
        }
        space.push(space_row);
    }
    space
}

fn empty_row_indexes(space: &CharSpace) -> Vec<usize> {
    space
        .iter()
        .enumerate()
        .filter(|(_, s)| !s.contains(&'#'))
        .map(|(i, _)| i)
        .collect()
}

fn empty_col_indexes(space: &CharSpace) -> Vec<usize> {
    let cols = collect_cols(space);
    empty_row_indexes(&cols)
}

fn collect_cols(space: &CharSpace) -> CharSpace {
    let mut cols = vec![];
    for x in 0..space[0].len() {
        let mut column = vec![];
        for y in 0..space.len() {
            column.push(space[y][x]);
        }
        cols.push(column);
    }
    cols
}

fn expand_space(mut space: CharSpace) -> CharSpace {
    // expand rows
    let mut rows_added = 0;
    for (i, row) in space.clone().iter().enumerate() {
        if !row.contains(&'#') {
            space.insert(i + rows_added, row.clone());
            rows_added += 1;
        }
    }

    // expand cols
    let mut cols = collect_cols(&space);
    let mut cols_added = 0;

    for (i, col) in cols.clone().iter().enumerate() {
        if !col.contains(&'#') {
            cols.insert(i + cols_added, col.clone());
            cols_added += 1;
        }
    }

    space = vec![];

    for y in 0..cols[0].len() {
        let mut row = vec![];
        for x in 0..cols.len() {
            row.push(cols[x][y]);
        }
        space.push(row);
    }
    space
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    x: usize,
    y: usize,
    galaxy: Option<usize>,
}

type Space = Vec<Vec<Node>>;

fn name_galaxies(mut char_space: CharSpace) -> Space {
    let mut count = 0;
    let mut space = vec![];
    for y in 0..char_space.len() {
        let mut row = vec![];
        for x in 0..char_space[y].len() {
            let mut galaxy = None;
            if char_space[y][x] == '#' {
                count += 1;
                galaxy = Some(count);
            }
            row.push(Node { x, y, galaxy })
        }
        space.push(row)
    }
    space
}

fn task(input: &str, task_n: usize, expansion_size: usize) -> usize {
    let mut char_space = parse_space(input);
    print_char_space(&char_space);

    if task_n == 1 {
        char_space = expand_space(char_space);
        print_char_space(&char_space);
    }

    let space = name_galaxies(char_space.clone());
    print_space(&space);

    let nodes: Vec<Node> = space.into_iter().flatten().collect();
    let galaxies: Vec<&Node> = nodes.iter().filter(|n| n.galaxy.is_some()).collect();
    let mut galaxy_pairs = vec![];

    for galaxy_a in &galaxies {
        for galaxy_b in &galaxies {
            let mut galaxy_pair = vec![galaxy_a, galaxy_b];
            galaxy_pair.sort_by(|a, b| a.galaxy.cmp(&b.galaxy));
            if galaxy_b != galaxy_a && !galaxy_pairs.contains(&galaxy_pair) {
                galaxy_pairs.push(galaxy_pair);
            }
        }
    }

    if task_n == 1 {
        return galaxy_pairs
            .iter()
            .map(|gp| manhatten_distance(gp[0], gp[1]))
            .sum();
    }

    let galaxy_pairs_len = galaxy_pairs.len();

    let empty_row_indexes = empty_row_indexes(&char_space);
    let empty_col_indexes = empty_col_indexes(&char_space);

    println!("empty_row_indexes: {empty_row_indexes:?}");
    println!("empty_col_indexes: {empty_col_indexes:?}");

    let mut sum = 0;

    for gp in galaxy_pairs {
        // // 5 -> 9
        // if !(gp[0].galaxy == Some(5) && gp[1].galaxy == Some(9)) {
        //     continue;
        // }

        let a = gp[0];
        let b = gp[1];

        let mut empty_rows_passed = 0;
        let mut empty_cols_passed = 0;

        let range = match a.y <= b.y {
            true => a.y..b.y,
            false => b.y..a.y,
        };

        for y in range {
            if empty_row_indexes.contains(&y) {
                empty_rows_passed += 1;
            }
        }

        let range = match a.x <= b.x {
            true => a.x..b.x,
            false => b.x..a.x,
        };

        for x in range {
            if empty_col_indexes.contains(&x) {
                empty_cols_passed += 1;
            }
        }

        let mut a = a.to_owned().clone();
        let mut b = b.to_owned().clone();

        let mut dist = manhatten_distance(&a, &b);

        dist += expansion_size * empty_cols_passed - empty_cols_passed;
        dist += expansion_size * empty_rows_passed - empty_rows_passed;

        sum += dist;
    }

    sum
}
fn manhatten_distance(a: &Node, b: &Node) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn print_space(space: &Space) {
    for y in 0..space.len() {
        for x in 0..space[y].len() {
            print!(
                "{}",
                space[y][x]
                    .galaxy
                    .map(|g| g.to_string())
                    .unwrap_or(".".into())
            );
        }
        println!();
    }
}

fn print_char_space(space: &CharSpace) {
    for y in 0..space.len() {
        for x in 0..space[y].len() {
            print!("{}", space[y][x]);
        }
        println!();
    }
}
fn task_1(input: &str) -> usize {
    task(input, 1, 0)
}

fn task_2(input: &str, expansion_size: usize) -> usize {
    task(input, 2, expansion_size)
}

#[cfg(test)]
mod tests {
    use super::{task_1, task_2};

    fn input() -> String {
        std::fs::read_to_string("./data/day_11.txt").unwrap()
    }
    fn sample_input() -> String {
        std::fs::read_to_string("./data/day_11_sample.txt").unwrap()
    }

    #[test]
    fn test_day_11_task_1() {
        let sample_result = task_1(&sample_input());
        assert_eq!(sample_result, 374);

        let result = task_1(&input());
        println!("the solution for day 10, task 1 is: {result}",);
    }

    #[test]
    fn test_day_11_task_2() {
        let sample_result = task_2(&sample_input(), 2);
        assert_eq!(sample_result, 374);

        let sample_result = task_2(&sample_input(), 10);
        assert_eq!(sample_result, 1030);

        let sample_result = task_2(&sample_input(), 100);
        assert_eq!(sample_result, 8410);

        let result = task_2(&input(), 1_000_000);
        println!("the solution for day 10, task 2 is: {result}",);
    }
}

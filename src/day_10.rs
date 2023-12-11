use std::{collections::LinkedList, io::Write, path::Display, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pipe {
    pipe_type: PipeType,
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PipeType {
    Vertical,   // North <-> South
    Horizontal, // East  <-> West
    L,          // North <-> East
    J,          // North <-> West
    Seven,      // South <-> West
    F,          // South <-> East
    S,          // South <-> North <-> East <-> West
    Ground,     // . no pipe
}

impl PipeType {
    fn to_dirs(&self) -> Vec<Direction> {
        use Direction::*;
        use PipeType::*;

        match self {
            Vertical => vec![North, South],
            Horizontal => vec![East, West],
            L => vec![North, East],
            J => vec![North, West],
            Seven => vec![South, West],
            F => vec![South, East],
            S => vec![South, North, East, West],
            Ground => vec![],
        }
    }

    fn can_connect(&self, rhs: &Self, dir: Direction) -> bool {
        use Direction::*;
        use PipeType::*;

        let mut self_allowed = vec![];
        let mut rhs_allowed = vec![];

        match dir {
            North => {
                self_allowed = vec![Vertical, L, J, S];
                rhs_allowed = vec![Vertical, F, Seven, S];
            }
            South => {
                self_allowed = vec![Vertical, F, Seven, S];
                rhs_allowed = vec![Vertical, L, J, S];
            }
            East => {
                self_allowed = vec![Horizontal, F, L, S];
                rhs_allowed = vec![Horizontal, J, Seven, S];
            }
            West => {
                self_allowed = vec![Horizontal, J, Seven, S];
                rhs_allowed = vec![Horizontal, F, L, S];
            }
        }

        self_allowed.contains(self) && rhs_allowed.contains(rhs)
    }
}

impl std::fmt::Display for PipeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PipeType::*;
        let s = match self {
            Vertical => '|',
            Horizontal => '-',
            L => 'L',
            J => 'J',
            Seven => '7',
            F => 'F',
            S => 'S',
            Ground => '.',
        };
        write!(f, "{}", s)
    }
}

impl From<char> for PipeType {
    fn from(value: char) -> Self {
        match value {
            '|' => PipeType::Vertical,
            '-' => PipeType::Horizontal,
            'L' => PipeType::L,
            'J' => PipeType::J,
            '7' => PipeType::Seven,
            'F' => PipeType::F,
            'S' => PipeType::S,
            '.' => PipeType::Ground,
            _ => panic!("invalid field!"),
        }
    }
}

fn collect_pipes(input: &str) -> Vec<Vec<Pipe>> {
    let lines: Vec<&str> = input.lines().collect();

    let mut pipes = vec![];

    for y in 0..lines.len() {
        let line: Vec<char> = lines[y].chars().collect();
        let mut x_pipes = vec![];
        for x in 0..lines[0].chars().count() {
            let pipe_type = PipeType::from(line[x]);
            let pipe = Pipe { x, y, pipe_type };

            x_pipes.push(pipe)
        }
        pipes.push(x_pipes);
    }
    pipes
}

#[derive(Debug, Clone)]
struct PipesList {
    prev_pipe: Option<Box<PipesList>>,
    pipe: Pipe,
}

impl PipesList {
    pub fn new(pipe: Pipe) -> Self {
        PipesList {
            prev_pipe: None,
            pipe,
        }
    }
}

struct PipeMaze {
    list: PipesList,
    matrix: Vec<Vec<Pipe>>,
    start: Pipe,
    visited: Vec<Pipe>,
    step_count: usize,
}

impl PipeMaze {
    // https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule
    fn is_point_in_polygon(&self, x: isize, y: isize) -> bool {
        let poly = &self.visited;

        let num = poly.len();
        let mut j = num - 1;
        let mut c = false;
        for i in 0..num {
            if x == poly[i].x as isize && y == poly[i].y as isize {
                // point is a corner
                return true;
            }
            if (poly[i].y as isize > y) != (poly[j].y as isize > y) {
                let slope = (x - poly[i].x as isize) * (poly[j].y as isize - poly[i].y as isize)
                    - (poly[j].x as isize - poly[i].x as isize) * (y - poly[i].y as isize);

                if slope == 0 {
                    // point is on boundary
                    return true;
                }
                if (slope < 0) != (poly[j].y < poly[i].y) {
                    c = !c;
                }
            }
            j = i
        }

        c
    }

    fn enclosed_points(&self) -> usize {
        self.log_maze();

        let mut points = 0;
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[0].len() {
                if self.is_point_in_polygon(x as isize, y as isize) {
                    points += 1;
                }
            }
        }
        points - self.visited.len()
    }

    fn log_maze(&self) {
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[0].len() {
                let pipe = self.matrix[y][x];
                if self.visited.contains(&pipe) {
                    print!("\x1b[93m{}\x1b[0m", pipe.pipe_type)
                } else {
                    print!("{}", pipe.pipe_type)
                }
            }
            println!("");
        }
        std::thread::sleep(Duration::from_millis(200));
    }

    fn new(pipes: Vec<Vec<Pipe>>) -> Self {
        let pipes_flattened: Vec<&Pipe> = pipes.iter().flatten().collect();

        let start = pipes_flattened
            .iter()
            .find(|p| p.pipe_type == PipeType::S)
            .unwrap();

        let list = PipesList::new(**start);
        let visited = vec![**start];

        Self {
            start: **start,
            matrix: pipes,
            list,
            visited,
            step_count: 0,
        }
    }

    fn solve(&mut self) {
        loop {
            let old_step = self.step_count;
            let dirs = self.list.pipe.pipe_type.to_dirs();
            let mut possible_steps = vec![];
            for dir in dirs {
                possible_steps.push(self.step(dir));
            }
            if old_step == self.step_count {
                if possible_steps
                    .iter()
                    .any(|ps| ps.is_some() && ps.unwrap().pipe_type == PipeType::S)
                {
                    break;
                } else {
                    println!("going back");
                    self.go_back();
                }
            }
        }
    }

    fn step(&mut self, dir: Direction) -> Option<Pipe> {
        use Direction::*;

        let y = self.list.pipe.y;
        let x = self.list.pipe.x;

        let y_len = self.matrix.len();
        let x_len = self.matrix[0].len();

        // len
        if !match dir {
            North => y > 0,
            South => y < y_len - 1,
            West => x > 0,
            East => x < x_len - 1,
        } {
            return None;
        }

        let mut new_pipe = match dir {
            North => self.matrix[y - 1][x],
            South => self.matrix[y + 1][x],
            East => self.matrix[y][x + 1],
            West => self.matrix[y][x - 1],
        };

        println!(
            "{} -> {dir:?} -> {}",
            self.list.pipe.pipe_type, new_pipe.pipe_type
        );

        if !self
            .list
            .pipe
            .pipe_type
            .can_connect(&new_pipe.pipe_type, dir)
        {
            return None;
        }

        if !self.visited.contains(&new_pipe) {
            println!("new_pipe: {new_pipe:?}, step_count: {}", self.step_count);
            // println!("visited: {:?}", self.visited);
            self.list.prev_pipe = Some(Box::new(self.list.clone()));
            self.list.pipe = new_pipe;
            self.visited.push(new_pipe);
            self.step_count += 1;
        }

        Some(new_pipe)
    }

    fn go_back(&mut self) {
        if let Some(prev_pipe) = &self.list.prev_pipe {
            self.list = prev_pipe.as_ref().clone();
        }
    }
}

fn task_1(input: &str) -> usize {
    let pipes = collect_pipes(input);
    let mut maze = PipeMaze::new(pipes);
    maze.solve();
    maze.visited.len() / 2
}

fn task_2(input: &str) -> usize {
    let pipes = collect_pipes(input);
    let mut maze = PipeMaze::new(pipes);
    maze.solve();

    maze.enclosed_points()
}

#[cfg(test)]
mod tests {
    use super::{task_1, task_2};

    fn input() -> String {
        std::fs::read_to_string("./data/day_10.txt").unwrap()
    }
    fn sample_input() -> String {
        std::fs::read_to_string("./data/day_10_sample.txt").unwrap()
    }
    fn sample_input_2() -> String {
        std::fs::read_to_string("./data/day_10_sample_2.txt").unwrap()
    }
    fn sample_input_3() -> String {
        std::fs::read_to_string("./data/day_10_sample_3.txt").unwrap()
    }
    fn sample_input_4() -> String {
        std::fs::read_to_string("./data/day_10_sample_4.txt").unwrap()
    }

    #[test]
    fn test_day_10_task_1() {
        let sample_result = task_1(&sample_input());
        assert_eq!(sample_result, 8);

        let result = task_1(&input());
        println!("the solution for day 10, task 1 is: {result}",);
    }

    #[test]
    fn test_day_10_task_2() {
        let sample_result = task_2(&sample_input_2());
        assert_eq!(sample_result, 4);

        let sample_result = task_2(&sample_input_3());
        assert_eq!(sample_result, 8);

        let sample_result = task_2(&sample_input_4());
        assert_eq!(sample_result, 10);

        let result = task_2(&input());
        println!("the solution for day 10, task 2 is: {result}",);
    }
}

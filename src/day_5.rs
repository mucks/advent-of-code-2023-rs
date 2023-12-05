#[derive(Debug, Clone, Copy)]
struct Map {
    source_range_start: usize,
    dest_range_start: usize,
    range_len: usize,
}

#[derive(Debug, Clone)]
struct MapGroup {
    source_idx: usize,
    dest_idx: usize,
    maps: Vec<Map>,
}

impl MapGroup {
    fn map_seed(&self, mut seed: usize) -> usize {
        for m in &self.maps {
            if seed >= m.source_range_start && seed < m.source_range_start + m.range_len {
                let seed_start_diff = seed - m.source_range_start;
                let dest = m.dest_range_start + seed_start_diff;
                seed = dest;
                break;
            }
        }

        seed
    }
}

struct Almanac {
    map_groups: Vec<MapGroup>,
    seeds: Vec<usize>,
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut seeds: Vec<usize> = vec![];

        let mut map_groups = vec![];

        let mut map_group = MapGroup {
            source_idx: 0,
            dest_idx: 1,
            maps: vec![],
        };

        for (i, line) in value.lines().enumerate() {
            if line.starts_with("seeds:") {
                seeds = line
                    .trim_start_matches("seeds:")
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                continue;
            }

            // ignore seed line and whitespace line at this point
            if i <= 2 {
                continue;
            }

            if line.contains("map:") {
                map_group = MapGroup {
                    source_idx: map_groups.len(),
                    dest_idx: map_groups.len() + 1,
                    maps: vec![],
                };
                continue;
            }

            if line.is_empty() {
                map_groups.push(map_group.clone());
                continue;
            }

            let map: Vec<usize> = line
                .split_whitespace()
                .filter_map(|l| l.parse().ok())
                .collect();

            map_group.maps.push(Map {
                dest_range_start: map[0],
                source_range_start: map[1],
                range_len: map[2],
            });
        }
        // last mapping doesn't have a following whitespace
        map_groups.push(map_group.clone());

        Self { map_groups, seeds }
    }
}

impl Almanac {
    fn map_seeds(&self) -> Vec<usize> {
        let mut locations = vec![];
        for (i, s) in self.seeds.iter().enumerate() {
            let l = self.map_seed(*s);
            locations.push(l);
        }

        locations
    }

    fn map_seed(&self, mut seed: usize) -> usize {
        let old_seed = seed;
        for mg in &self.map_groups {
            seed = mg.map_seed(seed);
        }
        seed
    }
}

fn task_1(input: &str) -> usize {
    let alma = Almanac::from(input);

    println!("seeds: {:?}", alma.seeds);

    println!("map_groups: {:?}", alma.map_groups);

    let mut locations = alma.map_seeds();

    println!("{:?}", locations);

    locations.sort();

    locations[0]
}

#[derive(Debug)]
struct SeedRange {
    start: usize,
    range: usize,
}

// NOTE: this task is horribly inefficient
fn task_2(input: &str) -> usize {
    let mut alma = Almanac::from(input);

    let mut seeds = vec![];

    // seeds are changed in part 2
    for i in (0..alma.seeds.len()).step_by(2) {
        seeds.push(SeedRange {
            start: alma.seeds[i],
            range: alma.seeds[i + 1],
        })
    }

    let mut smallest_location = usize::MAX;

    for seed in seeds {
        println!("calculating seed: {seed:?}");
        let mut local_seeds = Vec::with_capacity(seed.range);
        for s in seed.start..seed.start + seed.range {
            local_seeds.push(s);
        }
        alma.seeds = local_seeds;
        let mut locations = alma.map_seeds();
        locations.sort();

        if locations[0] < smallest_location {
            smallest_location = locations[0];
        }
    }

    smallest_location
}

#[cfg(test)]
mod tests {
    use crate::day_5::{task_1, task_2};

    fn input() -> String {
        std::fs::read_to_string("./data/day_5.txt").unwrap()
    }
    fn sample_input() -> String {
        std::fs::read_to_string("./data/day_5_sample.txt").unwrap()
    }

    #[test]
    fn test_day_5_task_1() {
        let sample_result = task_1(&sample_input());
        assert_eq!(sample_result, 35);

        let result = task_1(&input());
        println!("the solution for day 5, task 1 is: {result}",);
    }

    #[test]
    fn test_day_5_task_2() {
        let sample_result = task_2(&sample_input());
        assert_eq!(sample_result, 46);

        let result = task_2(&input());
        println!("the solution for day 5, task 2 is: {result}",);
    }
}

use crate::{aoclib::day::*, run_day};

fn solution<const WINDOW_SIZE: usize>(s: String) -> u64 {
    let depths: Vec<u64> = s.lines().map(|n| n.parse().expect("Invalid input")).collect();
    let win1 = depths.windows(WINDOW_SIZE);
    let win2 = depths[1..].windows(WINDOW_SIZE);

    win1.zip(win2).filter(|(a, b)| {
        b.iter().sum::<u64>() > a.iter().sum::<u64>()
    }).count() as u64
}

pub fn run(is_sample: bool) {
    let part1 = solution::<1>;
    let part2 = solution::<3>;
    run_day!(2021, 1, is_sample, (part1, part2));
}

use crate::aoclib::day::*;

fn solution<const WINDOW_SIZE: usize>(depths: Vec<u64>) -> u64 {
    let win1 = depths.windows(WINDOW_SIZE);
    let win2 = depths[1..].windows(WINDOW_SIZE);

    win1.zip(win2).filter(|(a, b)| {
        b.iter().sum::<u64>() > a.iter().sum::<u64>()
    }).count() as u64
}

aoc_day_with_line_serializer!(2021, 1, |l| l.parse::<u64>().expect("Invalid Input"), solution::<1>, solution::<3>);

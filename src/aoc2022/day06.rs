use crate::{aoclib::day::*, run_day};

pub fn run(is_sample: bool) {
    let part1 = solution::<4>;
    let part2 = solution::<14>;
    run_day!(2022, 6, is_sample, (part1, part2));
}

pub fn solution<const WINDOW_SIZE: usize> (s: String) -> usize {
    // first whole window is after WINDOW_SIZE characters
    let mut index = WINDOW_SIZE;
    let windows = s.as_bytes().windows(WINDOW_SIZE);

    for w in windows {
        // for such small WINDOW_SIZES, using a sort/dedup
        // is faster than a HashSet (about 66% faster for my
        // specific inputs)
        let mut v: Vec<&u8> = w.iter().collect();
        v.sort();
        v.dedup();

        if v.len() == WINDOW_SIZE {
            return index;
        }

        index += 1;
    }

    panic!("Algorithmic error, a solution is guaranteed");
}

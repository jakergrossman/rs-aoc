use crate::aoclib::day::*;
use itertools::Itertools;

aoc_day!(2022, 6, solution::<4>, solution::<14>);

pub fn solution<const WINDOW_SIZE: usize> (s: String) -> usize {
    // first whole window is after WINDOW_SIZE characters
    let mut index = WINDOW_SIZE;
    let windows = s.as_bytes().windows(WINDOW_SIZE);

    for w in windows {
        // for such small WINDOW_SIZES, using a sort/dedup
        // is faster than a HashSet (about 66% faster for my
        // specific inputs)
        if w.iter().sorted().dedup().count() == WINDOW_SIZE {
            return index;
        }

        index += 1;
    }

    panic!("Algorithmic error, a solution is guaranteed");
}

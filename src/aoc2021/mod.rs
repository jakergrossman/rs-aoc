use crate::aoclib::*;

pub mod day01;

pub fn run(days: DaySpecifier, is_sample: bool) {
    run_year!(
        is_sample,
        days,
        (1, day01::run)
    );
}

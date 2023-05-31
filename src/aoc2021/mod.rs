use crate::aoclib::*;

pub mod day01;
pub mod day02;
pub mod day06;

pub fn run(days: Specifier, is_sample: bool) {
    run_year!(
        is_sample,
        days,
        (1, day01::run),
        (2, day02::run),
        (6, day06::run)
    );
}

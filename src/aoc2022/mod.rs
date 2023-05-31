use crate::aoclib::day::*;
use crate::aoclib::cli::*;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day09;
pub mod day10;
pub mod day11;

pub fn run(days: Specifier, is_sample: bool) {
    run_year!(
        is_sample,
        days,
        (1, day01::run),
        (2, day02::run),
        (3, day03::run),
        (4, day04::run),
        (5, day05::run),
        (6, day06::run),
        (9, day09::run),
        (10, day10::run),
        (11, day11::run)
    );
}

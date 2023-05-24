mod aoclib;

mod aoc2022;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_sample = args.contains(&String::from("--sample"));

    aoc2022::run(is_sample);
}

mod aoclib;

mod aoc2021;
mod aoc2022;

use clap::Parser;

fn main() -> Result<(), String> {
    let specifiers = aoclib::cli::AocCli::parse();

    let is_sample = specifiers.sample;

    match specifiers.year {
        2022 => Ok(aoc2022::run(specifiers.days, is_sample)),
        _ => Err(format!("Year {} is not implemented", specifiers.year)),
    }
}

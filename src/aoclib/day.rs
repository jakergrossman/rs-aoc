use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{Read, Write};

#[cfg(feature = "colored-output")]
use colored::*;

use time::Instant;

macro_rules! print_info {
    ($day:ident, $puzzle:expr) => {
        #[cfg(feature = "colored-output")]
        print!("[{} {}, {} {}, {} {}]: ",
               "AoC".yellow(), $day.year,
               "day".bright_cyan(), $day.day,
               "part".bright_cyan(), $puzzle.part
              );

        #[cfg(not(feature = "colored-output"))]
        print!("[{} {}, {} {}, {} {}]: ",
               "AoC", $day.year,
               "day", $day.day,
               "part", $puzzle.part
              );

        std::io::stdout().flush().unwrap();
    }
}

macro_rules! run_solver {
    ($puzzle:expr, $input:expr) => {
        let start = Instant::now();
        let output = ($puzzle.algorithm)($input);
        let elapsed = start.elapsed();
        println!("{}", output);

        let time_taken_str = {
            let mut str = String::new();
            let (d, h, m, s, ms, us, ns) = (
                elapsed.whole_days(),
                elapsed.whole_hours() % 24,
                elapsed.whole_minutes() % 60,
                elapsed.whole_seconds() % 60,
                elapsed.whole_milliseconds() % 1000,
                elapsed.whole_microseconds() % 1000,
                elapsed.whole_nanoseconds() % 1000,
                );

            if d > 0 {
                str.push_str(&format!("{}d ", d));
            }
            if h > 0 {
                str.push_str(&format!("{}h ", h));
            }
            if m > 0 {
                str.push_str(&format!("{}m ", m));
            }
            if s > 0 {
                str.push_str(&format!("{}s ", s));
            }
            if ms > 0 {
                str.push_str(&format!("{}ms ", ms));
            }
            if us > 0 {
                str.push_str(&format!("{}us ", us));
            }
            if ns > 0 {
                str.push_str(&format!("{}ns ", ns));
            }

            str
        };

        #[cfg(feature = "colored-output")]
        println!(" ↪ {} {}", "Finished in".dimmed(), time_taken_str.dimmed().italic());

        #[cfg(not(feature = "colored-output"))]
        println!(" ↪ Finished in {}", time_taken_str);
    };
}

#[macro_export]
macro_rules! run_day_with_serializer {
    ($year:expr, $day:expr, $is_sample:expr, $serializer:ident $(,$algo:ident)*) => {
        #[allow(unused_assignments, unused_variables, unused_mut)]
        {
            let day = AocDay::new_with_serializer($year, $day, $serializer, $is_sample);
            let mut part_idx = 1;
            $(
                {
                    day.run(Solver::new(part_idx, $algo));
                    part_idx += 1;
                }
            )*;
            println!("");
        }
    }
}

#[macro_export]
macro_rules! run_day {
    ($year:expr, $day:expr, $is_sample:expr $(,$algo:ident)*) => {
        let day = AocDay::new($year, $day, $is_sample);
        let mut part_idx = 1;
        $(
            #[allow(unused_assignments)]
            {
                day.run(Solver::new(part_idx, $algo));
                part_idx += 1;
            }
        )*;
        println!("");
    }
}

pub struct AocDay<T> {
    year: i32,
    day: u8,
    input_path: String,
    serializer: fn(String) -> T,
}

#[allow(dead_code)]
impl AocDay<String> {
    pub fn new(year: i32, day: u8, is_sample: bool) -> Self {
        let input_set = if is_sample { "sample" } else { "real" };

        AocDay {
            year,
            day,
            input_path: format!("inputs/{}/{}/day{:02}.txt", year, input_set, day),
            serializer: |x| x
        }
    }
}

#[allow(dead_code)]
impl<T> AocDay<T> {
    pub fn new_with_serializer(year: i32, day: u8, serializer: fn(String) -> T, is_sample: bool) -> Self {
        let input_set = if is_sample { "sample" } else { "real" };

        AocDay {
            year,
            day,
            input_path: format!("inputs/{}/{}/day{:02}.txt", year, input_set, day),
            serializer
        }
    }

    pub fn run(&self, solver: Solver<T, impl Display>) {
        let mut input_file = match OpenOptions::new()
            .read(true)
            .open(&self.input_path) {
                Ok(file) => file,
                Err(e) => panic!("Unable to open {}: {}", &self.input_path, e.to_string())
            };

        let mut contents = String::new();
        input_file.read_to_string(&mut contents).expect("File read error");

        print_info!(self, solver);
        run_solver!(solver, (self.serializer)(contents.to_string()));
    }
}

#[derive(Debug)]
pub struct Solver<T, D> {
    part: u8,
    algorithm: fn(T) -> D
}

impl<T, D: Display> Solver<T, D> {
    pub fn new(part: u8, algorithm: fn(T) -> D) -> Self {
        Solver { part, algorithm }
    }
}

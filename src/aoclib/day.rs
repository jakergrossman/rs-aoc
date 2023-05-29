use std::{fs::OpenOptions, fmt::Display, io::Read, io::Write};

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

/// Create a run method for a single day that serializes the entire input file
macro_rules! aoc_day_with_serializer {
    ($year:literal, $day:literal, $serializer:expr $(,$algo:expr)*) => {
        #[allow(unused_assignments, unused_variables, unused_mut)]
        pub fn run(is_sample: bool) {
            let day = AocDay::new($year, $day, $serializer, is_sample);
            let mut part_idx = 1;

            $(
                {
                    day.run(Solver::new(part_idx, $algo));
                    part_idx += 1;
                }
            )*
            println!("");
        }
    };
}

/// Create a run method for a single day that serializes each line an input
/// file into a vector
macro_rules! aoc_day_with_line_serializer {
    ($year:literal, $day:literal, $line_serializer:expr $(,$algo:expr)*) => {
        aoc_day_with_serializer!(
            $year,
            $day,
            |str: String| str.lines().map($line_serializer).collect()
            $(,$algo)*
            );
    }
}

/// Create a run method for a single day that does not serialize the input
macro_rules! aoc_day {
    ($year:literal, $day:literal $(,$algo:expr)*) => {
        aoc_day_with_serializer!($year, $day, |x| x $(,$algo)*);
    }
}

pub struct AocDay<T> {
    year: i32,
    day: u8,
    input_path: String,
    serializer: fn(String) -> T,
}

#[allow(dead_code)]
impl<T> AocDay<T> {
    pub fn new(year: i32, day: u8, serializer: fn(String) -> T, is_sample: bool) -> Self {
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

macro_rules! run_year {
    ($is_sample:expr, $days:expr, $(($day_id:expr, $fn:expr)),*) => {
        let supported_days = vec![ $($day_id),* ];

        let mut day_table: [fn(bool); 25] = [|_| (); 25];
        $(
            {
                day_table[$day_id-1] = $fn;
            }
         )*;

        use crate::aoclib::cli::DaySpecifier;
        let run_days = match $days {
            DaySpecifier::All => Some(supported_days),
            DaySpecifier::Pick(days) => {
                let unsupported_days: Vec<u8> =
                        days.clone().into_iter()
                        .filter(|d| !supported_days.contains(d))
                        .collect();

                match unsupported_days.len() {
                    0 => Some(days),
                    _ => {
                        println!("Unimplemented days: {:?}", unsupported_days);
                        None
                    },
                }
            }
        };

        if let Some(mut days) = run_days {
            days.sort();
            for d in days {
                day_table[d as usize - 1]($is_sample);
            }
        }
    }
}

pub(crate) use aoc_day_with_serializer;
pub(crate) use aoc_day_with_line_serializer;
pub(crate) use aoc_day;
pub(crate) use run_year;

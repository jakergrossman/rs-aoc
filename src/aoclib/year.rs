use crate::aoclib::*;

macro_rules! define_aoc_entry_point {
    ($($year:literal),*) => {
        use itertools::Itertools;
        use clap::Parser;
        use crate::aoclib::cli::Specifier;
        $( paste::item! { mod [<aoc $year>] ; } )*
        fn main() -> Result<(), String> {
            let specifiers = aoclib::cli::AocCli::parse();
            let defined_years = vec![$($year),*];
            let years = match specifiers.years {
                Specifier::All => defined_years.clone(),
                Specifier::Pick(years) =>  {
                    if !years.iter().all(|y| defined_years.contains(y)) {
                        return Err(format!("Not all years defined"));
                    }

                    years
                }
            };


            for year in years.iter().sorted() {
                match year {
                    $(
                        $year =>  {paste::item! {
                            [<aoc $year>]::run(specifiers.days.clone(), specifiers.sample);
                            println!();
                            println!();
                        } },
                    )*
                    _ => return Err(format!("Year {} not implemented", year))?,
                }
            }

            return Ok(());
        }
    };
}

pub(crate) use define_aoc_entry_point;

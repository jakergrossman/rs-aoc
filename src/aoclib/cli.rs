use clap::Parser;

#[derive(Debug, Clone)]
pub enum DaySpecifier {
    All,
    Pick(Vec<u8>),
}

fn parse_day_specifier(arg: &str) -> Result<DaySpecifier, std::num::ParseIntError> {
    match arg {
        "all" => Ok(DaySpecifier::All),
        _ => {
            let tokens = arg.split(",");
            let mut days = Vec::new();

            for t in tokens {
                let val: u8 = t.parse()?;
                days.push(val);
            }

            Ok(DaySpecifier::Pick(days))
        }
    }
}

#[derive(Debug, Parser)]
pub(crate) struct AocCli {
    #[arg(long, short, num_args=0, help="Use sample inputs")]
    pub sample: bool,

    #[arg(long, short, help="Select AoC year")]
    pub year: i32,

    #[arg(long, short, default_value="all", value_parser = parse_day_specifier)]
    pub days: DaySpecifier,
}

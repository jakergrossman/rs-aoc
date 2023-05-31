use clap::Parser;

#[derive(Debug, Clone)]
pub enum Specifier {
    All,
    Pick(Vec<u16>),
}

fn parse_day_specifier(arg: &str) -> Result<Specifier, std::num::ParseIntError> {
    match arg {
        "all" => Ok(Specifier::All),
        _ => {
            let tokens = arg.split(",");
            let mut days = Vec::new();

            for t in tokens {
                let val: u16 = t.parse()?;
                days.push(val);
            }

            Ok(Specifier::Pick(days))
        }
    }
}

#[derive(Debug, Parser)]
pub(crate) struct AocCli {
    #[arg(long, short, num_args=0, help="Use sample inputs")]
    pub sample: bool,

    #[arg(long, short, help="Select AoC year(s)", default_value="all", value_parser = parse_day_specifier)]
    pub years: Specifier,

    #[arg(long, short, default_value="all", value_parser = parse_day_specifier)]
    pub days: Specifier,
}

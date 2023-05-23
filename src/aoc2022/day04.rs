use crate::{aoclib::day::*, run_day};

struct ElfPair {
    elf1: (u128, u128),
    elf2: (u128, u128),
}

impl ElfPair {
    fn new(line: &str) -> ElfPair {
        let pairs: Vec<_> 
            = line.split(",").map(|range| range.split("-").collect::<Vec<_>>()).collect();

        ElfPair {
            elf1: 
                (u128::from_str_radix(pairs[0][0], 10).unwrap(), 
                 u128::from_str_radix(pairs[0][1], 10).unwrap()),
            elf2:
                (u128::from_str_radix(pairs[1][0], 10).unwrap(), 
                 u128::from_str_radix(pairs[1][1], 10).unwrap()),
        }
    }

    fn is_fully_overlapping(&self) -> bool {
        ((self.elf1.0 <= self.elf2.0) && (self.elf1.1 >= self.elf2.1)) ||
        ((self.elf2.0 <= self.elf1.0) && (self.elf2.1 >= self.elf1.1))
    }

    fn is_overlapping(&self) -> bool {
        (self.elf1.0 <= self.elf2.1) && (self.elf2.0 <= self.elf1.1)
    }
}

fn serialize(s: String) -> Vec<ElfPair> {
    s.lines().map(ElfPair::new).collect()
}

pub fn run(is_sample: bool) {
    run_day!(2022, 4, is_sample, serialize, (part1, part2));
}

fn part1(pairs: Vec<ElfPair>) -> u128 {
    pairs.iter().filter(|pair| pair.is_fully_overlapping()).count() as u128
}

fn part2(pairs: Vec<ElfPair>) -> u128 {
    pairs.iter().filter(|pair| pair.is_overlapping()).count() as u128
}

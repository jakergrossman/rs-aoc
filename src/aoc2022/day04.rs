use std::ops::RangeInclusive;

use crate::{aoclib::day::*, run_day};

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_range_reflective(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps(&self, other: &Self) -> bool;

    fn overlaps_reflective(&self, other: &Self) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

struct ElfPair {
    elf1: RangeInclusive<u128>,
    elf2: RangeInclusive<u128>
}

impl ElfPair {
    fn new(line: &str) -> Option<ElfPair> {
        let (elf1_str, elf2_str) = line.split_once(",")?;
        let (elf1_start, elf1_stop) = elf1_str.split_once("-")?;
        let (elf2_start, elf2_stop) = elf2_str.split_once("-")?;

        let elf1 = elf1_start.parse().ok()?..=elf1_stop.parse().ok()?;
        let elf2 = elf2_start.parse().ok()?..=elf2_stop.parse().ok()?;

        Some(ElfPair { elf1, elf2 })
    }
}

fn serialize(s: String) -> Vec<ElfPair> {
    s.lines().map(|l| ElfPair::new(l).expect("Invalid input")).collect()
}

pub fn run(is_sample: bool) {
    run_day!(2022, 4, is_sample, serialize, (part1, part2));
}

fn part1(pairs: Vec<ElfPair>) -> u128 {
    pairs.iter().filter(|pair| pair.elf1.contains_range_reflective(&pair.elf2)).count() as u128
}

fn part2(pairs: Vec<ElfPair>) -> u128 {
    pairs.iter().filter(|pair| pair.elf1.overlaps_reflective(&pair.elf2)).count() as u128
}

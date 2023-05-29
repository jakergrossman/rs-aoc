use crate::aoclib::day::*;
use itertools::Itertools;

/// Parse elf packs from input
/// Each elf pack is a series of integers,
/// one per line, with each elf pack 
/// separated by an empty line.
pub fn elf_packs(s: String) -> Vec<u128> {
    s.trim().split("\n\n").map(|bag| {
        bag.split("\n")
            .map(|line| line.parse::<u128>().expect("Invalid Input"))
            .sum()
    }).collect()
}

fn solution<const ELVES: usize>(pack_sums: Vec<u128>) -> u128 {
    pack_sums.iter().sorted().rev().take(ELVES).sum()
}

aoc_day_with_serializer!(2022, 1, elf_packs, solution::<1>, solution::<3>);

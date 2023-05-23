use crate::{aoclib::day::*, run_day};

/// Parse elf packs from input
/// Each elf pack is a series of integers,
/// one per line, with each elf pack 
/// separated by an empty line.
pub fn elf_packs(s: String) -> Vec<u128> {
    s.trim().split("\n\n").map(|bag| {
        bag.split("\n")
            .map(|line| u128::from_str_radix(line, 10).expect("Invalid Input"))
            .sum()
    }).collect()
}

pub fn run(is_sample: bool) {
    run_day!(2022, 1, is_sample, elf_packs, (part1, part2));
}

fn part1(pack_sums: Vec<u128>) -> u128 {
    *pack_sums.iter().max()
        .expect("Empty pack")
}

fn part2(pack_sums: Vec<u128>) -> u128 {
    let mut copy = pack_sums.clone();
    copy.sort();
    copy.iter().rev().take(3).sum()
}

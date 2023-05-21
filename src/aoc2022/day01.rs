use crate::aoclib::day::{AocDay, Solver};

/// Parse elf packs from input
/// Each elf pack is a series of integers,
/// one per line, with each elf pack 
/// separated by an empty line.
pub fn elf_packs(s: String) -> Vec<u128> {
    s.split("\n\n").map(|bag| {
        bag.split("\n")
            .map(|line| u128::from_str_radix(line, 10).expect("Invalid Input"))
            .sum()
    }).collect()
}

pub fn run(sample: bool) {
    let day = AocDay::new_with_serializer(2022, 1, elf_packs, sample);
    let part1 = Solver::new(1, part1);
    let part2 = Solver::new(2, part2);

    day.run(part1);
    day.run(part2);
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

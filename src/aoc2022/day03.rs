use crate::aoclib::day::*;

use std::collections::HashSet;

aoc_day!(2022, 3, part1, part2);

// TODO:
// not happy with the performance of this and also
// not convinced it's idiomatic. Revisit this
// with more experience.

fn part1(s: String) -> u128 {
    let sets = s.lines().map(compartmentize);
    let uniq: Vec<char> = sets.map(uniq).collect();

    uniq.iter().map(priority).sum()
}

fn part2(s: String) -> u128 {
    let mut sum = 0;
    let sacks: Vec<&str> = s.lines().collect();

    for i in (0..sacks.len()).step_by(3) {
        let elf1: HashSet<char> = HashSet::from_iter(sacks[i].chars().into_iter()); let elf2: HashSet<char> = HashSet::from_iter(sacks[i+1].chars().into_iter());
        let elf3: HashSet<char> = HashSet::from_iter(sacks[i+2].chars().into_iter());

        let mut inter = elf1.iter().filter(|k| elf2.contains(k)).filter(|k| elf3.contains(k));

        sum += priority(inter.next().unwrap());
    }


    sum
}

fn compartmentize(sack: &str) -> (HashSet<char>, HashSet<char>) {
    let comp1 = &sack[0..sack.len()/2];
    let comp2 = &sack[sack.len()/2..];

    (HashSet::from_iter(comp1.chars()), HashSet::from_iter(comp2.chars()))
}

fn uniq(sacks: (HashSet<char>, HashSet<char>)) -> char {
    *sacks.0.intersection(&sacks.1).nth(0).unwrap()
}

fn priority(c: &char) -> u128 {
    let p = match c {
        'a'..='z' => (*c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (*c as u32) - ('A' as u32) + 27,
        _ => panic!("Non-alphabetic character found"),
    };

    p as u128
}

use crate::aoclib::day::*;
use crate::run_day_with_serializer;

use std::str::Lines;

#[derive(Debug)]
struct MoveDescriptor {
    count: usize,
    from: usize,
    to: usize
}

impl MoveDescriptor {
    fn new(line: &str) -> MoveDescriptor {
        let mut tokens = line.splitn(6, ' ');

        assert!(tokens.clone().count() == 6, "Malformed move descriptor input: {}", line);

        MoveDescriptor {
            count: usize::from_str_radix(tokens.nth(1).unwrap(), 10)
                .expect(format!("Malformed move descriptor input: {}", line).as_str()),
            from: usize::from_str_radix(tokens.nth(1).unwrap(), 10)
                .expect(format!("Malformed move descriptor input: {}", line).as_str()),
            to: usize::from_str_radix(tokens.nth(1).unwrap(), 10)
                .expect(format!("Malformed move descriptor input: {}", line).as_str()),
        }
    }
}

fn serialize_stacks(lines: Lines) -> Vec<Vec<char>> {
    let mut v: Vec<Vec<char>> = lines.map(|n| n.chars().collect()).collect();
    v.pop();

    let mut stacks: Vec<Vec<char>> = vec![];

    for i in (1..v[0].len()).step_by(4) {
        let mut stack = vec![];
        for j in 0..v.len() {
            if v[j][i] != ' ' {
                stack.push(v[j][i]);
            }
        }

        stack.reverse();
        stacks.push(stack);
    }

    stacks
}

fn serialize_moves(lines: Lines) -> Vec<MoveDescriptor> {
    lines.map(|s| MoveDescriptor::new(&s)).collect()
}

fn serializer(s: String) -> (Vec<Vec<char>>, Vec<MoveDescriptor>) {
    let split: Vec<&str> = s.split("\n\n").collect();

    let stacks = serialize_stacks(split[0].lines());
    let moves = serialize_moves(split[1].lines());

    (stacks, moves)
}

pub fn run(is_sample: bool) {
    run_day_with_serializer!(2022, 5, is_sample, serializer, part1, part2);
}

fn part1(input: (Vec<Vec<char>>, Vec<MoveDescriptor>)) -> String {
    let mut stacks = input.0;
    let moves = input.1;

    for m in moves {
        let from_size = stacks[m.from-1].len();
        let mut temp: Vec<char> 
            = stacks[m.from-1].split_off(from_size-m.count);
        temp.reverse();
        stacks[m.to-1].append(&mut temp);
    }

    stacks.iter_mut().map(|n| n.last().unwrap()).collect()
}

fn part2(input: (Vec<Vec<char>>, Vec<MoveDescriptor>)) -> String {
    let mut stacks = input.0;
    let moves = input.1;

    for m in moves {
        let from_size = stacks[m.from-1].len();
        let mut temp: Vec<char> 
            = stacks[m.from-1].split_off(from_size-m.count);
        stacks[m.to-1].append(&mut temp);
    }

    stacks.iter_mut().map(|n| n.last().unwrap()).collect()
}

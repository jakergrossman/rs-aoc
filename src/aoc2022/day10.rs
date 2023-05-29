use crate::aoclib::day::*;

#[derive(Debug)]
enum Instruction {
    AddX(i128),
    NoOp,
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let mnemonic = &s[0..4];

        match mnemonic {
            "addx" => Instruction::AddX(i128::from_str_radix(&s[5..], 10).unwrap()),
            "noop" => Instruction::NoOp,
            _ => panic!("Invalid instruction")
        }
    }
}

fn parse_file(s: String) -> Vec<Instruction> {
    s.lines().map(Instruction::parse).collect()
}

// the output of part two is textual, and entirely based
// on part one. the combined result appears as a single
// string from part one that appears to be both the
// part one AND part two solution
aoc_day_with_serializer!(2022, 10, parse_file, solution);

fn significant(n: usize) -> bool {
    (n + 20) % 40 == 0
}

fn solution(ops: Vec<Instruction>) -> String {
    let mut register: i128 = 1;
    let mut pc = 0;
    let mut sum = 0;
    let mut s: String = String::new();
    for op in ops {
        let (new_value, cycles) = match op {
            Instruction::AddX(val) => (register + val, 2),
            Instruction::NoOp => (register, 1),
        };

        for _ in 0..cycles {
            if significant((pc+1) as usize) {
                sum += pc * register;
            }

            if (register-1..=register+1).contains(&(pc % 40)) {
                s += "█";
            } else {
                s += " ";
            }

            pc += 1;
            if pc % 40 == 0 {
                s += "\n";
            }
        }

        register = new_value;
    }
    
    format!("{}\n{}", sum, s)
}

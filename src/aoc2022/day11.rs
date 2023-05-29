use crate::aoclib::day::*;

#[derive(Clone)]
enum WorryValue {
    Old,
    Num(u128),
}

impl WorryValue {
    fn value(&self, old: u128) -> u128 {
        match self {
            WorryValue::Num(n) => *n,
            WorryValue::Old => old
        }
    }
}

#[derive(Clone)]
enum Operation {
    Add(WorryValue),
    Mul(WorryValue),
}

impl Operation {
    fn apply(&self, old: u128) -> u128 {
        match &self {
            Operation::Add(n) => old + n.value(old),
            Operation::Mul(n) => old * n.value(old),
        }
    }
}

#[derive(Clone)]
struct PassTest {
    divisor: u128,
    true_monkey: usize,
    false_monkey: usize
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    pass_test: PassTest,
}

fn parse(s: String) -> Option<Vec<Monkey>> {
    let mut monkeys = Vec::new();

    for block in s.split("\n\n") {
        let mut lines = block.lines().skip(1); // drop monkey label
        let (_, items) = lines.next()?.split_once(": ")?;
        let items = items
            .split_terminator(", ")
            .filter_map(|s| s.parse().ok())
            .collect();

        let (_, operation) = lines.next()?.split_once("= old ")?;
        let (operator, operand) = operation.split_once(" ")?;
        let operand = match operand {
            "old" => WorryValue::Old,
            _ => WorryValue::Num(operand.parse().ok()?)
        };
        let operation = match operator {
            "+" => Operation::Add(operand),
            "*" => Operation::Mul(operand),
            _ => panic!("Invalid input")
        };


        let (_, divisor) = lines.next()?.rsplit_once(" ")?;
        let (_, true_monkey) = lines.next()?.rsplit_once(" ")?;
        let (_, false_monkey) = lines.next()?.rsplit_once(" ")?;
        let divisor = divisor.parse().ok()?;
        let true_monkey = true_monkey.parse().ok()?;
        let false_monkey = false_monkey.parse().ok()?;

        let pass_test = PassTest { divisor, true_monkey, false_monkey };

        monkeys.push(Monkey { items, operation, pass_test });
    }

    Some(monkeys)
}

fn solution<const PRE_SCALER: u128, const ROUNDS: usize>(ms: Vec<Monkey>) -> u128 {
    let mut monkeys = ms;
    let mut inspections = vec![0; monkeys.len()];
    let common_multiple: u128 = monkeys.iter().map(|m| m.pass_test.divisor).product();

    for _ in 0..ROUNDS {
        for idx in 0..monkeys.len() {
            let items: Vec<u128> = monkeys[idx].items.drain(..).collect();
            let monkey = monkeys[idx].clone();

            for worry in items {
                inspections[idx] += 1;

                let new_worry = monkey.operation.apply(worry) / PRE_SCALER % common_multiple;

                // only the remainder matters if we're counting inspections only,
                // so we can avoid giant numbers
                let idx = if new_worry % monkey.pass_test.divisor == 0 {
                    monkey.pass_test.true_monkey
                } else {
                    monkey.pass_test.false_monkey
                };

                monkeys[idx].items.push(new_worry);
            }
        }
    }

    inspections.sort_unstable();
    inspections.iter().rev().take(2).product()
}

aoc_day_with_serializer!(2022, 11, |s| parse(s).expect("Malformed input"), solution::<3, 20>, solution::<1, 10_000>);

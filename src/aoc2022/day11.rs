use crate::{aoclib::day::*, run_day};

#[derive(Copy, Clone, Debug)]
enum WorryFn {
    SelfAdd,
    SelfMul,
    OtherAdd(u64),
    OtherMul(u64),
}

#[derive(Copy, Clone, Debug)]
struct MonkeyFlow(u64, usize, usize);

#[derive(Debug, Clone)]
struct Monkey {
    worries: Vec<u64>,
    worry_fxn: WorryFn,
    flow: MonkeyFlow,
    inspections: u64,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            worries: Vec::new(),
            worry_fxn: WorryFn::SelfAdd,
            flow: MonkeyFlow { 0: 0, 1: 0, 2: 0 },
            inspections: 0,
        }
    }

    fn starting_worries(&self, s: &str) -> Self {
        let rhs = &s["  Starting items: ".len()..];
        let worries =
            rhs.split(", ").map(|n| u64::from_str_radix(n, 10).unwrap()).collect();

        Monkey {
            worries,
            ..*self
        }
    }

    fn operation(&self, s: &str) -> Self {
        let rhs = &s["  Operation: ".len()..];
        let tokens: Vec<&str> = rhs.split(" ").collect();
        let op = tokens[3];
        let op_rhs = tokens[4];

        let worry_fxn = match op_rhs {
            "old" => if op == "+" { WorryFn::SelfAdd } else { WorryFn::SelfMul },
            _ =>  {
                let val = u64::from_str_radix(op_rhs, 10).unwrap();
                if op == "+" { WorryFn::OtherAdd(val) } else { WorryFn::OtherMul(val) }
            }
        };

        Monkey {
            worries: self.worries.clone(),
            worry_fxn,
            ..*self
        }
    }

    fn flow(&self, s: Vec<&str>) -> Self {
        let divisor = u64::from_str_radix(&s[0]["  Test: divisible by ".len()..], 10).unwrap();
        let true_flow = usize::from_str_radix(&s[1]["    If true: throw to monkey ".len()..], 10).unwrap();
        let false_flow = usize::from_str_radix(&s[2]["    If false: throw to monkey ".len()..], 10).unwrap();

        let flow = MonkeyFlow(divisor, true_flow, false_flow);

        Monkey {
            worries: self.worries.clone(),
            flow,
            ..*self
        }
    }

    fn eval_worry(worry: u64, fxn: WorryFn) -> u64 {
        match fxn {
            WorryFn::SelfAdd => worry + worry,
            WorryFn::SelfMul => worry * worry,
            WorryFn::OtherAdd(o) => worry + o,
            WorryFn::OtherMul(o) => worry * o,
        }
    }

    fn eval_flow(worry: u64, flow: MonkeyFlow) -> usize {
        let MonkeyFlow(divisor, true_monkey, false_monkey) = flow;

        if worry % divisor == 0 { true_monkey } else { false_monkey }
    }

    fn eval_monkey(turn: usize, pre_scaler: u64, common_multiple: u64, monkeys: &mut Vec<Self>) {
        let worries = monkeys[turn].worries.clone();
        let worry_fxn = monkeys[turn].worry_fxn.clone();
        let flow = monkeys[turn].flow.clone();

        for worry in worries.iter() {
            monkeys[turn].inspections += 1;
            let new_worry = Monkey::eval_worry(*worry, worry_fxn) / pre_scaler % common_multiple;
            let new_monkey = Monkey::eval_flow(new_worry, flow);
            monkeys[new_monkey].worries.push(new_worry);
        }

        monkeys[turn].worries.clear();
    }
}

fn serialize(s: String) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for desc in s.split("\n\n") {
        let mut lines = desc.lines();
        lines.next(); // drop header
        let monkey 
            = Monkey::new()
                .starting_worries(lines.next().unwrap())
                .operation(lines.next().unwrap())
                .flow(lines.take(3).collect());

        monkeys.push(monkey);
    }

    monkeys
}

pub fn run(is_sample: bool) {
    let part1 = solution::<20, 3>;
    let part2 = solution::<10000, 1>;
    run_day!(2022, 11, is_sample, serialize, (part1, part2));
}

fn solution<const ROUNDS: usize, const WORRY_SCALER: u64>(ms: Vec<Monkey>) -> u64 {
    let mut monkeys = ms.clone();
    let common_multiple: u64 = monkeys.iter().map(|m| {
        let MonkeyFlow(div, _, _) = m.flow;
        div
    }).product();

    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            Monkey::eval_monkey(i, WORRY_SCALER, common_multiple, &mut monkeys);
        }
    }

    let mut inspections: Vec<u64> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();

    inspections.iter().rev().take(2).product()
}

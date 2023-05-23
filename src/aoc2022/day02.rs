use crate::{aoclib::day::*, run_day};

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors
}

enum Outcome {
    Lose,
    Draw,
    Win
}

impl Move {
    fn absolute_move(c: char) -> Self {
        match c {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("Unknown move")
        }
    }

    fn win(opponent: Move) -> Self {
        match opponent {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn lose(opponent: Move) -> Self {
        match opponent {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn outcome(opponent: Move, you: Move) -> Outcome {
        if opponent == you {
            Outcome::Draw
        } else if Self::win(opponent) == you {
            Outcome::Lose
        } else {
            Outcome::Win
        }
    }

    fn score(play: Move) -> u128 {
        match play {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn score_match(opponent: Move, you: Move) -> u128 {
        Self::score(you) + match Self::outcome(opponent, you) {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6
        }
    }
}

struct MoveSet {
    opponent: Move,
    absolute_based: Move,
    outcome_based: Move
}

/// input serializer
fn moves(s: String) -> Vec<MoveSet> {
    s.lines().map(|line| {
        let opponent_char = line.chars().nth(0).expect("Expected two moves");
        let you_char = line.chars().nth(2).expect("Expected two moves");

        let opponent = Move::absolute_move(opponent_char);
        let absolute_based = Move::absolute_move(you_char);
        let outcome_based = match you_char {
            'X' => Move::lose(opponent),
            'Y' => opponent,
            'Z' => Move::win(opponent),
            _ => panic!("Unknown move")
        };

        MoveSet {
            opponent,
            absolute_based,
            outcome_based
        }
    }).collect()
}

pub fn run(is_sample: bool) {
    run_day!(2022, 2, is_sample, moves, (part1, part2));
}

fn part1(moves: Vec<MoveSet>) -> u128 {
    moves
        .iter()
        .map(|m| Move::score_match(m.opponent, m.absolute_based))
        .sum()
}

fn part2(moves: Vec<MoveSet>) -> u128 {
    moves
        .iter()
        .map(|m| Move::score_match(m.opponent, m.outcome_based))
        .sum()
}

use crate::aoclib::day::*;

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
    fn absolute_move(s: &str) -> Self {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Unknown move")
        }
    }

    fn wins_to(opponent: Move) -> Self {
        match opponent {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn loses_to(opponent: Move) -> Self {
        match opponent {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn outcome(opponent: Move, you: Move) -> Outcome {
        if opponent == you {
            Outcome::Draw
        } else if Self::wins_to(opponent) == you {
            Outcome::Win
        } else {
            Outcome::Lose
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

fn parse_move(s: &str) -> MoveSet {
    let (opponent, you) = s.split_once(" ").expect("Malformed input");

    let opponent = Move::absolute_move(opponent);
    let absolute_based = Move::absolute_move(you);
    let outcome_based = match you {
        "X" => Move::loses_to(opponent),
        "Y" => opponent,
        "Z" => Move::wins_to(opponent),
        _ => panic!("Unknown move")
    };

    MoveSet { opponent, absolute_based, outcome_based }
}

aoc_day_with_line_serializer!(2022, 2, parse_move, part1, part2);

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

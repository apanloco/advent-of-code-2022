// --- Day 2: Rock Paper Scissors ---
// part1: map your shape to specific shape
// part2: map your shape to specific outcome

use crate::error::Error;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

pub struct Round {
    lhs: Shape,
    rhs: Shape,
}

pub enum Outcome {
    Win,
    Loose,
    Draw,
}

pub struct Strategy {
    pub rounds: Vec<Round>,
}

pub enum MappingKind {
    Shape(Shape),
    Outcome(Outcome),
}

impl Shape {
    pub fn win_over(&self, input: Shape) -> Shape {
        match input {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    pub fn loose_over(&self, input: Shape) -> Shape {
        match input {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    pub fn draw(&self, input: Shape) -> Shape {
        input
    }
}

impl Shape {
    pub fn score(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl Round {
    pub fn outcome(mine: Shape, other: Shape) -> Outcome {
        if mine == other {
            return Outcome::Draw;
        }

        if mine == Shape::Rock && other == Shape::Scissors {
            return Outcome::Win;
        }

        if mine == Shape::Paper && other == Shape::Rock {
            return Outcome::Win;
        }

        if mine == Shape::Scissors && other == Shape::Paper {
            return Outcome::Win;
        }

        Outcome::Loose
    }

    pub fn score(&self) -> usize {
        let shape_score = self.rhs.score();
        let win_score = match Round::outcome(self.rhs, self.lhs) {
            Outcome::Win => 6,
            Outcome::Loose => 0,
            Outcome::Draw => 3,
        };
        shape_score + win_score
    }
}

impl Strategy {
    pub fn from_str_with_mapping(input: &str, mapping: &HashMap<String, MappingKind>) -> Result<Self, Error> {
        let mut rounds = Vec::new();
        for line in input.trim_start().trim_end().lines() {
            let two_tokens_error = || Error::General(format!("should be two tokens: {}", line));
            let mut tokens = line.trim_start().trim_end().split(' ');
            let lhs_token = tokens.next().ok_or_else(two_tokens_error)?;
            let rhs_token = tokens.next().ok_or_else(two_tokens_error)?;
            let lhs_shape = match lhs_token {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => panic!("invalid lhs token: {}", lhs_token),
            };
            let kind = mapping
                .get(rhs_token)
                .ok_or_else(|| Error::General(format!("no mapping for: {}", rhs_token)))?;

            let rhs_shape = match kind {
                MappingKind::Shape(shape) => *shape,
                MappingKind::Outcome(outcome) => match outcome {
                    Outcome::Win => lhs_shape.win_over(lhs_shape),
                    Outcome::Loose => lhs_shape.loose_over(lhs_shape),
                    Outcome::Draw => lhs_shape.draw(lhs_shape),
                },
            };
            rounds.push(Round {
                lhs: lhs_shape,
                rhs: rhs_shape,
            });
        }
        Ok(Strategy { rounds })
    }
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
A Y
B X
C Z"#;

    let mut mapping_part1: HashMap<String, MappingKind> = HashMap::new();
    mapping_part1.insert("X".to_owned(), MappingKind::Shape(Shape::Rock));
    mapping_part1.insert("Y".to_owned(), MappingKind::Shape(Shape::Paper));
    mapping_part1.insert("Z".to_owned(), MappingKind::Shape(Shape::Scissors));

    let mut mapping_part2: HashMap<String, MappingKind> = HashMap::new();
    mapping_part2.insert("X".to_owned(), MappingKind::Outcome(Outcome::Loose));
    mapping_part2.insert("Y".to_owned(), MappingKind::Outcome(Outcome::Draw));
    mapping_part2.insert("Z".to_owned(), MappingKind::Outcome(Outcome::Win));

    let strategy: Strategy = Strategy::from_str_with_mapping(input, &mapping_part1)?;

    assert_eq!(strategy.rounds.len(), 3);
    assert_eq!(strategy.rounds[0].score(), 8);
    assert_eq!(strategy.rounds[1].score(), 1);
    assert_eq!(strategy.rounds[2].score(), 6);
    let total_score: usize = strategy.rounds.iter().map(|r| r.score()).sum();
    assert_eq!(total_score, 15);

    let strategy: Strategy = Strategy::from_str_with_mapping(input, &mapping_part2)?;

    assert_eq!(strategy.rounds.len(), 3);
    assert_eq!(strategy.rounds[0].score(), 4);
    assert_eq!(strategy.rounds[1].score(), 1);
    assert_eq!(strategy.rounds[2].score(), 7);
    let total_score: usize = strategy.rounds.iter().map(|r| r.score()).sum();
    assert_eq!(total_score, 12);

    let file_contents = std::fs::read_to_string("input/day2")?;

    let strategy: Strategy = Strategy::from_str_with_mapping(&file_contents, &mapping_part1)?;
    let total_score: usize = strategy.rounds.iter().map(|r| r.score()).sum();
    assert_eq!(total_score, 9651);

    let strategy: Strategy = Strategy::from_str_with_mapping(&file_contents, &mapping_part2)?;
    let total_score: usize = strategy.rounds.iter().map(|r| r.score()).sum();
    assert_eq!(total_score, 10560);

    Ok(())
}

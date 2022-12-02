use std::collections::HashMap;
use crate::error::Error;

#[derive(Clone, Copy, PartialEq)]
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
    pub fn get_win_shape(&self, input: Shape) -> Shape {
        match input {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    pub fn get_loose_shape(&self, input: Shape) -> Shape {
        match input {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    pub fn get_draw_shape(&self, input: Shape) -> Shape {
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

        return Outcome::Loose;
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
    pub fn from_str_with_mapping(input: &str, mapping: &HashMap<String, MappingKind>) -> Self {
        let mut rounds = Vec::new();
        for line in input.trim_start().trim_end().lines() {
            let lhs_plus_rhs: Vec<_> = line.trim_start().trim_end().split(' ').collect();
            if lhs_plus_rhs.len() != 2 {
                panic!("invalid input (should be two tokens)");
            }
            let lhs_token = lhs_plus_rhs[0];
            let rhs_token = lhs_plus_rhs[1];
            let lhs_shape = match lhs_token {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => panic!("invalid lhs token: {}", lhs_token)
            };
            let kind = mapping.get(rhs_token).expect(&format!("mapping not found for rhs token: {}", rhs_token));

            let rhs_shape = match kind {
                MappingKind::Shape(shape) => {
                    *shape
                }
                MappingKind::Outcome(outcome) => {
                    match outcome {
                        Outcome::Win => lhs_shape.get_win_shape(lhs_shape),
                        Outcome::Loose => lhs_shape.get_loose_shape(lhs_shape),
                        Outcome::Draw => lhs_shape.get_draw_shape(lhs_shape),
                    }
                }
            };
            let round = Round {
                lhs: lhs_shape,
                rhs: rhs_shape,
            };
            rounds.push(round);
        }
        Strategy {
            rounds,
        }
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

    let strategy: Strategy = Strategy::from_str_with_mapping(input, &mapping_part1);

    assert_eq!(strategy.rounds.len(), 3);
    assert_eq!(strategy.rounds[0].score(), 8);
    assert_eq!(strategy.rounds[1].score(), 1);
    assert_eq!(strategy.rounds[2].score(), 6);
    let total_score: usize = strategy.rounds.iter().map(|r| r.score()).sum();
    assert_eq!(total_score, 15);

    let strategy: Strategy = Strategy::from_str_with_mapping(input, &mapping_part2);

    assert_eq!(strategy.rounds.len(), 3);
    assert_eq!(strategy.rounds[0].score(), 4);
    assert_eq!(strategy.rounds[1].score(), 1);
    assert_eq!(strategy.rounds[2].score(), 7);
    let total_score: usize = strategy.rounds.iter().map(|r| r.score()).sum();
    assert_eq!(total_score, 12);

    let strategy: Strategy = Strategy::from_str_with_mapping(&std::fs::read_to_string("input/day2")?, &mapping_part1);
    let total_score: usize = strategy.rounds.iter().map(|r| r.score()).sum();
    assert_eq!(total_score, 9651);

    let strategy: Strategy = Strategy::from_str_with_mapping(&std::fs::read_to_string("input/day2")?, &mapping_part2);
    let total_score: usize = strategy.rounds.iter().map(|r| r.score()).sum();
    assert_eq!(total_score, 10560);

    Ok(())
}

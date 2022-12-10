// --- Day 9: Rope Bridge ---
// part1: (2 knots on rope) How many positions does the tail of the rope visit at least once?
// part2: (10 knots on rope) How many positions does the tail of the rope visit at least once?

use crate::error::Error;
use std::collections::HashSet;
use std::str::FromStr;

type Pos = (i64, i64);

pub fn too_long(lhs: &Pos, rhs: &Pos) -> bool {
    (lhs.0 - rhs.0).abs() > 1 || (lhs.1 - rhs.1).abs() > 1
}

pub fn move_diagonally_towards(from: &Pos, towards: &Pos) -> Pos {
    (from.0 + (towards.0 - from.0).signum(), from.1 + (towards.1 - from.1).signum())
}

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn from_char(c: char) -> Result<Direction, Error> {
        Ok(match c {
            'U' => Direction::UP,
            'R' => Direction::RIGHT,
            'D' => Direction::DOWN,
            'L' => Direction::LEFT,
            _ => return Err(Error::General(format!("invalid direction: {}", c))),
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Instruction {
    direction: Direction,
    amount: usize,
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Instruction {
            direction: Direction::from_char(s.chars().next().ok_or_else(|| Error::General("empty instruction".to_string()))?)?,
            amount: s[2..].parse()?,
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Instructions {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct State {
    knots: Vec<Pos>,
    tails_visited: HashSet<Pos>,
}

impl State {
    pub fn new(num_knots: usize) -> Self {
        Self {
            knots: vec![Pos::default(); num_knots],
            tails_visited: HashSet::from([Pos::default()]),
        }
    }

    pub fn apply(&mut self, i: &Instruction) -> Result<(), Error> {
        let no_knots_error = || Error::General("no knots".to_string());
        for _ in 0..i.amount {
            let head = self.knots.first_mut().ok_or_else(no_knots_error)?;
            match i.direction {
                Direction::UP => {
                    head.1 -= 1;
                }
                Direction::RIGHT => {
                    head.0 += 1;
                }
                Direction::DOWN => {
                    head.1 += 1;
                }
                Direction::LEFT => {
                    head.0 -= 1;
                }
            }
            for index in 1..self.knots.len() {
                if too_long(&self.knots[index], &self.knots[index - 1]) {
                    self.knots[index] = move_diagonally_towards(&self.knots[index], &self.knots[index - 1]);
                    if index == self.knots.len() - 1 {
                        self.tails_visited.insert(*self.knots.last().ok_or_else(no_knots_error)?);
                    }
                }
            }
        }
        Ok(())
    }
}

impl Instructions {
    pub fn follow(&self, num_knots: usize) -> Result<State, Error> {
        let mut state = State::new(num_knots);
        for instruction in self.instructions.iter() {
            state.apply(instruction)?;
        }
        Ok(state)
    }
}

impl FromStr for Instructions {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Instructions {
            instructions: s
                .trim_start()
                .trim_end()
                .lines()
                .map(|l| l.parse())
                .collect::<Result<Vec<Instruction>, Error>>()?,
        })
    }
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
    let instructions: Instructions = input.parse()?;
    assert_eq!(instructions.instructions.len(), 8);
    assert_eq!(
        instructions.instructions[0],
        Instruction {
            direction: Direction::RIGHT,
            amount: 4,
        }
    );
    assert_eq!(
        instructions.instructions[1],
        Instruction {
            direction: Direction::UP,
            amount: 4,
        }
    );
    let result = instructions.follow(2)?;
    assert_eq!(result.tails_visited.len(), 13);
    let result = instructions.follow(10)?;
    assert_eq!(result.tails_visited.len(), 1);

    let input = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
    let instructions: Instructions = input.parse()?;
    let result = instructions.follow(10)?;
    assert_eq!(result.tails_visited.len(), 36);

    let instructions: Instructions = std::fs::read_to_string("input/day9")?.parse()?;
    let result = instructions.follow(2)?;
    assert_eq!(result.tails_visited.len(), 6470);
    let result = instructions.follow(10)?;
    assert_eq!(result.tails_visited.len(), 2658);

    Ok(())
}

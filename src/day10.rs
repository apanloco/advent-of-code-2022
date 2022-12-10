// --- Day 10: Cathode-Ray Tube ---
// part1: What is the sum of these six signal strengths?
// part2: What eight capital letters appear on your CRT?

use crate::error::Error;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub struct States {
    states: Vec<State>,
}

impl States {
    pub fn cycle(&self, index: usize) -> &State {
        &self.states[index - 1]
    }
}

impl Debug for States {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for state in self.states.iter() {
            f.write_fmt(format_args!("{:?}\n", state))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct State {
    pub cycle: usize,
    pub x: i64,
    pub change: i64,
    pub instruction: Instruction,
}

impl State {
    pub fn signal_strength(&self) -> i64 {
        self.cycle as i64 * self.x
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Noop(),
    AddX(i64),
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            Ok(Instruction::Noop())
        } else if s.starts_with("addx") {
            Ok(Instruction::AddX(s[5..].parse()?))
        } else {
            Err(Error::General(format!("invalid instruction: {}", s)))
        }
    }
}

pub fn execute(input: &str) -> Result<States, Error> {
    let mut states: Vec<State> = vec![State {
        cycle: 0,
        x: 1,
        change: 0,
        instruction: Instruction::Noop(),
    }];
    for instruction in input.trim_start().trim_end().lines() {
        let instruction = instruction.parse::<Instruction>()?;
        match instruction {
            Instruction::Noop() => {
                let last = states.last().unwrap();
                states.push(State {
                    cycle: last.cycle + 1,
                    x: last.x + last.change,
                    change: 0,
                    instruction,
                })
            }
            Instruction::AddX(amount) => {
                let last = states.last().unwrap();
                states.push(State {
                    cycle: last.cycle + 1,
                    x: last.x + last.change,
                    change: 0,
                    instruction,
                });
                let last = states.last().unwrap();
                states.push(State {
                    cycle: last.cycle + 1,
                    x: last.x,
                    change: amount,
                    instruction,
                });
            }
        }
    }
    let last = states.last().unwrap();
    if last.change != 0 {
        states.push(State {
            cycle: last.cycle + 1,
            x: last.x + last.change,
            change: 0,
            instruction: Instruction::Noop(),
        });
    }
    states.remove(0);
    Ok(States { states })
}

pub fn render_states(states: &States) -> Result<(), Error> {
    for (pixel_index, state) in states.states.iter().enumerate() {
        let pixel_index = pixel_index as i64 % 40;
        if pixel_index == 0 {
            println!();
        }
        if state.x == pixel_index - 1 || state.x == pixel_index || state.x == pixel_index + 1 {
            print!("X");
        } else {
            print!(".");
        }
    }
    Ok(())
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
noop
addx 3
addx -5
    "#;

    let states = execute(input)?;
    assert_eq!(states.cycle(6).x, -1);
    let input = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
    let states = execute(input)?;
    assert_eq!(states.cycle(20).x, 21);
    assert_eq!(states.cycle(20).signal_strength(), 420);
    assert_eq!(states.cycle(60).signal_strength(), 1140);
    assert_eq!(states.cycle(100).signal_strength(), 1800);
    assert_eq!(states.cycle(140).signal_strength(), 2940);
    assert_eq!(states.cycle(180).signal_strength(), 2880);
    assert_eq!(states.cycle(220).signal_strength(), 3960);

    //render_states(&states)?;

    let states = execute(&std::fs::read_to_string("input/day10")?)?;
    assert_eq!(
        states.cycle(20).signal_strength()
            + states.cycle(60).signal_strength()
            + states.cycle(100).signal_strength()
            + states.cycle(140).signal_strength()
            + states.cycle(180).signal_strength()
            + states.cycle(220).signal_strength(),
        12560
    );

    //render_states(&states)?;

    Ok(())
}

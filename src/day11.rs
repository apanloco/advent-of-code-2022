// --- Day 11: Monkey in the Middle ---
// part1: What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans?
// part2: (mega big numbers) what is the level of monkey business after 10000 rounds?
use crate::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    pub monkeys: Vec<Monkey>,
    pub modulus: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Times(usize),
    Square,
    Plus(usize),
}

impl Operation {
    fn apply(&self, value: usize) -> usize {
        match self {
            Operation::Times(amount) => value * *amount,
            Operation::Square => value * value,
            Operation::Plus(amount) => value + *amount,
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<usize>,
    pub operation: Operation,
    pub divisible_by: usize,
    pub if_true: usize,
    pub if_false: usize,
    pub num_inspections: usize,
}

impl Game {
    pub fn monkey_business(&self) -> usize {
        let mut inspections: Vec<usize> = self.monkeys.iter().map(|m| m.num_inspections).collect();
        inspections.sort_by(|a, b| b.cmp(a));
        inspections[0] * inspections[1]
    }

    pub fn simulate_rounds(&mut self, num_rounds: usize, part2: bool) {
        let mut changes = Vec::new();
        for _round in 0..num_rounds {
            for monkey_index in 0..self.monkeys.len() {
                self.monkeys[monkey_index].num_inspections += self.monkeys[monkey_index].items.len();
                let items: Vec<usize> = self.monkeys[monkey_index].items.drain(..).collect();
                for mut item in items {
                    item = self.monkeys[monkey_index].operation.apply(item);
                    if !part2 {
                        item /= 3;
                    } else {
                        item %= self.modulus;
                    }
                    let target_monkey = if item % self.monkeys[monkey_index].divisible_by == 0 {
                        self.monkeys[monkey_index].if_true
                    } else {
                        self.monkeys[monkey_index].if_false
                    };
                    changes.push((target_monkey, item));
                }
                for (target_monkey, item) in changes.drain(..) {
                    self.monkeys[target_monkey].items.push(item);
                }
            }
        }
    }
}

impl FromStr for Game {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkeys = Vec::new();
        let mut lines = s.trim_start().trim_end().lines();
        loop {
            let (_line0, line1, line2, line3, line4, line5) =
                (lines.next(), lines.next(), lines.next(), lines.next(), lines.next(), lines.next());
            if line5.is_none() {
                break;
            }

            let items = line1
                .unwrap()
                .trim_start()
                .trim_start_matches("Starting items: ")
                .split(',')
                .map(|item| item.trim().parse::<usize>().unwrap())
                .collect();

            let line2 = line2.unwrap().trim_start().trim_start_matches("Operation: new = old ").to_string();
            let operation = if line2.starts_with("* old") {
                Operation::Square
            } else if line2.starts_with('*') {
                Operation::Times(line2.trim_start_matches("* ").parse()?)
            } else if line2.starts_with('+') {
                Operation::Plus(line2.trim_start_matches("+ ").parse()?)
            } else {
                panic!("invalid operation");
            };

            let divide_by = line3.unwrap().trim_start().trim_start_matches("Test: divisible by ").parse()?;

            let if_true = line4
                .unwrap()
                .trim_start()
                .trim_start_matches("If true: throw to monkey ")
                .parse()?;
            let if_false = line5
                .unwrap()
                .trim_start()
                .trim_start_matches("If false: throw to monkey ")
                .parse()?;

            monkeys.push(Monkey {
                items,
                operation,
                divisible_by: divide_by,
                if_true,
                if_false,
                num_inspections: 0,
            });

            let _ = lines.next();
        }
        let mut modulus: usize = 1;
        for divide_by in monkeys.iter().map(|m| m.divisible_by) {
            modulus *= divide_by;
        }
        Ok(Game { monkeys, modulus })
    }
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    let mut game = input.parse::<Game>()?;
    game.simulate_rounds(20, false);
    assert_eq!(game.monkey_business(), 10605);

    let mut game = input.parse::<Game>()?;
    game.simulate_rounds(20, true);
    assert_eq!(game.monkey_business(), 99 * 103);

    let mut game = input.parse::<Game>()?;
    game.simulate_rounds(1000, true);
    assert_eq!(game.monkey_business(), 5204 * 5192);

    let mut game = input.parse::<Game>()?;
    game.simulate_rounds(10000, true);
    assert_eq!(game.monkey_business(), 52166 * 52013);

    let mut game = std::fs::read_to_string("input/day11")?.parse::<Game>()?;
    game.simulate_rounds(20, false);
    assert_eq!(game.monkey_business(), 55458);

    let mut game = std::fs::read_to_string("input/day11")?.parse::<Game>()?;
    game.simulate_rounds(10_000, true);
    assert_eq!(game.monkey_business(), 14508081294);

    Ok(())
}

// --- Day 1: Calorie Counting ---
// part1: what elv has the most snacks
// part2: top 3 elves with the most snacks

use crate::error::Error;
use std::cmp::Ordering;
use std::str::FromStr;

pub struct Elves {
    pub elves: Vec<Elv>,
}

#[derive(Eq, Debug)]
pub struct Elv {
    pub snacks: Vec<usize>,
}

impl PartialEq<Self> for Elv {
    fn eq(&self, other: &Self) -> bool {
        self.snacks == other.snacks
    }
}

impl Ord for Elv {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sum().cmp(&other.sum())
    }
}

impl PartialOrd for Elv {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.sum().cmp(&other.sum()))
    }
}

impl Elv {
    fn new() -> Self {
        Self { snacks: vec![] }
    }

    pub fn sum(&self) -> usize {
        self.snacks.iter().sum()
    }

    fn add_snack(&mut self, snack: usize) {
        self.snacks.push(snack);
    }
}

impl FromStr for Elves {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elves = vec![Elv::new()];

        for line in s.trim_start().trim_end().lines() {
            let line = line.trim_start().trim_end();
            if line.is_empty() {
                elves.push(Elv::new());
            } else {
                elves.last_mut().unwrap().add_snack(line.parse()?);
            }
        }

        Ok(Elves { elves })
    }
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;
    let elves: Elves = input.parse()?;
    assert_eq!(elves.elves.len(), 5);
    assert_eq!(elves.elves.first().unwrap().sum(), 6000);
    assert_eq!(elves.elves.last().unwrap().sum(), 10000);
    let max_elv = elves.elves.iter().max();
    assert!(max_elv.is_some());
    let max_elv = max_elv.unwrap();
    assert_eq!(
        max_elv,
        &Elv {
            snacks: vec![7000, 8000, 9000],
        }
    );
    assert_eq!(max_elv.sum(), 24000);

    let input = std::fs::read_to_string("input/day1")?;
    let elves: Elves = input.parse()?;
    assert_eq!(elves.elves.iter().max().unwrap().sum(), 69883);

    let mut e = elves.elves;

    e.sort_by(|a, b| b.cmp(a));

    Ok(())
}

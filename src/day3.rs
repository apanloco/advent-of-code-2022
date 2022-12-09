// --- Day 3: Rucksack Reorganization ---
// part1: find common char in two strings (two halves of a string)
// part2: find common char in three strings

use crate::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub struct Rucksack {
    contents: String,
}

impl Rucksack {
    pub fn compartments(&self) -> (&str, &str) {
        let len = self.contents.len();
        (&self.contents[0..len / 2], &self.contents[len / 2..])
    }

    pub fn common_item(&self) -> Result<char, Error> {
        let (lhs, rhs) = self.compartments();
        for x in lhs.chars() {
            if rhs.contains(x) {
                return Ok(x);
            }
        }
        Err(Error::General("no common char found".to_string()))
    }

    pub fn score_of_common_item(&self) -> Result<usize, Error> {
        let item = self.common_item()?;
        let score = match item {
            'a'..='z' => {
                // Lowercase item types a through z have priorities 1 through 26.
                item as u32 - 96
            }
            'A'..='Z' => {
                // Uppercase item types A through Z have priorities 27 through 52.
                item as u32 - 38
            }
            _ => {
                return Err(Error::General("invalid char".to_string()));
            }
        };
        Ok(score as usize)
    }
}

fn score_for_char(c: char) -> Result<usize, Error> {
    match c {
        // Lowercase item types a through z have priorities 1 through 26.
        'a'..='z' => Ok((c as u32 - 96) as usize),
        // Uppercase item types A through Z have priorities 27 through 52.
        'A'..='Z' => Ok((c as u32 - 38) as usize),
        _ => Err(Error::General("invalid char".to_string())),
    }
}

impl FromStr for Rucksack {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            return Err(Error::General(format!("string {} not dividible by 2", s)));
        }
        Ok(Rucksack { contents: s.to_string() })
    }
}

pub fn group_score(group: &[Rucksack]) -> Result<usize, Error> {
    for c in group[0].contents.chars() {
        if group[1].contents.contains(c) && group[2].contents.contains(c) {
            return score_for_char(c);
        }
    }
    Err(Error::General("failed to find three equal chars in group".to_string()))
}

#[test]
fn test_empty_rucksack() {
    assert_eq!(&""[0..0], "");
    let r = Rucksack { contents: "".to_string() };
    let c = r.compartments();
    assert_eq!(c.0, "");
    assert_eq!(c.1, "");
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
    let file_contents = std::fs::read_to_string("input/day3")?;

    let rucksacks: Vec<Rucksack> = input.lines().map(|l| l.parse().unwrap()).collect();
    assert_eq!(rucksacks[0].score_of_common_item().unwrap(), 16);
    assert_eq!(rucksacks[1].score_of_common_item().unwrap(), 38);
    assert_eq!(rucksacks[2].score_of_common_item().unwrap(), 42);
    assert_eq!(rucksacks[3].score_of_common_item().unwrap(), 22);
    assert_eq!(rucksacks[4].score_of_common_item().unwrap(), 20);
    let score: usize = rucksacks.iter().map(|r| r.score_of_common_item().unwrap()).sum();
    assert_eq!(score, 157);
    let rucksacks: Vec<Rucksack> = file_contents.lines().map(|l| l.parse().unwrap()).collect();
    let score: usize = rucksacks.iter().map(|r| r.score_of_common_item().unwrap()).sum();
    assert_eq!(score, 8298);

    let rucksacks: Vec<Rucksack> = input.lines().map(|l| l.parse().unwrap()).collect();
    let groups: Vec<&[Rucksack]> = rucksacks.chunks(3).collect();
    assert_eq!(groups.len(), 2);
    assert_eq!(group_score(groups[0])?, 18);
    assert_eq!(group_score(groups[1])?, 52);

    let rucksacks: Vec<Rucksack> = file_contents.lines().map(|l| l.parse().unwrap()).collect();
    let groups: Vec<&[Rucksack]> = rucksacks.chunks(3).collect();
    let score: usize = groups.iter().map(|g| group_score(g).unwrap()).sum();
    assert_eq!(score, 2708);

    Ok(())
}

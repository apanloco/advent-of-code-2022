// --- Day 6: Tuning Trouble ---
// part1: find marker of len 4 in a string
// part2: find marker of len 14 in a string

use crate::error::Error;
use std::collections::HashSet;
use std::hash::Hash;

fn hashset_from_slice<R>(slice: &[R]) -> HashSet<R>
where
    R: Eq + Hash + Copy,
{
    let mut set = HashSet::with_capacity(slice.len());
    for c in slice {
        set.insert(*c);
    }
    set
}

pub fn find_marker(input: &str, marker_len: usize) -> Result<(String, usize), Error> {
    for (index, window) in input.as_bytes().windows(marker_len).enumerate() {
        if hashset_from_slice(window).len() == marker_len {
            return Ok((String::from_utf8_lossy(window).to_string(), index + marker_len));
        }
    }
    Err(Error::General("no marker found".to_string()))
}

#[test]
fn test() -> Result<(), Error> {
    assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)?, ("jpqm".to_string(), 7));
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4)?, ("vwbj".to_string(), 5));
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4)?.1, 6);
    let input = std::fs::read_to_string("input/day6")?;
    assert_eq!(find_marker(&input, 4)?, ("fwgm".to_string(), 1480));

    assert_eq!(
        find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)?,
        ("qmgbljsphdztnv".to_string(), 19)
    );
    let input = std::fs::read_to_string("input/day6")?;
    assert_eq!(find_marker(&input, 14)?, ("mwncpfhvqlsbtr".to_string(), 2746));

    Ok(())
}

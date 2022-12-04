use crate::error::Error;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Range {
    from: usize,
    to: usize,
}

impl Range {
    pub fn fully_contains(&self, other: Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    pub fn overlaps(&self, other: Range) -> bool {
        let outside_of = self.to < other.from || self.from > other.to;
        !outside_of
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Pair {
    lhs: Range,
    rhs: Range,
}

fn to_range(range: &str) -> Range {
    let from_to: Vec<usize> = range.split('-').map(|x| x.parse().unwrap()).collect();
    Range {
        from: from_to[0],
        to: from_to[1],
    }
}

fn to_pair(line: &str) -> Result<Pair, Error> {
    let mut range_iter = line.split(',').map(to_range);
    let error = || Error::General(format!("invalid pair: {}", line));
    Ok(Pair {
        lhs: range_iter.next().ok_or_else(error)?,
        rhs: range_iter.next().ok_or_else(error)?,
    })
}

pub fn to_range_pairs(input: &str) -> Result<Vec<Pair>, Error> {
    input.trim_start().trim_end().lines().map(to_pair).collect()
}

impl Pair {
    pub fn fully_contains(&self) -> bool {
        self.lhs.fully_contains(self.rhs) || self.rhs.fully_contains(self.lhs)
    }

    pub fn overlaps(&self) -> bool {
        self.lhs.overlaps(self.rhs)
    }
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
    let file_contents = std::fs::read_to_string("input/day4")?;

    let mut pairs = to_range_pairs(input)?;
    assert_eq!(
        pairs.first().unwrap(),
        &Pair {
            lhs: Range { from: 2, to: 4 },
            rhs: Range { from: 6, to: 8 },
        }
    );
    assert_eq!(
        pairs.last().unwrap(),
        &Pair {
            lhs: Range { from: 2, to: 6 },
            rhs: Range { from: 4, to: 8 },
        }
    );
    pairs.retain(|p| p.fully_contains());
    assert_eq!(pairs.len(), 2);

    let mut pairs = to_range_pairs(input)?;
    pairs.retain(|p| p.overlaps());
    assert_eq!(pairs.len(), 4);

    let mut pairs = to_range_pairs(&file_contents)?;
    pairs.retain(|p| p.fully_contains());
    assert_eq!(pairs.len(), 602);

    let mut pairs = to_range_pairs(&file_contents)?;
    pairs.retain(|p| p.overlaps());
    assert_eq!(pairs.len(), 891);

    Ok(())
}

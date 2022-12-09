// --- Day 5: Supply Stacks ---
// part1: move one crate from one stack to another
// part2: move a set of crates from one stack to another

use crate::error::Error;

pub struct Procedure {
    pub num: usize,
    pub from: usize,
    pub to: usize,
}

pub fn load(input: &str) -> Result<(Vec<Vec<char>>, Vec<Procedure>), Error> {
    let mut stacks = Vec::new();
    let mut procedures = Vec::new();
    for line in input.lines() {
        if line.contains('[') {
            let mut stack_index = 0;
            let mut chars = line.chars();
            while let Some(_start_bracket) = chars.next() {
                if let Some(crate_name) = chars.next() {
                    if crate_name != ' ' {
                        while stacks.len() <= stack_index {
                            stacks.push(Vec::new());
                        }
                        stacks[stack_index].push(crate_name);
                    }
                    stack_index += 1;
                }
                let _end_bracket = chars.next();
                let _separator = chars.next();
            }
        } else if line.starts_with("move") {
            use text_io::try_scan;
            let num;
            let from;
            let to;
            try_scan!(line.bytes() => "move {} from {} to {}", num, from, to);
            procedures.push(Procedure { num, from, to });
        }
    }
    for s in stacks.iter_mut() {
        s.reverse();
    }
    Ok((stacks, procedures))
}

pub fn apply_procedures_part1(mut input: Vec<Vec<char>>, procedures: Vec<Procedure>) -> Vec<Vec<char>> {
    for procedure in procedures {
        for _ in 0..procedure.num {
            let popped = input[procedure.from - 1].pop().unwrap();
            input[procedure.to - 1].push(popped);
        }
    }

    input
}

pub fn apply_procedures_part2(mut input: Vec<Vec<char>>, procedures: Vec<Procedure>) -> Vec<Vec<char>> {
    for procedure in procedures {
        let new_len = input[procedure.from - 1].len() - procedure.num;
        let mut moved = input[procedure.from - 1].split_off(new_len);
        input[procedure.to - 1].append(&mut moved);
    }

    input
}

pub fn message(input: &[Vec<char>]) -> String {
    let mut chars = Vec::new();
    for vec in input.iter() {
        if let Some(c) = vec.last() {
            chars.push(c);
        } else {
            chars.push(&' ');
        }
    }
    chars.into_iter().collect()
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    let (stacks, procedure) = load(input)?;
    assert_eq!(stacks.len(), 3);
    assert_eq!(stacks[0].len(), 2);
    assert_eq!(stacks[1].len(), 3);
    assert_eq!(stacks[2].len(), 1);
    assert_eq!(stacks[0], vec!['Z', 'N']);
    assert_eq!(stacks[1], vec!['M', 'C', 'D']);
    assert_eq!(stacks[2], vec!['P']);
    let transformed = apply_procedures_part1(stacks, procedure);
    assert_eq!(message(&transformed), "CMZ");

    let (stacks, procedure) = load(input)?;
    let transformed = apply_procedures_part2(stacks, procedure);
    assert_eq!(message(&transformed), "MCD");

    let (stacks, procedure) = load(&std::fs::read_to_string("input/day5")?)?;
    let transformed = apply_procedures_part1(stacks, procedure);
    assert_eq!(message(&transformed), "LBLVVTVLP");

    let (stacks, procedure) = load(&std::fs::read_to_string("input/day5")?)?;
    let transformed = apply_procedures_part2(stacks, procedure);
    assert_eq!(message(&transformed), "TPFFBDRJD");

    Ok(())
}

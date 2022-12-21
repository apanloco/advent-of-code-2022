// --- Day 21: Monkey Math ---
// part1: However, your actual situation involves considerably more monkeys. What number will the monkey named root yell?
// part2: (solve equation) What number do you yell to pass root's equality test?

use crate::error::Error;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Operation {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Num(i64),
    Human,
}

type Operations = HashMap<String, Operation>;

pub fn read_operations(input: &str) -> Result<Operations, Error> {
    let mut operations = HashMap::new();
    for line in input.trim().lines() {
        let line = line.trim();
        let key = &line[0..4];
        let line = &line[6..];
        let operation = if let Ok(number) = line.parse::<i64>() {
            Operation::Num(number)
        } else {
            use text_io::try_scan;
            let lhs: String;
            let operation: String;
            let rhs: String;
            try_scan!(line.bytes() => "{} {} {}", lhs, operation, rhs);
            let operation = match operation.as_ref() {
                "+" => Operation::Add(lhs, rhs),
                "-" => Operation::Sub(lhs, rhs),
                "*" => Operation::Mul(lhs, rhs),
                "/" => Operation::Div(lhs, rhs),
                _ => panic!("invalid operation: {}", operation),
            };
            operation
        };
        operations.insert(key.to_string(), operation);
    }
    Ok(operations)
}

fn calculate_rec(operations: &Operations, name: &str) -> i64 {
    match &operations[name] {
        Operation::Add(lhs, rhs) => calculate_rec(operations, lhs) + calculate_rec(operations, rhs),
        Operation::Sub(lhs, rhs) => calculate_rec(operations, lhs) - calculate_rec(operations, rhs),
        Operation::Mul(lhs, rhs) => calculate_rec(operations, lhs) * calculate_rec(operations, rhs),
        Operation::Div(lhs, rhs) => calculate_rec(operations, lhs) / calculate_rec(operations, rhs),
        Operation::Num(i) => *i,
        operation => {
            panic!("invalid operation: {:?}", operation)
        }
    }
}

pub fn calculate(operations: &Operations) -> i64 {
    calculate_rec(operations, "root")
}

pub fn print_operation(operations: &Operations, name: &str) -> String {
    match &operations[name] {
        Operation::Add(lhs, rhs) => format!("({} + {})", print_operation(operations, lhs), print_operation(operations, rhs)),
        Operation::Sub(lhs, rhs) => format!("({} - {})", print_operation(operations, lhs), print_operation(operations, rhs)),
        Operation::Mul(lhs, rhs) => format!("({} * {})", print_operation(operations, lhs), print_operation(operations, rhs)),
        Operation::Div(lhs, rhs) => format!("({} / {})", print_operation(operations, lhs), print_operation(operations, rhs)),
        Operation::Num(i) => format!("{}", *i),
        Operation::Human => "x".to_string(),
    }
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;
    let mut operations = read_operations(input)?;
    assert_eq!(calculate(&operations), 152);

    if let Operation::Add(lhs, rhs) = &operations["root"] {
        println!("{}+{}", print_operation(&operations, lhs), print_operation(&operations, rhs));
    }

    operations.insert("humn".to_string(), Operation::Human);
    if let Operation::Add(lhs, rhs) = &operations["root"] {
        println!("{}={}", print_operation(&operations, lhs), print_operation(&operations, rhs));
    }

    let mut operations = read_operations(&std::fs::read_to_string("input/day21")?)?;
    assert_eq!(calculate(&operations), 157714751182692);

    operations.insert("humn".to_string(), Operation::Human);
    if let Operation::Add(lhs, rhs) = &operations["root"] {
        println!("{}={}", print_operation(&operations, lhs), print_operation(&operations, rhs));
    }

    // x = 3373767893067
    // x = 3373767893067 (sympy)

    Ok(())
}

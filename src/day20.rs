// --- Day 20: Grove Positioning System ---
// part1: What is the sum of the three numbers that form the grove coordinates?
// part2: What is the sum of the three numbers that form the grove coordinates?

use crate::error::Error;
use std::cmp::Ordering;

pub fn read_numbers(input: &str) -> Result<Vec<i64>, Error> {
    let mut numbers = Vec::new();
    for line in input.trim().lines() {
        numbers.push(line.trim().parse()?);
    }
    Ok(numbers)
}

pub fn decrypt(encrypted: Vec<i64>, key: i64, times: usize) -> Vec<i64> {
    let encrypted: Vec<(usize, i64)> = encrypted.iter().enumerate().map(|(a, b)| (a, *b * key)).collect();
    let mut decrypted = encrypted.clone();
    for _ in 0..times {
        for (i, n) in encrypted.iter() {
            match n.cmp(&0) {
                Ordering::Less => decrypted = move_left(decrypted, *i),
                Ordering::Greater => decrypted = move_right(decrypted, *i),
                Ordering::Equal => {}
            }
        }
    }
    decrypted.iter().map(|(_, n)| *n).collect()
}

fn move_left(mut numbers: Vec<(usize, i64)>, o_index: usize) -> Vec<(usize, i64)> {
    let index = numbers.iter().position(|(i, _)| i == &o_index).unwrap() as i64;
    let number = numbers[index as usize].1;
    numbers.remove(index as usize);
    let mut new_index = (index - number.abs()) % numbers.len() as i64;
    if new_index <= 0 {
        new_index += numbers.len() as i64;
    }
    numbers.insert(new_index as usize, (o_index, number));
    numbers
}

fn move_right(mut numbers: Vec<(usize, i64)>, o_index: usize) -> Vec<(usize, i64)> {
    let index = numbers.iter().position(|(i, _)| i == &o_index).unwrap() as i64;
    let number = numbers[index as usize].1;
    numbers.remove(index as usize);
    let mut new_index = (index + number) as usize % numbers.len();
    if new_index == 0 {
        new_index = numbers.len();
    }
    numbers.insert(new_index as usize, (o_index, number));
    numbers
}

pub fn sum(numbers: &Vec<i64>) -> i64 {
    let mut index = numbers.iter().position(|&n| n == 0).unwrap() as i64;
    let mut sum = 0;
    for counter in 1..=3000 {
        index += 1;
        if index >= numbers.len() as i64 {
            index -= numbers.len() as i64;
        }
        if counter == 1000 || counter == 2000 || counter == 3000 {
            let value = numbers[index as usize];
            sum += value
        }
    }
    sum
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
    1
    2
    -3
    3
    -2
    0
    4"#;
    let encrypted = read_numbers(input)?;
    let decrypted = decrypt(encrypted, 1, 1);
    assert_eq!(decrypted, vec![1, 2, -3, 4, 0, 3, -2]);
    assert_eq!(sum(&decrypted), 3);

    let encrypted = read_numbers(input)?;
    let decrypted = decrypt(encrypted, 811589153, 10);
    assert_eq!(sum(&decrypted), 1623178306);

    let encrypted = read_numbers(&std::fs::read_to_string("input/day20")?)?;
    let decrypted = decrypt(encrypted, 1, 1);
    assert_eq!(sum(&decrypted), 8721);

    let encrypted = read_numbers(&std::fs::read_to_string("input/day20")?)?;
    let decrypted = decrypt(encrypted, 811589153, 10);
    assert_eq!(sum(&decrypted), 831878881825);

    Ok(())
}

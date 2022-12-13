// --- Day 13: Distress Signal ---
// part1: Determine which pairs of packets are already in the right order. What is the sum of the indices of those pairs?
// part2: Organize all of the packets into the correct order. What is the decoder key for the distress signal?

use crate::error::Error;
use serde_json::Value;
use std::cmp::Ordering;

type Packet = Vec<Value>;

pub struct Pair {
    pub lhs: Packet,
    pub rhs: Packet,
}

pub fn read_packets(input: &str) -> Result<Vec<Packet>, Error> {
    let mut packets = Vec::new();
    for line in input.trim_start().trim_end().lines() {
        if line.trim_start().trim_end().is_empty() {
            continue;
        }
        let packet: Packet = serde_json::from_str(line).unwrap();
        packets.push(packet);
    }
    Ok(packets)
}

pub fn split_into_pairs(packets: Vec<Packet>) -> Vec<Pair> {
    let mut pairs = Vec::with_capacity(packets.len() / 2);
    for chunk in packets.chunks(2) {
        pairs.push(Pair {
            lhs: chunk[0].clone(),
            rhs: chunk[1].clone(),
        })
    }
    pairs
}

fn is_end_of_array(array: &Vec<Value>, index: usize) -> bool {
    array.len() < index + 1
}

fn compare_recursively(lhs: &Vec<Value>, rhs: &Vec<Value>) -> std::cmp::Ordering {
    for i in 0..std::cmp::max(lhs.len(), rhs.len()) {
        if is_end_of_array(lhs, i) {
            return Ordering::Less;
        }
        if is_end_of_array(rhs, i) {
            return Ordering::Greater;
        }
        let a: &Value = &lhs[i];
        let b: &Value = &rhs[i];
        if a.is_number() && b.is_number() {
            let a = a.as_u64().unwrap();
            let b = b.as_u64().unwrap();
            match a.cmp(&b) {
                Ordering::Equal => {}
                Ordering::Less => {
                    return Ordering::Less;
                }
                Ordering::Greater => {
                    return Ordering::Greater;
                }
            }
        } else {
            let result = if a.is_number() && b.is_array() {
                compare_recursively(&vec![a.clone()], b.as_array().unwrap())
            } else if a.is_array() && b.is_number() {
                compare_recursively(a.as_array().unwrap(), &vec![b.clone()])
            } else {
                compare_recursively(a.as_array().unwrap(), b.as_array().unwrap())
            };
            match result {
                Ordering::Equal => {}
                Ordering::Less => {
                    return Ordering::Less;
                }
                Ordering::Greater => {
                    return Ordering::Greater;
                }
            }
        }
    }
    Ordering::Equal
}

pub fn is_in_order(pair: &Pair) -> bool {
    compare_recursively(&pair.lhs, &pair.rhs) == Ordering::Less
}

pub fn sum_of_in_order_indices(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_index, pair)| is_in_order(pair))
        .map(|(index, _pair)| index + 1)
        .sum()
}

pub fn create_divider_packets() -> Vec<Packet> {
    read_packets("[[2]]\n[[6]]").unwrap()
}

pub fn sort_packets(packets: &mut [Packet]) {
    packets.sort_by(compare_recursively);
}

pub fn decoder_key(packets: &[Packet]) -> usize {
    let divider_packets = create_divider_packets();
    packets
        .iter()
        .enumerate()
        .filter(|(_index, packet)| divider_packets.iter().any(|divider| packet == &divider))
        .map(|(index, _pair)| index + 1)
        .product()
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    let packets = read_packets(input)?;
    let pairs = split_into_pairs(packets);
    assert!(is_in_order(&pairs[0]));
    assert!(is_in_order(&pairs[1]));
    assert!(!is_in_order(&pairs[2]));
    assert!(is_in_order(&pairs[3]));
    assert!(!is_in_order(&pairs[4]));
    assert!(is_in_order(&pairs[5]));
    assert!(!is_in_order(&pairs[6]));
    assert!(!is_in_order(&pairs[7]));
    assert_eq!(sum_of_in_order_indices(&pairs), 13);
    let packets = read_packets(&std::fs::read_to_string("input/day13")?)?;
    let pairs = split_into_pairs(packets);
    assert_eq!(sum_of_in_order_indices(&pairs), 5605);

    let mut packets = read_packets(input)?;
    packets.append(&mut create_divider_packets());
    sort_packets(&mut packets);
    assert_eq!(decoder_key(&packets), 140);

    let mut packets = read_packets(&std::fs::read_to_string("input/day13")?)?;
    packets.append(&mut create_divider_packets());
    sort_packets(&mut packets);
    assert_eq!(decoder_key(&packets), 24969);

    Ok(())
}

// --- Day 16: Proboscidea Volcanium ---
// part1: Work out the steps to release the most pressure in 30 minutes. What is the most pressure you can release?
// part2: With you and an elephant working together for 26 minutes, what is the most pressure you could release?

use crate::day16::{load_valves, Valve};
use crate::error::Error;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action<'a> {
    Start,
    Open(&'a str, i64),
    MoveTo(&'a str, i64),
}

fn find_possible_actions<'a, 'b>(
    valves: &'a HashMap<String, Valve>,
    minute: i64,
    opened: &'b [&'a str],
    last_action: &Action<'a>,
) -> Vec<Action<'a>> {
    let current_value: &str = match last_action {
        Action::Start => "AA",
        Action::Open(valve, _) => valve,
        Action::MoveTo(valve, _) => valve,
    };
    let mut possible_actions = Vec::with_capacity(6);
    let valve_details = &valves[current_value];
    if !opened.contains(&current_value) {
        let valve_details = &valves[current_value];
        if valve_details.flow_rate > 0 {
            let increase = valve_details.flow_rate * (26 - minute);
            possible_actions.push(Action::Open(current_value, increase));
        }
    }
    for path in &valve_details.paths {
        possible_actions.push(Action::MoveTo(&path.name, path.cost));
    }
    possible_actions
}

struct End {
    max_open: usize,
    max_minutes: i64,
}

fn recursively_find_max_pressure<'a, 'b>(
    total_actions: &'b mut Vec<(Action<'a>, Action<'a>)>,
    valves: &'a HashMap<String, Valve>,
    opened: &'b mut Vec<&'a str>,
    minute_me: i64,
    minute_elephant: i64,
    end: &End,
    mut total_pressure_release: i64,
    max_total_pressure_release: &mut i64,
    total_times_called: &mut usize,
) -> Option<i64> {
    // println!();
    // println!("total actions: {:?}", &total_actions);

    // if minute_me > end.max_minutes || minute_elephant > end.max_minutes {
    //     panic!("poop");
    // }

    *total_times_called += 1;

    let last_action = total_actions.last().unwrap();

    // println!("last action: {:?}", last_action);
    //
    let mut num_opened = 0;

    if let Action::Open(valve, increase) = &last_action.0 {
        if opened.contains(valve) {
            panic!("internal error");
        }
        total_pressure_release += increase;
        opened.push(valve);
        num_opened += 1;
    }

    if let Action::Open(valve, increase) = &last_action.1 {
        if opened.contains(valve) {
            panic!("internal error");
        }
        total_pressure_release += increase;
        opened.push(valve);
        num_opened += 1;
    }

    let mut results = Vec::with_capacity(6 * 6);

    if *max_total_pressure_release < total_pressure_release {
        println!("max: {}, actions:", max_total_pressure_release);
        for (i, action) in total_actions.iter().enumerate() {
            println!("{}:{:?}", i, action);
        }
        println!(
            "max_open: {}, opened: {}, left: {}",
            end.max_open,
            opened.len(),
            end.max_open - opened.len()
        );

        *max_total_pressure_release = total_pressure_release;
        results.push(Some(total_pressure_release));
    }

    if opened.len() < end.max_open && minute_me < end.max_minutes && minute_elephant < end.max_minutes {
        let my_possible_actions = find_possible_actions(valves, minute_me, opened, &last_action.0);
        let elephant_possible_actions = find_possible_actions(valves, minute_elephant, opened, &last_action.1);
        let mut possible_actions: Vec<(Action, Action)> = Vec::with_capacity(6 * 6);

        // println!("my_possible_actions: {:?}", &my_possible_actions);
        // println!("elephant_possible_actions: {:?}", &elephant_possible_actions);
        // //
        // println!("");

        for my_action in &my_possible_actions {
            for elephant_action in &elephant_possible_actions {
                if let Action::Open(lhs, _) = my_action {
                    if let Action::Open(rhs, _) = elephant_action {
                        if lhs == rhs {
                            continue;
                        }
                    }
                }

                if !possible_actions.contains(&(elephant_action.clone(), my_action.clone())) {
                    let new_action = (my_action.clone(), elephant_action.clone());
                    //println!("adding new_action: {:?}", &new_action);
                    possible_actions.push(new_action);
                }
            }
        }

        possible_actions.shuffle(&mut thread_rng());

        // possible_actions.sort_by(|a, b| {
        //     if let Action::Open(_, _) = a.0 {
        //         return std::cmp::Ordering::Less;
        //     }
        //     if let Action::Open(_, _) = a.1 {
        //         return std::cmp::Ordering::Less;
        //     }
        //     std::cmp::Ordering::Equal
        // });
        // println!("possible_actions: {:?}", &possible_actions);

        for action_pair in possible_actions {
            if idiot_move(total_actions, &action_pair) {
                //println!();
                continue;
            }

            let minute_increase_me = get_action_cost(&action_pair.0);

            if minute_me + minute_increase_me > end.max_minutes {
                continue;
            }

            let minute_increase_elephant = get_action_cost(&action_pair.1);

            if minute_elephant + minute_increase_elephant > end.max_minutes {
                continue;
            }

            total_actions.push(action_pair);
            let result = recursively_find_max_pressure(
                total_actions,
                valves,
                opened,
                minute_me + minute_increase_me,
                minute_elephant + minute_increase_elephant,
                end,
                total_pressure_release,
                max_total_pressure_release,
                total_times_called,
            );
            total_actions.pop();
            results.push(result);
        }
    }

    if num_opened > 0 {
        opened.truncate(opened.len() - num_opened);
    }

    let mut results: Vec<i64> = results.into_iter().flatten().collect();

    if results.is_empty() {
        return None;
    }

    results.sort_by(|a, b| b.cmp(a));

    let best = &results[0];

    Some(*best)
}

fn get_action_cost(action: &Action) -> i64 {
    match action {
        Action::Start => {
            panic!("internal error");
        }
        Action::Open(_, _) => 1,
        Action::MoveTo(_, cost) => *cost,
    }
}

pub fn max_pressure(valves: HashMap<String, Valve>, max_minutes: i64) -> Result<Option<i64>, Error> {
    let mut max_total_pressure_release = 0;
    let mut total_times_called = 0;
    let result = recursively_find_max_pressure(
        &mut vec![(Action::Start, Action::Start)],
        &valves,
        &mut Vec::new(),
        1,
        1,
        &End {
            max_open: valves.values().filter(|v| v.flow_rate > 0).count(),
            max_minutes,
        },
        0,
        &mut max_total_pressure_release,
        &mut total_times_called,
    );
    println!("result: {:?}, total_times_called: {}", result, total_times_called);
    Ok(result)
}

pub fn idiot_move(total_actions: &[(Action, Action)], action: &(Action, Action)) -> bool {
    for check in total_actions.iter().rev() {
        match &check.0 {
            Action::Open(_, _) => {
                break;
            }
            Action::MoveTo(valve1, _) => {
                if let Action::MoveTo(valve2, _) = &action.0 {
                    if valve1 == valve2 {
                        //println!("idiot_move 0: {:?}, {:?}", action, total_actions);
                        return true;
                    }
                }
            }
            _ => {}
        }
    }

    for check in total_actions.iter().rev() {
        match &check.1 {
            Action::Open(_, _) => {
                break;
            }
            Action::MoveTo(valve1, _) => {
                if let Action::MoveTo(valve2, _) = &action.1 {
                    if valve1 == valve2 {
                        //println!("idiot_move 1: {:?}, {:?}", action, total_actions);
                        return true;
                    }
                }
            }
            _ => {}
        }
    }

    false
}
// #[test]
// fn test_idiot_move() -> Result<(), Error> {
//     let actions = vec![
//         (Action::Start, Action::Start),
//         (Action::MoveTo("DD".to_string()), Action::MoveTo("AA".to_string())),
//         (Action::MoveTo("EE".to_string()), Action::Open("AA".to_string(), 0)),
//         (Action::MoveTo("CC".to_string()), Action::MoveTo("DD".to_string())),
//     ];
//
//     assert!(idiot_move(
//         &actions,
//         &(Action::MoveTo("DD".to_string()), Action::Open("DD".to_string(), 0))
//     ),);
//
//     assert!(idiot_move(
//         &actions,
//         &(Action::Open("DD".to_string(), 0), Action::MoveTo("DD".to_string()))
//     ),);
//
//     assert!(!idiot_move(
//         &actions,
//         &(Action::MoveTo("FF".to_string()), Action::Open("DD".to_string(), 0))
//     ),);
//
//     assert!(!idiot_move(
//         &actions,
//         &(Action::MoveTo("FF".to_string()), Action::MoveTo("AA".to_string()))
//     ),);
//
//     Ok(())
// }
//
// fn prosperity(valves: &HashMap<String, Valve>, opened: &mut Vec<&str>, from: &str, action: &Action, minutes: i64, max_minutes: i64) -> i64 {
//     match action {
//         Action::Start => 0,
//         Action::Open(_, increase) => *increase,
//         Action::MoveTo(valve, _) => recursively_find_prosperity(valves, opened, &mut vec![from], valve, minutes, max_minutes),
//     }
// }
//
// fn recursively_find_prosperity(
//     valves: &HashMap<String, Valve>,
//     opened: &mut Vec<&str>,
//     visited: &mut Vec<&str>,
//     valve: &str,
//     minutes: i64,
//     max_minutes: i64,
// ) -> i64 {
//     let mut costs: Vec<i64> = Vec::new();
//     for path in &valves[valve].paths {
//         if visited.contains(&path.name) {
//             continue;
//         }
//         visited.push(&path.name);
//         recursively_find_prosperity(valves, opened, visited, &path.name, minutes + path.cost)
//     }
//     if costs.is_empty() {
//         0
//     } else {
//         costs.sort_by(|a, b| b.cmp(a));
//         costs[0]
//     }
// }
//
// #[test]
// fn test_prosperity() -> Result<(), Error> {
//     let input = r#"
//     Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
//     Valve BB has flow rate=13; tunnels lead to valves CC, AA
//     Valve CC has flow rate=2; tunnels lead to valves DD, BB
//     Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
//     Valve EE has flow rate=3; tunnels lead to valves FF, DD
//     Valve FF has flow rate=0; tunnels lead to valves EE, GG
//     Valve GG has flow rate=0; tunnels lead to valves FF, HH
//     Valve HH has flow rate=22; tunnel leads to valve GG
//     Valve II has flow rate=0; tunnels lead to valves AA, JJ
//     Valve JJ has flow rate=21; tunnel leads to valve II"#;
//     let valves = load_valves(input, false)?;
//     let p = prosperity(&valves, &mut Vec::new(), "AA", &Action::MoveTo("BB", 1), 1, 26);
//     Ok(())
// }

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
    Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II"#;

    /*
    Valve JJ has flow rate=21; tunnels lead to valves DD(3) BB(3)
    Valve AA has flow rate=0; tunnels lead to valves DD(1) JJ(2) BB(1)
    Valve DD has flow rate=20; tunnels lead to valves CC(1) JJ(3) BB(2) EE(1)
    Valve EE has flow rate=3; tunnels lead to valves HH(3) DD(1)
    Valve BB has flow rate=13; tunnels lead to valves CC(1) DD(2) JJ(3)
    Valve HH has flow rate=22; tunnels lead to valves EE(3)
    Valve CC has flow rate=2; tunnels lead to valves DD(1) BB(1)
         */

    // let valves = load_valves(input, true)?;
    // assert!(max_pressure(valves, 11)?.is_some());

    // 8: 22641 vs 5271
    // 9: 58285 vs 12945
    // 10: 132794 vs 31307
    // 11
    // 12*
    // 13*
    // 14*
    // 15*: 2698102 vs 1561356
    // 16*: 4096860 vs 2945693
    // 17: 5732898 vs 5475983
    // 18: 7414279 vs 8946509
    // let valves = load_valves(input, false)?;
    // assert_eq!(max_pressure(valves, 26)?, Some(1707));

    // 2125 too low (just tried, too early)
    // 2211 too low (too early...)
    // 2253 too low (too early...)
    // 2268 (if you're stuck...)
    // 2368 (if you're stuck...)
    // 2581 (if you're stuck...)
    // 2639 (if you're stuck...)
    // 2672 (if you're stuck...)
    let valves = load_valves(&std::fs::read_to_string("input/day16")?, true)?;
    assert_eq!(max_pressure(valves, 24)?, Some(666));

    Ok(())
}

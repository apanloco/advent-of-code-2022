// --- Day 16: Proboscidea Volcanium ---
// part1: Work out the steps to release the most pressure in 30 minutes. What is the most pressure you can release?
// part2: With you and an elephant working together for 26 minutes, what is the most pressure you could release?

use crate::day16::Valve;
use crate::error::Error;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        possible_actions.push(Action::MoveTo(&path.name, 1));
    }
    possible_actions
}

struct End {
    max_open: usize,
    max_minutes: i64,
}

#[derive(Hash, Eq, PartialEq)]
struct State<'a> {
    me_last_state: Action<'a>,
    elephant_last_state: Action<'a>,
    minute_me: i64,
    minute_elephant: i64,
}

fn recursively_find_max_pressure<'a, 'b>(
    total_actions: &'b mut Vec<(Action<'a>, Action<'a>)>,
    valves: &'a HashMap<String, Valve>,
    visited: &mut HashMap<State<'a>, i64>,
    opened: &'b mut Vec<&'a str>,
    minute_me: i64,
    minute_elephant: i64,
    end: &End,
    mut total_pressure_release: i64,
) -> Option<i64> {
    let last_action = total_actions.last().unwrap();

    if let Action::Open(_, increase) = &last_action.0 {
        total_pressure_release += increase;
    }
    if let Action::Open(_, increase) = &last_action.1 {
        total_pressure_release += increase;
    }

    let entry = visited
        .entry(State {
            me_last_state: last_action.0.clone(),
            elephant_last_state: last_action.1.clone(),
            minute_me,
            minute_elephant,
        })
        .or_insert(total_pressure_release);
    if *entry > total_pressure_release {
        return None;
    } else {
        *entry = total_pressure_release;
    }

    let mut num_opened = 0;
    if let Action::Open(valve, _) = &last_action.0 {
        opened.push(valve);
        num_opened += 1;
    }
    if let Action::Open(valve, _) = &last_action.1 {
        opened.push(valve);
        num_opened += 1;
    }

    let mut results = Vec::with_capacity(6 * 6);

    results.push(Some(total_pressure_release));

    if opened.len() < end.max_open && minute_me < end.max_minutes && minute_elephant < end.max_minutes {
        let my_possible_actions = find_possible_actions(valves, minute_me, opened, &last_action.0);
        let elephant_possible_actions = find_possible_actions(valves, minute_elephant, opened, &last_action.1);
        let mut possible_actions: Vec<(Action, Action)> = Vec::with_capacity(6 * 6);

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

        for action_pair in possible_actions {
            if invalid_move(total_actions, &action_pair) {
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
                visited,
                opened,
                minute_me + minute_increase_me,
                minute_elephant + minute_increase_elephant,
                end,
                total_pressure_release,
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
    let result = recursively_find_max_pressure(
        &mut vec![(Action::Start, Action::Start)],
        &valves,
        &mut HashMap::new(),
        &mut Vec::new(),
        1,
        1,
        &End {
            max_open: valves.values().filter(|v| v.flow_rate > 0).count(),
            max_minutes,
        },
        0,
    );
    Ok(result)
}

pub fn invalid_move(total_actions: &[(Action, Action)], action: &(Action, Action)) -> bool {
    for check in total_actions.iter().rev() {
        match &check.0 {
            Action::Open(_, _) => {
                break;
            }
            Action::MoveTo(valve1, _) => {
                if let Action::MoveTo(valve2, _) = &action.0 {
                    if valve1 == valve2 {
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

    let valves = crate::day16::load_valves(input)?;
    assert_eq!(max_pressure(valves, 26)?, Some(1707));

    let valves = crate::day16::load_valves(&std::fs::read_to_string("input/day16")?)?;
    assert_eq!(max_pressure(valves, 26)?, Some(2999));

    Ok(())
}

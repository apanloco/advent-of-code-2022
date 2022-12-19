// --- Day 16: Proboscidea Volcanium ---
// part1: Work out the steps to release the most pressure in 30 minutes. What is the most pressure you can release?
// part2: With you and an elephant working together for 26 minutes, what is the most pressure you could release?

use crate::error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Path {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Valve {
    pub name: String,
    pub flow_rate: i64,
    pub paths: Vec<Path>,
}

pub fn load_valves(input: &str) -> Result<HashMap<String, Valve>, Error> {
    let mut valves = HashMap::new();
    for line in input.trim().lines() {
        let mut split = line.trim().split(';');
        let (lhs, rhs) = (split.next().unwrap(), split.next().unwrap());
        use text_io::try_scan;
        let name: String;
        let flow_rate: i64;
        try_scan!(lhs.bytes() => "Valve {} has flow rate={}", name, flow_rate);
        let paths: Vec<Path> = rhs
            .trim_start_matches(" tunnel leads to valve ")
            .trim_start_matches(" tunnels lead to valves ")
            .split(',')
            .map(|p| Path {
                name: p.trim().to_string(),
            })
            .collect();
        valves.insert(name.to_string(), Valve { name, flow_rate, paths });
    }
    Ok(valves)
}

fn recursively_find_max_pressure(
    visited: Vec<String>,
    total_actions: String,
    valves: &HashMap<String, Valve>,
    opened: Vec<String>,
    minute: i64,
    max_minutes: i64,
    total_pressure_release: i64,
    max_total_pressure_release: &mut i64,
) -> Option<(i64, Vec<String>, String)> {
    if minute > max_minutes {
        return None;
    }

    if *max_total_pressure_release < total_pressure_release {
        *max_total_pressure_release = total_pressure_release;
        println!("max: {}, actions: {}", max_total_pressure_release, total_actions);
    }

    if minute == max_minutes {
        return Some((total_pressure_release, visited, total_actions));
    }

    if opened.len() == valves.len() - 1 {
        // println!("bajs detected (all valves opened, no need to run around)");
        return None;
    }

    if visited.len() > 4
        && visited[visited.len() - 1] == visited[visited.len() - 3]
        && visited[visited.len() - 2] == visited[visited.len() - 4]
    {
        // println!("bajs detected (loop");
        return None;
    }

    let current_valve_name = visited.last().unwrap();

    // println!("{}", &total_actions);

    let mut results = Vec::new();
    results.push(Some((total_pressure_release, visited.clone(), total_actions.to_string())));

    let valve_details = &valves[current_valve_name];

    for path in &valve_details.paths {
        if minute > max_minutes {
            continue;
        }
        {
            // open (if possible or necessary)
            if minute < max_minutes && valve_details.flow_rate > 0 && !opened.contains(current_valve_name) {
                let total_actions = format!("{}|{}-open({})", total_actions, minute, &current_valve_name,);
                let total_pressure_release = total_pressure_release + valve_details.flow_rate * (30 - minute);
                results.push(Some((total_pressure_release, visited.clone(), total_actions.to_string())));
                let minute = minute + 1;
                if minute < max_minutes {
                    let mut opened = opened.clone();
                    opened.push(current_valve_name.to_string());
                    let mut visited = visited.clone();
                    visited.push(path.name.to_string());
                    let result = recursively_find_max_pressure(
                        visited,
                        format!("{}|{}-move({})", &total_actions, minute, &path.name),
                        valves,
                        opened,
                        minute + 1,
                        max_minutes,
                        total_pressure_release,
                        max_total_pressure_release,
                    );
                    results.push(result);
                }
            }
        }
        {
            if visited.len() > 2 && visited[visited.len() - 2] == path.name {
                // println!("bajs detected (going back with no opening)");
                continue;
            }
            // don't open
            let mut visited = visited.clone();
            visited.push(path.name.to_string());
            let result = recursively_find_max_pressure(
                visited,
                format!("{}|{}-move({})", &total_actions, minute, &path.name),
                valves,
                opened.clone(),
                minute + 1,
                max_minutes,
                total_pressure_release,
                max_total_pressure_release,
            );
            results.push(result);
        }
    }

    let mut results: Vec<(i64, Vec<String>, String)> = results.into_iter().flatten().collect();

    if results.is_empty() {
        return None;
    }

    results.sort_by(|a, b| b.0.cmp(&a.0));

    let best = &results[0];

    Some((best.0, best.1.clone(), best.2.clone()))
}

pub fn max_pressure(valves: HashMap<String, Valve>, minutes: i64) -> Result<Option<i64>, Error> {
    let mut max_total_pressure_release = 0;
    let result = recursively_find_max_pressure(
        vec!["AA".to_string()],
        "0-start|".to_string(),
        &valves,
        Vec::new(),
        1,
        minutes,
        0,
        &mut max_total_pressure_release,
    );
    println!("result: {:?}", result);
    Ok(result.map(|x| x.0))
}

// #[test]
// fn test() -> Result<(), Error> {
//     let input = r#"
// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
// Valve BB has flow rate=13; tunnels lead to valves CC, AA
// Valve CC has flow rate=2; tunnels lead to valves DD, BB
// Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
// Valve EE has flow rate=3; tunnels lead to valves FF, DD
// Valve FF has flow rate=0; tunnels lead to valves EE, GG
// Valve GG has flow rate=0; tunnels lead to valves FF, HH
// Valve HH has flow rate=22; tunnel leads to valve GG
// Valve II has flow rate=0; tunnels lead to valves AA, JJ
// Valve JJ has flow rate=21; tunnel leads to valve II"#;
//     let valves = load_valves(input)?;
//     assert_eq!(max_pressure(valves, 30)?, Some(1651));
//
//     // let valves = load_valves(&std::fs::read_to_string("input/day16")?)?;
//     // assert_eq!(max_pressure(valves, 30)?, Some(2359)); // 74 minuter, no optimizations
//
//     Ok(())
// }

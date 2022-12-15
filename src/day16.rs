use crate::error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Path {
    pub name: String,
    pub cost: i64,
}

#[derive(Debug, Clone)]
pub struct Valve {
    pub name: String,
    pub flow_rate: i64,
    pub paths: Vec<Path>,
}

impl Valve {
    pub fn total_presure_released(&self, minute: i64) -> i64 {
        self.flow_rate * (30 - minute)
    }
}

fn dump_valves(valves: &HashMap<String, Valve>) {
    for (k, v) in valves {
        if k != &v.name {
            panic!("internal error!");
        }
        print!("Valve {} has flow rate={}; tunnels lead to valves ", k, v.flow_rate);
        for p in &v.paths {
            print!("{}({}) ", p.name, p.cost);
        }
        println!();
    }
    println!();
}

// fn optimize(mut valves: HashMap<String, Valve>) -> HashMap<String, Valve> {
//     dump_valves(&valves);
//     let mut optimized: HashMap<String, Valve> = HashMap::new();
//     for (k, v1) in valves.iter() {
//         loop {
//             let mut changes = false;
//             println!("optimizing {} {:?}", k, v1);
//             let mut new_paths = Vec::new();
//             for x in v1.paths.iter() {
//                 let v2 = &valves[&x.name];
//                 if v2.flow_rate == 0 {
//                     changes = true;
//                     for p in &v2.paths {
//                         if &p.name != k {
//                             new_paths.push(Path {
//                                 name: p.name.to_string(),
//                                 cost: p.cost + 1,
//                             })
//                         }
//                     }
//                 } else {
//                     new_paths.push(x.clone());
//                 }
//                 //println!("new paths: {:?}", &new_paths);
//             }
//
//             if !changes {
//                 optimized.insert(
//                     k.to_string(),
//                     Valve {
//                         name: k.to_string(),
//                         flow_rate: v1.flow_rate,
//                         paths: new_paths,
//                     },
//                 );
//                 break;
//             }
//         }
//     }
//
//     optimized
// }

pub fn load_valves(input: &str) -> Result<HashMap<String, Valve>, Error> {
    let mut valves = HashMap::new();
    for line in input.trim().lines() {
        let mut split = line.split(';');
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
                cost: 1,
            })
            .collect();
        valves.insert(name.to_string(), Valve { name, flow_rate, paths });
    }
    //let valves = optimize(valves);
    Ok(valves)
}

fn recursively_do_something(
    visited: Vec<String>,
    total_actions: String,
    valves: &HashMap<String, Valve>,
    opened: Vec<String>,
    minute: i64,
    max_minutes: i64,
    total_pressure_release: i64,
    max_total_pressure_release: &mut i64,
) -> Option<(i64, Vec<String>, String)> {
    if *max_total_pressure_release < total_pressure_release {
        *max_total_pressure_release = total_pressure_release;
        println!("max: {}, actions: {}", max_total_pressure_release, total_actions);
    }
    // if total_actions.starts_with(
    //     "|1-move(DD)|2-open(DD)|3-move(CC)|4-move(BB)|5-open(BB)|6-move(AA)|7-move(II)|8-move(JJ)|9-open(JJ)|10-move(II)|11-move(AA)",
    // ) {
    //     println!("YES!!");
    // }

    if opened.len() == 15 {
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

    if minute == max_minutes {
        return Some((total_pressure_release, visited.clone(), total_actions));
    }

    if minute > max_minutes {
        return None;
    }

    let mut results = Vec::new();
    results.push(Some((total_pressure_release, visited.clone(), total_actions.to_string())));

    let valve_details = &valves[current_valve_name];

    for path in &valve_details.paths {
        // println!("from {} trying: {:?}", &current_valve_name, &path);
        // let loop_detection = current_valve_name.to_string() + ">" + &path.name;
        // if total_path.contains(&loop_detection) {
        //     // println!("loop detected, skipping");
        //     continue;
        // }
        {
            // open (if possible or necessary)
            if minute < max_minutes && valve_details.flow_rate > 0 && !opened.contains(&current_valve_name) {
                let total_actions = format!("{}|{}-open({})", total_actions, minute, &current_valve_name,);
                let total_pressure_release = total_pressure_release + valve_details.total_presure_released(minute);
                results.push(Some((total_pressure_release, visited.clone(), total_actions.to_string())));
                let minute = minute + 1;
                if minute < max_minutes {
                    let mut opened = opened.clone();
                    opened.push(current_valve_name.to_string());
                    let mut visited = visited.clone();
                    visited.push(path.name.to_string());
                    let result = recursively_do_something(
                        visited,
                        format!("{}|{}-move({})", &total_actions, minute, &path.name),
                        valves,
                        opened,
                        minute + path.cost,
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
            let result = recursively_do_something(
                visited,
                format!("{}|{}-move({})", &total_actions, minute, &path.name),
                valves,
                opened.clone(),
                minute + path.cost,
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
    let result = recursively_do_something(
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

// |1-move(DD)|2-open(DD)|3-move(CC)|4-move(BB)|5-open(BB)|6-move(AA)|7-move(II)|8-move(JJ)|9-open(JJ)|10-move(II)|11-move(AA)|12-move(DD)

#[test]
fn test() -> Result<(), Error> {
    // brute: 5,066549581e+20
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
    let valves = load_valves(input)?;
    assert_eq!(max_pressure(valves, 30)?, Some(1651));

    // let valves = load_valves(&std::fs::read_to_string("input/day16")?)?;
    // assert_eq!(max_pressure(valves, 30)?, Some(2359)); // 74 minuter, no optimizations

    Ok(())
}

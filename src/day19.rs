// --- Day 19: Not Enough Minerals ---
// part1: What do you get if you add up the quality level of all of the blueprints in your list?
// part2: (more iterations) What do you get if you multiply these numbers together?

use crate::error::Error;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use text_io::try_scan;

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub id: usize,
    pub cost_ore_robot_ore: usize,
    pub cost_clay_robot_ore: usize,
    pub cost_obsidian_robot_ore: usize,
    pub cost_obsidian_robot_clay: usize,
    pub cost_geode_robot_ore: usize,
    pub cost_geode_robot_obsidian: usize,
}

pub fn load_blueprint(s: &str) -> Result<Blueprint, Error> {
    let id: usize;
    let cost_ore_robot_ore: usize;
    let cost_clay_robot_ore: usize;
    let cost_obsidian_robot_ore: usize;
    let cost_obsidian_robot_clay: usize;
    let cost_geode_robot_ore: usize;
    let cost_geode_robot_obsidian: usize;

    try_scan!(s.trim().bytes() => "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
        id,
    cost_ore_robot_ore,
    cost_clay_robot_ore,
    cost_obsidian_robot_ore,
    cost_obsidian_robot_clay,
    cost_geode_robot_ore,
    cost_geode_robot_obsidian);

    Ok(Blueprint {
        id,
        cost_ore_robot_ore,
        cost_clay_robot_ore,
        cost_obsidian_robot_ore,
        cost_obsidian_robot_clay,
        cost_geode_robot_ore,
        cost_geode_robot_obsidian,
    })
}

#[derive(Debug, Clone)]
pub struct State {
    pub num_ore_robots: usize,
    pub num_clay_robots: usize,
    pub num_obsidian_robots: usize,
    pub num_geode_robots: usize,

    pub num_ore: usize,
    pub num_clay: usize,
    pub num_obsidian: usize,
    pub num_geode: usize,

    pub can_buy_ore_robot: bool,
    pub can_buy_clay_robot: bool,
    pub can_buy_obsidian_robot: bool,
    pub can_buy_geode_robot: bool,
    pub bought_ore_robot: bool,
    pub bought_clay_robot: bool,
    pub bought_obsidian_robot: bool,
    pub bought_geode_robot: bool,

    pub minute: usize,
}

impl State {
    pub fn bought_nothing(&self) -> bool {
        !self.bought_ore_robot && !self.bought_clay_robot && !self.bought_obsidian_robot && !self.bought_geode_robot
    }
}

pub fn simulate(blueprint: &Blueprint, end_time: usize) -> usize {
    let start_state = State {
        num_ore_robots: 1,
        num_clay_robots: 0,
        num_obsidian_robots: 0,
        num_geode_robots: 0,
        num_ore: 0,
        num_clay: 0,
        num_obsidian: 0,
        num_geode: 0,
        can_buy_ore_robot: false,
        can_buy_clay_robot: false,
        can_buy_obsidian_robot: false,
        can_buy_geode_robot: false,
        bought_ore_robot: false,
        bought_clay_robot: false,
        bought_obsidian_robot: false,
        bought_geode_robot: false,
        minute: 1,
    };

    let mut states = vec![start_state.clone()];
    let mut best_state = start_state;

    loop {
        if states.is_empty() {
            break;
        }

        let current_state = states.pop().unwrap();

        if current_state.minute == end_time {
            if best_state.num_geode < current_state.num_geode {
                best_state = current_state;
                println!(
                    "bp: {}, (states: {}, capacity: {}) best: {:?}",
                    blueprint.id,
                    states.len(),
                    states.capacity(),
                    best_state.num_geode
                );
            }
            continue;
        }

        let can_buy_ore_robot = current_state.num_ore >= blueprint.cost_ore_robot_ore;
        let can_buy_clay_robot = current_state.num_ore >= blueprint.cost_clay_robot_ore;
        let can_buy_obsidian_robot =
            current_state.num_ore >= blueprint.cost_obsidian_robot_ore && current_state.num_clay >= blueprint.cost_obsidian_robot_clay;
        let can_buy_geode_robot =
            current_state.num_ore >= blueprint.cost_geode_robot_ore && current_state.num_obsidian >= blueprint.cost_geode_robot_obsidian;

        if can_buy_ore_robot && !(current_state.can_buy_ore_robot && current_state.bought_nothing()) {
            states.push(State {
                num_ore_robots: current_state.num_ore_robots + 1,
                num_clay_robots: current_state.num_clay_robots,
                num_obsidian_robots: current_state.num_obsidian_robots,
                num_geode_robots: current_state.num_geode_robots,
                num_ore: current_state.num_ore + current_state.num_ore_robots - blueprint.cost_ore_robot_ore,
                num_clay: current_state.num_clay + current_state.num_clay_robots,
                num_obsidian: current_state.num_obsidian + current_state.num_obsidian_robots,
                num_geode: current_state.num_geode + current_state.num_geode_robots,
                can_buy_ore_robot,
                can_buy_clay_robot,
                can_buy_obsidian_robot,
                can_buy_geode_robot,
                bought_ore_robot: true,
                bought_clay_robot: false,
                bought_obsidian_robot: false,
                bought_geode_robot: false,
                minute: current_state.minute + 1,
            });
        }

        if can_buy_clay_robot && !(current_state.can_buy_clay_robot && current_state.bought_nothing()) {
            states.push(State {
                num_ore_robots: current_state.num_ore_robots,
                num_clay_robots: current_state.num_clay_robots + 1,
                num_obsidian_robots: current_state.num_obsidian_robots,
                num_geode_robots: current_state.num_geode_robots,
                num_ore: current_state.num_ore + current_state.num_ore_robots - blueprint.cost_clay_robot_ore,
                num_clay: current_state.num_clay + current_state.num_clay_robots,
                num_obsidian: current_state.num_obsidian + current_state.num_obsidian_robots,
                num_geode: current_state.num_geode + current_state.num_geode_robots,
                can_buy_ore_robot,
                can_buy_clay_robot,
                can_buy_obsidian_robot,
                can_buy_geode_robot,
                bought_ore_robot: false,
                bought_clay_robot: true,
                bought_obsidian_robot: false,
                bought_geode_robot: false,
                minute: current_state.minute + 1,
            });
        }

        if can_buy_obsidian_robot && !(current_state.can_buy_obsidian_robot && current_state.bought_nothing()) {
            states.push(State {
                num_ore_robots: current_state.num_ore_robots,
                num_clay_robots: current_state.num_clay_robots,
                num_obsidian_robots: current_state.num_obsidian_robots + 1,
                num_geode_robots: current_state.num_geode_robots,
                num_ore: current_state.num_ore + current_state.num_ore_robots - blueprint.cost_obsidian_robot_ore,
                num_clay: current_state.num_clay + current_state.num_clay_robots - blueprint.cost_obsidian_robot_clay,
                num_obsidian: current_state.num_obsidian + current_state.num_obsidian_robots,
                num_geode: current_state.num_geode + current_state.num_geode_robots,
                can_buy_ore_robot,
                can_buy_clay_robot,
                can_buy_obsidian_robot,
                can_buy_geode_robot,
                bought_ore_robot: false,
                bought_clay_robot: false,
                bought_obsidian_robot: true,
                bought_geode_robot: false,
                minute: current_state.minute + 1,
            });
        }

        if can_buy_geode_robot && !(current_state.can_buy_geode_robot && current_state.bought_nothing()) {
            states.push(State {
                num_ore_robots: current_state.num_ore_robots,
                num_clay_robots: current_state.num_clay_robots,
                num_obsidian_robots: current_state.num_obsidian_robots,
                num_geode_robots: current_state.num_geode_robots + 1,
                num_ore: current_state.num_ore + current_state.num_ore_robots - blueprint.cost_geode_robot_ore,
                num_clay: current_state.num_clay + current_state.num_clay_robots,
                num_obsidian: current_state.num_obsidian + current_state.num_obsidian_robots - blueprint.cost_geode_robot_obsidian,
                num_geode: current_state.num_geode + current_state.num_geode_robots,
                can_buy_ore_robot,
                can_buy_clay_robot,
                can_buy_obsidian_robot,
                can_buy_geode_robot,
                bought_ore_robot: false,
                bought_clay_robot: false,
                bought_obsidian_robot: false,
                bought_geode_robot: true,
                minute: current_state.minute + 1,
            });
        }

        if !can_buy_ore_robot || !can_buy_clay_robot || !can_buy_obsidian_robot || !can_buy_geode_robot {
            states.push(State {
                num_ore_robots: current_state.num_ore_robots,
                num_clay_robots: current_state.num_clay_robots,
                num_obsidian_robots: current_state.num_obsidian_robots,
                num_geode_robots: current_state.num_geode_robots,
                num_ore: current_state.num_ore + current_state.num_ore_robots,
                num_clay: current_state.num_clay + current_state.num_clay_robots,
                num_obsidian: current_state.num_obsidian + current_state.num_obsidian_robots,
                num_geode: current_state.num_geode + current_state.num_geode_robots,
                can_buy_ore_robot,
                can_buy_clay_robot,
                can_buy_obsidian_robot,
                can_buy_geode_robot,
                bought_ore_robot: false,
                bought_clay_robot: false,
                bought_obsidian_robot: false,
                bought_geode_robot: false,
                minute: current_state.minute + 1,
            });
        }
    }

    best_state.num_geode
}

pub fn simulate_multi(blueprints: &[Blueprint], end_time: usize) -> Vec<(Blueprint, usize)> {
    blueprints
        .par_iter()
        .map(|blueprint| (blueprint.clone(), simulate(blueprint, end_time)))
        .collect()
}

#[test]
fn test() -> Result<(), Error> {
    // let input = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian."#;
    // let blueprint1 = load_blueprint(input)?;
    // let input = r#"Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;
    // let blueprint2 = load_blueprint(input)?;
    // assert_eq!(simulate_multi(&[blueprint1.clone(), blueprint2.clone()], 25), vec![9, 12]);
    // assert_eq!(simulate_multi(&[blueprint1.clone(), blueprint2.clone()], 33), vec![56, 62]);

    let blueprints = std::fs::read_to_string("input/day19")?
        .lines()
        .map(load_blueprint)
        .collect::<Result<Vec<Blueprint>, Error>>()?;

    assert_eq!(
        simulate_multi(&blueprints, 25)
            .iter()
            .map(|(bp, result)| bp.id * result)
            .sum::<usize>(),
        1487
    );

    assert_eq!(
        simulate_multi(&blueprints[0..3], 33)
            .iter()
            .map(|(_, result)| result)
            .product::<usize>(),
        13440
    );

    Ok(())
}

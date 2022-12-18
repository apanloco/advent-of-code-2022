// --- Day 18: Boiling Boulders ---
// part1: What is the surface area of your scanned lava droplet?
// part2: What is the exterior surface area of your scanned lava droplet?

use crate::error::Error;
use std::collections::HashSet;

type Cube = (i64, i64, i64);

pub fn load_cubes(s: &str) -> Result<Vec<Cube>, Error> {
    let mut cubes = Vec::new();
    for line in s.trim().lines() {
        let mut cords = line.trim().split(',');
        let x = cords.next().unwrap();
        let y = cords.next().unwrap();
        let z = cords.next().unwrap();
        cubes.push((x.parse()?, y.parse()?, z.parse()?));
    }
    Ok(cubes)
}

pub fn surface_area(cubes: &[Cube]) -> usize {
    let mut xyz = HashSet::new();

    for cube in cubes.iter() {
        xyz.insert((cube.0, cube.1, cube.2));
    }

    let zero_if_cube_exists = |cube: Cube| {
        if xyz.contains(&cube) {
            0
        } else {
            1
        }
    };
    let mut count = 0;
    for cube in cubes.iter() {
        count += zero_if_cube_exists((cube.0, cube.1, cube.2 - 1));
        count += zero_if_cube_exists((cube.0, cube.1, cube.2 + 1));
        count += zero_if_cube_exists((cube.0, cube.1 + 1, cube.2));
        count += zero_if_cube_exists((cube.0, cube.1 - 1, cube.2));
        count += zero_if_cube_exists((cube.0 + 1, cube.1, cube.2));
        count += zero_if_cube_exists((cube.0 - 1, cube.1, cube.2));
    }

    count
}

pub fn can_reach_water(cubes: &HashSet<Cube>, starting_point: Cube) -> bool {
    if cubes.contains(&starting_point) {
        return false;
    }

    let min_x = cubes.iter().map(|p| p.0).min().unwrap();
    let max_x = cubes.iter().map(|p| p.0).max().unwrap();
    let min_y = cubes.iter().map(|p| p.1).min().unwrap();
    let max_y = cubes.iter().map(|p| p.1).max().unwrap();
    let min_z = cubes.iter().map(|p| p.2).min().unwrap();
    let max_z = cubes.iter().map(|p| p.2).max().unwrap();

    let mut flow: HashSet<Cube> = HashSet::new();
    flow.insert(starting_point);
    loop {
        // any outside
        for cube in flow.iter() {
            if cube.0 <= min_x || cube.0 >= max_x {
                return true;
            }

            if cube.1 <= min_y || cube.1 >= max_y {
                return true;
            }

            if cube.2 <= min_z || cube.2 >= max_z {
                return true;
            }
        }

        let num_flow_cubes_before = flow.len();
        for cube in flow.clone().iter() {
            let new_cubes = vec![
                (cube.0 + 1, cube.1, cube.2),
                (cube.0 - 1, cube.1, cube.2),
                (cube.0, cube.1 + 1, cube.2),
                (cube.0, cube.1 - 1, cube.2),
                (cube.0, cube.1, cube.2 + 1),
                (cube.0, cube.1, cube.2 - 1),
            ];
            for new_cube in new_cubes.iter() {
                if !cubes.contains(new_cube) {
                    flow.insert(*new_cube);
                }
            }
        }

        if flow.len() == num_flow_cubes_before {
            return false;
        }
    }
}

pub fn outer_surface_area(cubes: &[Cube]) -> usize {
    let mut xyz = HashSet::new();

    for cube in cubes.iter() {
        xyz.insert((cube.0, cube.1, cube.2));
    }

    let one_if_can_reach_water = |cube: Cube| usize::from(can_reach_water(&xyz, cube));
    let mut count = 0;
    for cube in cubes.iter() {
        count += one_if_can_reach_water((cube.0, cube.1, cube.2 - 1));
        count += one_if_can_reach_water((cube.0, cube.1, cube.2 + 1));
        count += one_if_can_reach_water((cube.0, cube.1 + 1, cube.2));
        count += one_if_can_reach_water((cube.0, cube.1 - 1, cube.2));
        count += one_if_can_reach_water((cube.0 + 1, cube.1, cube.2));
        count += one_if_can_reach_water((cube.0 - 1, cube.1, cube.2));
    }

    count
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
    1,1,1
    2,1,1"#;
    let cubes = load_cubes(input)?;
    assert_eq!(cubes.len(), 2);
    assert_eq!(surface_area(&cubes), 10);

    let input = r#"
    1,1,1
    2,1,1
    1,2,1
    2,2,1
    1,3,1
    2,3,1"#;
    let cubes = load_cubes(input)?;
    assert_eq!(surface_area(&cubes), 22);

    let input = r#"
    1,1,1
    2,1,1
    1,2,1
    1,3,1
    2,3,1"#;
    let cubes = load_cubes(input)?;
    assert_eq!(surface_area(&cubes), 22);

    let input = r#"
    2,2,2
    1,2,2
    3,2,2
    2,1,2
    2,3,2
    2,2,1
    2,2,3
    2,2,4
    2,2,6
    1,2,5
    3,2,5
    2,1,5
    2,3,5"#;
    let cubes = load_cubes(input)?;
    assert_eq!(surface_area(&cubes), 64);
    assert_eq!(outer_surface_area(&cubes), 58);

    let cubes = load_cubes(&std::fs::read_to_string("input/day18")?)?;
    assert_eq!(surface_area(&cubes), 4608);
    assert_eq!(outer_surface_area(&cubes), 2652);

    Ok(())
}

// --- Day 15: Beacon Exclusion Zone ---
// part1: In the row where y=2000000, how many positions cannot contain a beacon?
// part2: Find the only possible position for the distress beacon. What is its tuning frequency?

use crate::error::Error;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};

type Point = (i64, i64);

pub fn load_line(line: &str) -> Result<(Point, Point), Error> {
    use text_io::try_scan;
    let sensor_x;
    let sensor_y;
    let beacon_x;
    let beacon_y;
    try_scan!(line.trim_start().trim_end().bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}", sensor_x, sensor_y, beacon_x, beacon_y);

    Ok(((sensor_x, sensor_y), (beacon_x, beacon_y)))
}

pub fn load_sensors_and_beacons(input: &str) -> Result<(Vec<Point>, Vec<Point>), Error> {
    let mut beacons: Vec<Point> = Vec::new();
    let mut sensors: Vec<Point> = Vec::new();
    for line in input.trim_start().trim_end().lines() {
        if line.trim_start().trim_end().is_empty() {
            continue;
        }
        let (sensor, beacon) = load_line(line)?;
        sensors.push(sensor);
        beacons.push(beacon);
    }
    Ok((sensors, beacons))
}

pub fn manhattan_distance(p1: Point, p2: Point) -> i64 {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as i64
}

pub fn num_no_beacon_points_at_row(input: &str, row: i64) -> Result<usize, Error> {
    let mut coverage_at_row: Vec<Point> = Vec::with_capacity(10_000_000); // min 7_672_418
    let (sensors, mut beacons) = load_sensors_and_beacons(input)?;
    for index in 0..sensors.len() {
        let sensor = sensors[index];
        let beacon = beacons[index];
        let distance = manhattan_distance(sensor, beacon);
        let diff_to_row = sensor.1.abs_diff(row) as i64;
        let left_to_sides = distance - diff_to_row;
        if left_to_sides > 0 {
            coverage_at_row.push((sensor.0, row));
            for x in 1..=left_to_sides {
                coverage_at_row.push(((sensor.0 - x), row));
                coverage_at_row.push(((sensor.0 + x), row));
            }
        }
    }

    beacons.sort();
    beacons.dedup();

    coverage_at_row.sort();
    coverage_at_row.dedup();

    Ok(coverage_at_row
        .into_iter()
        .filter(|p| !sensors.contains(p) && !beacons.contains(p))
        .count())
}

pub fn tuning_frequency(p: Point) -> i64 {
    p.0 * 4000000i64 + p.1
}

pub fn find_distress_beacon(input: &str) -> Result<Option<Point>, Error> {
    let (sensors, beacons) = load_sensors_and_beacons(input)?;
    let mut vision = Vec::with_capacity(sensors.len());
    let mut max_x = 0;
    let mut max_y = 0;
    for index in 0..sensors.len() {
        let sensor = sensors[index];
        let beacon = beacons[index];
        let sensor_vision = manhattan_distance(sensor, beacon);
        vision.push((sensor, sensor_vision));
        max_x = std::cmp::max(max_x, sensor.0);
        max_y = std::cmp::max(max_y, sensor.1);
    }

    max_x = std::cmp::min(max_x, 4000000);
    max_y = std::cmp::min(max_y, 4000000);

    let count = Arc::new(Mutex::new(0i64));
    let found: Option<Point> = None;
    let found = Arc::new(Mutex::new(found));
    let ys: Vec<i64> = (0..=max_y).collect();
    let _found = ys.par_iter().find_any(|y| {
        for x in 0..=max_x {
            // can anyone see this point?
            let visible = vision.iter().any(|(sensor_location, vision)| {
                let distance = manhattan_distance(*sensor_location, (x, **y));
                distance <= *vision
            });

            if !visible {
                println!("found: {} {}", x, y);
                *found.lock().unwrap() = Some((x, **y));
                return true;
            }
        }

        let mut lock = count.lock().unwrap();
        *lock += 1;
        if *lock % 1000 == 0 {
            println!("{}%", (*lock as f64 / max_y as f64) * 100f64);
        }

        false
    });

    let found = *found.lock().unwrap();
    if let Some(point) = found {
        Ok(Some(point))
    } else {
        Ok(None)
    }
}

#[test]
fn test() -> Result<(), Error> {
    assert_eq!(manhattan_distance((8, 7), (2, 10)), 9);
    assert_eq!(tuning_frequency((14, 11)), 56000011);

    let input = r#"
    Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "#;

    let n = num_no_beacon_points_at_row(input, 9)?;
    assert_eq!(n, 25);
    let n = num_no_beacon_points_at_row(input, 10)?;
    assert_eq!(n, 26);
    let n = num_no_beacon_points_at_row(input, 11)?;
    assert_eq!(n, 27);
    let n = num_no_beacon_points_at_row(&std::fs::read_to_string("input/day15")?, 2000000)?;
    println!("n: {}", n);
    assert_eq!(n, 4737443);

    let p = find_distress_beacon(input)?;
    assert_eq!(p, Some((14, 11)));
    assert_eq!(tuning_frequency(p.unwrap()), 56000011);

    let p = find_distress_beacon(&std::fs::read_to_string("input/day15")?)?;
    assert!(p.is_some());
    //found at (2870615, 2818989) (55.06158740517171%)
    assert_eq!(tuning_frequency(p.unwrap()), 11482462818989);

    Ok(())
}

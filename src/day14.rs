// --- Day 14: Regolith Reservoir ---
// part1: How many units of sand come to rest before sand starts flowing into the abyss below?
// part2: (sand line) How many units of sand come to rest?

use crate::error::Error;

type Point = (usize, usize);

#[derive(Clone, Copy)]
pub enum Object {
    Nothing,
    Wall,
    Sand,
}

pub struct Map {
    pub map: Vec<Object>,
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
    pub sand_line: Option<usize>,
}

impl Map {
    pub fn width(&self) -> usize {
        self.end_x - self.start_x + 1
    }

    pub fn height(&self) -> usize {
        self.end_y - self.start_y + 1
    }

    pub fn set_object_at(&mut self, x: usize, y: usize, object: Object) {
        let x = x - self.start_x;
        let y = y - self.start_y;
        self.map[y * (self.end_x - self.start_x + 1) + x] = object;
    }

    pub fn get_object_at(&self, x: usize, y: usize) -> Option<Object> {
        if let Some(sand_line) = self.sand_line {
            if y == sand_line {
                return Some(Object::Sand);
            }
        } else {
            if x < self.start_x {
                return None;
            }

            if x > self.end_x {
                return None;
            }

            if y > self.end_y {
                return None;
            }
        }
        Some(self.map[(y - self.start_y) * self.width() + (x - self.start_x)])
    }

    pub fn pour_from(&mut self, from: Point) -> usize {
        let mut num_settled = 0;
        'pouring: loop {
            let mut sand_x = from.0;
            let mut sand_y = from.1;
            'grain: loop {
                let alternatives = [(0, 1), (-1, 1), (1, 1)];
                for alternative in alternatives.iter() {
                    let test_x = (sand_x as i32 + alternative.0) as usize;
                    let test_y = (sand_y as i32 + alternative.1) as usize;
                    match self.get_object_at(test_x, test_y) {
                        None => {
                            break 'pouring;
                        }
                        Some(object) => match object {
                            Object::Nothing => {
                                sand_x = test_x;
                                sand_y = test_y;
                                continue 'grain;
                            }
                            Object::Wall => {}
                            Object::Sand => {}
                        },
                    };
                }
                num_settled += 1;
                self.set_object_at(sand_x, sand_y, Object::Sand);
                if (sand_x, sand_y) == from {
                    break 'pouring;
                } else {
                    break 'grain;
                }
            }
        }
        num_settled
    }
}

pub fn generate_map(s: &str, part2: bool) -> Result<Map, Error> {
    let mut point_list: Vec<Point> = Vec::new();
    for line in s.trim_start().trim_end().lines() {
        let mut last_point: Option<Point> = None;
        for point in line.split(" -> ") {
            let mut points = point.split(',');
            let current_point: Point = (points.next().unwrap().parse()?, points.next().unwrap().parse()?);
            if let Some(last_point) = last_point {
                for y in std::cmp::min(last_point.1, current_point.1)..=std::cmp::max(last_point.1, current_point.1) {
                    for x in std::cmp::min(last_point.0, current_point.0)..=std::cmp::max(last_point.0, current_point.0) {
                        point_list.push((x, y));
                    }
                }
            }
            last_point = Some(current_point)
        }
    }
    let start_y = 0;
    let end_y = point_list.iter().map(|p| p.1).max().unwrap() + 2;
    let start_x = point_list.iter().map(|p| p.0).min().unwrap() - end_y;
    let end_x = point_list.iter().map(|p| p.0).max().unwrap() + end_y;
    let map: Vec<Object> = vec![Object::Nothing; (end_y - start_y + 1) * (end_x - start_x + 1)];
    let mut map = Map {
        map,
        start_x,
        start_y,
        end_x,
        end_y,
        sand_line: if part2 { Some(end_y) } else { None },
    };
    for point in point_list {
        map.set_object_at(point.0, point.1, Object::Wall);
    }
    Ok(map)
}

pub fn dump(map: &Map) {
    for y in 0..map.height() {
        for x in 0..map.width() {
            match map.get_object_at(x, y).unwrap() {
                Object::Nothing => {
                    print!(".");
                }
                Object::Wall => {
                    print!("#");
                }
                Object::Sand => {
                    print!("o");
                }
            }
        }
        println!();
    }
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
    let mut map = generate_map(input, false)?;
    assert_eq!(map.pour_from((500, 0)), 24);

    let mut map = generate_map(input, true)?;
    assert_eq!(map.pour_from((500, 0)), 93);

    let mut map = generate_map(&std::fs::read_to_string("input/day14")?, false)?;
    assert_eq!(map.pour_from((500, 0)), 862);

    let mut map = generate_map(&std::fs::read_to_string("input/day14")?, true)?;
    assert_eq!(map.pour_from((500, 0)), 28744);

    Ok(())
}

// --- Day 8: Treetop Tree House ---
// part1: Consider your map; how many trees are visible from outside the grid?
// part2: Consider each tree on your map. What is the highest scenic score possible for any tree?

use crate::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub struct Map {
    pub map: Vec<Vec<usize>>,
}

impl FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map {
            map: s
                .trim_start()
                .trim_end()
                .lines()
                .map(|line| line.chars().map(|c| c as usize - '0' as usize).collect())
                .collect(),
        })
    }
}

impl Map {
    pub fn width(&self) -> usize {
        self.map.first().unwrap().len() as usize
    }

    pub fn height(&self) -> usize {
        self.map.len() as usize
    }

    pub fn tree_height_at(&self, x: usize, y: usize) -> usize {
        self.map[y][x]
    }

    pub fn visibility_left(&self, x: usize, y: usize) -> (bool, usize) {
        for index in (0..x).rev() {
            if self.tree_height_at(index, y) >= self.tree_height_at(x, y) {
                return (false, x - index);
            }
        }
        (true, x)
    }

    pub fn visibility_right(&self, x: usize, y: usize) -> (bool, usize) {
        for index in (x + 1)..self.width() {
            if self.tree_height_at(index, y) >= self.tree_height_at(x, y) {
                return (false, index - x);
            }
        }
        (true, self.width() - x - 1)
    }

    pub fn visibility_up(&self, x: usize, y: usize) -> (bool, usize) {
        for index in (0..y).rev() {
            if self.tree_height_at(x, index) >= self.tree_height_at(x, y) {
                return (false, y - index);
            }
        }
        (true, y)
    }

    pub fn visibility_down(&self, x: usize, y: usize) -> (bool, usize) {
        for index in (y + 1)..self.height() {
            if self.tree_height_at(x, index) >= self.tree_height_at(x, y) {
                return (false, index - y);
            }
        }
        (true, self.height() - y - 1)
    }

    pub fn is_visible_left(&self, x: usize, y: usize) -> bool {
        self.visibility_left(x, y).0
    }

    pub fn is_visible_right(&self, x: usize, y: usize) -> bool {
        self.visibility_right(x, y).0
    }

    pub fn is_visible_up(&self, x: usize, y: usize) -> bool {
        self.visibility_up(x, y).0
    }

    pub fn is_visible_down(&self, x: usize, y: usize) -> bool {
        self.visibility_down(x, y).0
    }

    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        self.is_visible_left(x, y) || self.is_visible_right(x, y) || self.is_visible_up(x, y) || self.is_visible_down(x, y)
    }

    pub fn scenic_score(&self, x: usize, y: usize) -> usize {
        self.visibility_left(x, y).1 * self.visibility_right(x, y).1 * self.visibility_up(x, y).1 * self.visibility_down(x, y).1
    }

    pub fn count_visible(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.is_visible(x, y) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn highest_scenic_score(&self) -> (usize, usize, usize) {
        let mut highest_score = 0;
        let mut highest_x = 0;
        let mut highest_y = 0;
        for x in 0..self.width() {
            for y in 0..self.height() {
                let score = self.scenic_score(x, y);
                if score > highest_score {
                    highest_score = score;
                    highest_x = x;
                    highest_y = y;
                }
            }
        }
        (highest_score, highest_x, highest_y)
    }
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
30373
25512
65332
33549
35390"#;

    let map: Map = input.parse()?;
    assert_eq!(map.width(), 5);
    assert_eq!(map.height(), 5);
    assert_eq!(map.count_visible(), 21);
    assert!(map.is_visible(1, 1));
    assert!(map.is_visible(2, 1));
    assert!(!map.is_visible(3, 1));
    assert!(map.is_visible(1, 2));
    assert!(!map.is_visible(2, 2));
    assert!(map.is_visible(3, 2));

    assert_eq!(map.visibility_left(2, 1).1, 1);
    assert_eq!(map.visibility_right(2, 1).1, 2);
    assert_eq!(map.visibility_up(2, 1).1, 1);
    assert_eq!(map.visibility_down(2, 1).1, 2);

    assert_eq!(map.visibility_left(0, 0).1, 0);
    assert_eq!(map.visibility_right(4, 0).1, 0);
    assert_eq!(map.visibility_up(0, 0).1, 0);
    assert_eq!(map.visibility_down(0, 4).1, 0);

    assert_eq!(map.scenic_score(2, 1), 4);
    assert_eq!(map.scenic_score(2, 3), 8);

    let (score, x, y) = map.highest_scenic_score();
    assert_eq!(score, 8);
    assert_eq!((x, y), (2, 3));

    let map: Map = std::fs::read_to_string("input/day8")?.parse()?;
    assert_eq!(map.count_visible(), 1647);
    assert_eq!(map.visibility_up(50, 86), (false, 6));
    assert_eq!(map.highest_scenic_score().0, 392080);

    Ok(())
}

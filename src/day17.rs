use crate::error::Error;

type Point = (i64, i64);

#[derive(PartialEq, Debug, Clone, Eq)]
pub enum ShapeType {
    Minus,
    Plus,
    L,
    I,
    Box,
}

#[derive(Clone)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub data: Vec<Point>,
}

impl Shape {
    pub fn width(&self) -> i64 {
        let min_x = self.data.iter().map(|(x, _y)| x).min().unwrap();
        let max_x = self.data.iter().map(|(x, _y)| x).max().unwrap();
        max_x - min_x + 1
    }

    pub fn height(&self) -> i64 {
        let min_y = self.data.iter().map(|(_x, y)| y).min().unwrap();
        let max_y = self.data.iter().map(|(_x, y)| y).max().unwrap();
        max_y - min_y + 1
    }
}

pub struct Shapes {
    shapes: Vec<Shape>,
    next_shape: usize,
}

impl Shapes {
    pub fn new() -> Self {
        let shapes = vec![
            Shape {
                shape_type: ShapeType::Minus,
                data: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            },
            Shape {
                shape_type: ShapeType::Plus,
                data: vec![(1, 0), (0, -1), (1, -1), (2, -1), (1, -2)],
            },
            Shape {
                shape_type: ShapeType::L,
                data: vec![(0, 0), (1, 0), (2, 0), (2, -1), (2, -2)],
            },
            Shape {
                shape_type: ShapeType::I,
                data: vec![(0, 0), (0, -1), (0, -2), (0, -3)],
            },
            Shape {
                shape_type: ShapeType::Box,
                data: vec![(0, 0), (1, 0), (0, -1), (1, -1)],
            },
        ];
        Shapes { shapes, next_shape: 0 }
    }

    pub fn next_shape(&mut self) -> &Shape {
        let next_shape = &self.shapes[self.next_shape];
        self.next_shape += 1;
        self.next_shape %= self.shapes.len();
        next_shape
    }
}

impl Default for Shapes {
    fn default() -> Self {
        Shapes::new()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Pattern {
    PushLeft,
    PushRight,
}

pub struct Patterns {
    patterns: Vec<Pattern>,
    next_pattern: usize,
}

fn parse_patterns(s: &str) -> Result<Vec<Pattern>, Error> {
    Ok(s.trim()
        .chars()
        .map(|c| match c {
            '<' => Pattern::PushLeft,
            '>' => Pattern::PushRight,
            _ => panic!("invalid pattern: {}", c),
        })
        .collect())
}

impl Patterns {
    pub fn new(s: &str) -> Self {
        Patterns {
            patterns: parse_patterns(s).expect("failed to parse patterns"),
            next_pattern: 0,
        }
    }

    pub fn reset(&mut self) {
        self.next_pattern = 0;
    }

    pub fn next_pattern(&mut self) -> &Pattern {
        let next_pattern = &self.patterns[self.next_pattern];
        self.next_pattern += 1;
        self.next_pattern %= self.patterns.len();
        next_pattern
    }
}

pub struct Tetris {
    pub patterns: Patterns,
    pub shapes: Shapes,
    pub current_shape: Shape,
    pub num_rest: usize,
    pub current_shape_x: i64,
    pub current_shape_y: i64,
    pub map: Vec<Vec<MapShape>>,
}

pub fn add_dump_remove(tetris: &mut Tetris, shape: &Shape, shape_x: i64, shape_y: i64, map_shape: MapShape) {
    tetris.add_shape(shape, shape_x, shape_y, map_shape);
    tetris.dump();
    tetris.remove_shape(shape, shape_x, shape_y);
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MapShape {
    EmptySpace,
    FallingRock,
    SettledRock,
}

impl MapShape {
    pub fn to_char(&self) -> char {
        match self {
            MapShape::EmptySpace => '.',
            MapShape::FallingRock => '@',
            MapShape::SettledRock => '#',
        }
    }
}

pub fn new_map_row() -> Vec<MapShape> {
    vec![
        MapShape::EmptySpace,
        MapShape::EmptySpace,
        MapShape::EmptySpace,
        MapShape::EmptySpace,
        MapShape::EmptySpace,
        MapShape::EmptySpace,
        MapShape::EmptySpace,
    ]
}

impl Tetris {
    pub fn add_new_map_row(&mut self) {
        self.map.insert(0, new_map_row());
    }

    pub(crate) fn reached_end(&self, shape_y: i64) -> bool {
        shape_y >= self.map.len() as i64
    }

    pub fn new(patterns: Patterns, mut shapes: Shapes) -> Self {
        let current_shape = shapes.next_shape().clone();
        Tetris {
            patterns,
            shapes,
            current_shape,
            num_rest: 0,
            current_shape_x: 2,
            current_shape_y: 0,
            map: vec![new_map_row(), new_map_row(), new_map_row(), new_map_row()],
        }
    }

    pub fn settled_height(&self) -> usize {
        for (y, row) in self.map.iter().enumerate() {
            for (_x, ms) in row.iter().enumerate() {
                if ms == &MapShape::SettledRock {
                    return self.map.len() - y;
                }
            }
        }
        0
    }

    pub fn dump(&mut self) {
        // println!("=======");
        // for (_y, row) in self.map.iter().enumerate() {
        //     for (_x, ms) in row.iter().enumerate() {
        //         print!("{}", ms.to_char());
        //     }
        //     println!();
        // }
        // println!("=======");
    }

    pub fn draw_at(&mut self, x: i64, y: i64, map_shape: &MapShape) {
        self.map[y as usize][x as usize] = *map_shape;
    }

    fn add_shape(&mut self, shape: &Shape, shape_x: i64, shape_y: i64, map_shape: MapShape) {
        for point in &shape.data {
            self.draw_at(shape_x + point.0, shape_y + point.1, &map_shape);
        }
    }

    fn remove_shape(&mut self, shape: &Shape, shape_x: i64, shape_y: i64) {
        self.add_shape(shape, shape_x, shape_y, MapShape::EmptySpace)
    }

    fn collides(&mut self, shape: &Shape, shape_x: i64, shape_y: i64) -> bool {
        for point in &shape.data {
            let test_x = (shape_x + point.0) as usize;
            let test_y = (shape_y + point.1) as usize;
            if self.map[test_y][test_x] != MapShape::EmptySpace {
                return true;
            }
        }
        false
    }
}

pub fn simulate(tetris: &mut Tetris, steps: usize) {
    println!("start simulation");
    tetris.dump();
    for _step in 0..steps {
        //println!("new simulation (adjust map height and sprite position)");
        let shape_height = tetris.current_shape.height();
        let current_height = tetris.settled_height();
        let wanted_height = current_height + shape_height as usize + 3;
        while tetris.map.len() < wanted_height {
            tetris.add_new_map_row();
        }
        tetris.current_shape_y = tetris.map.len() as i64 - current_height as i64 - 4;

        loop {
            //println!("show state");
            add_dump_remove(
                tetris,
                &tetris.current_shape.clone(),
                tetris.current_shape_x,
                tetris.current_shape_y,
                MapShape::FallingRock,
            );

            let mut horizontal_move = 0;
            let next_pattern = tetris.patterns.next_pattern();
            //println!("next: {:?}", next_pattern);
            match next_pattern {
                Pattern::PushLeft => {
                    if tetris.current_shape_x > 0 {
                        horizontal_move = -1;
                    }
                }
                Pattern::PushRight => {
                    if 7 - tetris.current_shape.width() - tetris.current_shape_x > 0 {
                        horizontal_move = 1;
                    }
                }
            }

            if horizontal_move != 0
                && !tetris.collides(
                    &tetris.current_shape.clone(),
                    tetris.current_shape_x + horizontal_move,
                    tetris.current_shape_y,
                )
            {
                tetris.current_shape_x += horizontal_move;
            }

            //println!("moved horizontal {}", horizontal_move);
            add_dump_remove(
                tetris,
                &tetris.current_shape.clone(),
                tetris.current_shape_x,
                tetris.current_shape_y,
                MapShape::FallingRock,
            );

            tetris.current_shape_y += 1;

            if tetris.reached_end(tetris.current_shape_y)
                || tetris.collides(&tetris.current_shape.clone(), tetris.current_shape_x, tetris.current_shape_y)
            {
                //println!("settled");
                tetris.current_shape_y -= 1;
                tetris.add_shape(
                    &tetris.current_shape.clone(),
                    tetris.current_shape_x,
                    tetris.current_shape_y,
                    MapShape::SettledRock,
                );
                tetris.num_rest += 1;
                break;
            }

            //println!("moved down");
            add_dump_remove(
                tetris,
                &tetris.current_shape.clone(),
                tetris.current_shape_x,
                tetris.current_shape_y,
                MapShape::FallingRock,
            );
        }

        tetris.current_shape = tetris.shapes.next_shape().clone();
        tetris.current_shape_y = tetris.current_shape.height() - 1;
        tetris.current_shape_x = 2;

        //println!("dummy println for step debugging");
    }
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;
    // let mut tetris = Tetris::new(Patterns::new(input), Shapes::new());
    // simulate(&mut tetris, 1);
    // assert_eq!(tetris.num_rest, 1);
    // assert_eq!(tetris.settled_height(), 1);
    // simulate(&mut tetris, 1);
    // assert_eq!(tetris.num_rest, 2);
    // assert_eq!(tetris.settled_height(), 4);
    // simulate(&mut tetris, 1);
    // assert_eq!(tetris.num_rest, 3);
    // assert_eq!(tetris.settled_height(), 6);

    // let mut tetris = Tetris::new(Patterns::new(input), Shapes::new());
    // simulate(&mut tetris, 2022);
    // assert_eq!(tetris.settled_height(), 3068);
    //
    // let mut tetris = Tetris::new(Patterns::new(&std::fs::read_to_string("input/day17")?), Shapes::new());
    // simulate(&mut tetris, 2022);
    // assert_eq!(tetris.settled_height(), 3193);

    let mut tetris = Tetris::new(Patterns::new(&std::fs::read_to_string("input/day17")?), Shapes::new());
    simulate(&mut tetris, 1000000);
    assert_eq!(tetris.settled_height(), 999);

    //assert_eq!(height, 3068);
    Ok(())
}

// #[test]
// fn test_pattern() -> Result<(), Error> {
//     let input = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;
//     let mut patterns: Patterns = Patterns::new(input);
//     for _ in 0..40 {
//         patterns.next_pattern();
//     }
//     assert_eq!(patterns.next_pattern(), &Pattern::PushRight);
//     assert_eq!(patterns.next_pattern(), &Pattern::PushRight);
//     assert_eq!(patterns.next_pattern(), &Pattern::PushRight);
//     assert_eq!(patterns.next_pattern(), &Pattern::PushLeft);
//     assert_eq!(patterns.next_pattern(), &Pattern::PushLeft);
//     assert_eq!(patterns.next_pattern(), &Pattern::PushRight);
//     assert_eq!(patterns.next_pattern(), &Pattern::PushLeft);
//     assert_eq!(patterns.next_pattern(), &Pattern::PushRight);
//     assert_eq!(patterns.next_pattern(), &Pattern::PushRight);
//     assert_eq!(patterns.next_pattern(), &Pattern::PushLeft);
//     assert_eq!(patterns.next_pattern(), &Pattern::PushLeft);
//     assert_eq!(patterns.next_pattern(), &Pattern::PushLeft);
//     Ok(())
// }
//
// #[test]
// fn test_shapes() -> Result<(), Error> {
//     let mut shapelist = Shapes::new();
//     assert_eq!(shapelist.next_shape().shape_type, ShapeType::Minus);
//     assert_eq!(shapelist.next_shape().shape_type, ShapeType::Plus);
//     assert_eq!(shapelist.next_shape().shape_type, ShapeType::L);
//     assert_eq!(shapelist.next_shape().shape_type, ShapeType::I);
//     assert_eq!(shapelist.next_shape().shape_type, ShapeType::Box);
//     assert_eq!(shapelist.next_shape().width(), 4);
//     assert_eq!(shapelist.next_shape().width(), 3);
//     assert_eq!(shapelist.next_shape().width(), 3);
//     assert_eq!(shapelist.next_shape().width(), 1);
//     assert_eq!(shapelist.next_shape().width(), 2);
//     assert_eq!(shapelist.next_shape().height(), 1);
//     assert_eq!(shapelist.next_shape().height(), 3);
//     assert_eq!(shapelist.next_shape().height(), 3);
//     assert_eq!(shapelist.next_shape().height(), 4);
//     assert_eq!(shapelist.next_shape().height(), 2);
//     Ok(())
// }

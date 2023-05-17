use crate::error::Error;

type Point = (i64, i64);

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Void,
    Empty,
    Wall,
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    TurnRight,
    TurnLeft,
    Move(usize),
}

#[derive(Debug)]
pub struct Game {
    pub tiles: Vec<Tile>,
    pub instructions: Vec<Instruction>,
    pub map_width: i64,
    pub map_height: i64,
}

impl Game {
    pub fn find_start_pos(&self) -> Point {
        for y in 0..self.map_height {
            for x in 0..self.map_width {
                match self.get_tile_at(x, y) {
                    Tile::Void => {}
                    Tile::Empty => return (x, y),
                    Tile::Wall => {}
                }
            }
        }
        panic!("start pos not found");
    }

    fn get_tile_at(&self, x: i64, y: i64) -> Tile {
        self.tiles[(y * self.map_width + x) as usize]
    }

    fn dump(&self, pos: Option<Point>) {
        for y in 0..self.map_height {
            for x in 0..self.map_width {
                let mut c = match self.get_tile_at(x, y) {
                    Tile::Void => ' ',
                    Tile::Empty => '.',
                    Tile::Wall => '#',
                };
                if let Some(pos) = pos {
                    if x == pos.0 && y == pos.1 {
                        c = '%';
                    }
                }
                print!("{}", c);
            }
            println!();
        }
    }
}

pub fn load_game(input: &str) -> Result<Game, Error> {
    let mut tiles: Vec<Tile> = Vec::new();

    let lines: Vec<String> = input.lines().map(|l| l.to_string()).filter(|l| !l.is_empty()).collect();

    let map_lines = &lines[0..lines.len() - 1];
    let instr_line = lines[lines.len() - 1].to_string();

    let map_width = map_lines.iter().map(|l| l.len()).max().unwrap();
    for line in map_lines {
        println!("map line: {}", line);
        for c in line.chars() {
            tiles.push(match c {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                ' ' => Tile::Void,
                _ => panic!("invalid tile: {}", c),
            });
        }
        if line.len() < map_width {
            for _ in 0..(map_width - line.len()) {
                tiles.push(Tile::Void);
            }
        }
    }

    println!("instr line: {}", instr_line);

    let mut instructions: Vec<Instruction> = Vec::new();
    for c in instr_line.chars() {
        match c {
            'R' => {
                instructions.push(Instruction::TurnRight);
            }
            'L' => {
                instructions.push(Instruction::TurnLeft);
            }
            _ => match instructions.last().copied() {
                None => instructions.push(Instruction::Move(c as usize - '0' as usize)),
                Some(last) => match last {
                    Instruction::TurnRight | Instruction::TurnLeft => instructions.push(Instruction::Move(c as usize - '0' as usize)),
                    Instruction::Move(old) => {
                        instructions.pop();
                        instructions.push(Instruction::Move(format!("{}{}", old, c).parse().unwrap()))
                    }
                },
            },
        };
    }

    Ok(Game {
        map_height: map_lines.len() as i64,
        map_width: map_width as i64,
        tiles,
        instructions,
    })
}

fn move_in_direction(game: &Game, mut current_pos: Point, direction: (i64, i64)) -> Point {
    let mut new_pos = current_pos;
    loop {
        new_pos.0 += direction.0;
        new_pos.1 += direction.1;

        if new_pos.0 < 0 {
            new_pos.0 = game.map_width - 1;
        }
        if new_pos.0 > game.map_width - 1 {
            new_pos.0 = 0;
        }
        if new_pos.1 < 0 {
            new_pos.1 = game.map_height - 1;
        }
        if new_pos.1 > game.map_height - 1 {
            new_pos.1 = 0;
        }

        match game.get_tile_at(new_pos.0, new_pos.1) {
            Tile::Void => {}
            Tile::Empty => return new_pos,
            Tile::Wall => return current_pos,
        }
    }
}

fn move_in_direction_amount(game: &Game, mut current_pos: Point, direction: (i64, i64), num_moves: usize) -> Point {
    for _ in 0..num_moves {
        current_pos = move_in_direction(game, current_pos, direction);
    }
    current_pos
}

fn simulate(game: &Game) -> i64 {
    let mut pos = game.find_start_pos();
    let mut rotation = 0i64;
    //    println!("start pos: {:?}", pos);

    for i in &game.instructions {
        //        game.dump(Some(pos));
        //        println!("processing instruction: {:?}", i);
        match i {
            Instruction::TurnRight => {
                rotation = (rotation + 1).rem_euclid(4);
            }
            Instruction::TurnLeft => {
                rotation = (rotation - 1).rem_euclid(4);
            }
            Instruction::Move(amount) => {
                let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
                let direction = directions[rotation as usize];
                pos = move_in_direction_amount(game, pos, direction, *amount);
            }
        }
    }

    //    println!("done");
    //    game.dump(Some(pos));

    let row = pos.1 + 1;
    let col = pos.0 + 1;
    let dir = rotation as i64;

    1000 * row + 4 * col + dir
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;

    let game = load_game(input)?;
    assert_eq!(simulate(&game), 6032);

    let game = load_game(&std::fs::read_to_string("input/day22")?)?;
    assert_eq!(simulate(&game), 6032);

    Ok(())
}

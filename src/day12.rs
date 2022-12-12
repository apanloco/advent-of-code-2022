// --- Day 12: Hill Climbing Algorithm ---
// part1: What is the fewest steps required to move from your current position to the location that should get the best signal?
// part2: What is the fewest steps required to move starting from any square with elevation a to the location that should get the best signal?

use crate::error::Error;
use petgraph::algo::astar;
use petgraph::graph::{DefaultIx, DiGraph, NodeIndex};
use petgraph::Graph;
use std::str::FromStr;

type Pos = (usize, usize);

#[derive(Debug)]
pub struct Game {
    pub map: Vec<Vec<char>>,
    pub start_position: Pos,
    pub end_position: Pos,
}

impl FromStr for Game {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut start_position: Option<Pos> = None;
        let mut end_position: Option<Pos> = None;
        for (row_index, line) in s.trim_start().trim_end().lines().enumerate() {
            let mut row = Vec::new();
            for (column_index, c) in line.chars().enumerate() {
                let height = match c {
                    'a'..='z' => c,
                    'E' => {
                        end_position = Some((column_index as usize, row_index as usize));
                        'z'
                    }
                    'S' => {
                        start_position = Some((column_index as usize, row_index as usize));
                        'a'
                    }
                    _ => return Err(Error::General(format!("invalid char: {}", c))),
                };
                row.push(height);
            }
            map.push(row);
        }
        Ok(Game {
            map,
            start_position: start_position.unwrap(),
            end_position: end_position.unwrap(),
        })
    }
}

impl Game {
    pub fn map_width(&self) -> usize {
        self.map.first().unwrap().len() as usize
    }

    pub fn map_height(&self) -> usize {
        self.map.len() as usize
    }

    pub fn height_at(&self, x: usize, y: usize) -> usize {
        self.map[y][x] as usize - 'a' as usize
    }

    fn get_edges_for_pos(&self, x: usize, y: usize) -> Vec<(Pos, Pos)> {
        let mut edges = Vec::new();

        // top
        if y > 0 {
            edges.push(((x, y), (x, y - 1)));
        }

        // right
        if x < self.map_width() - 1 {
            edges.push(((x, y), (x + 1, y)));
        }

        // bottom
        if y < self.map_height() - 1 {
            edges.push(((x, y), (x, y + 1)));
        }

        // left
        if x > 0 {
            edges.push(((x, y), (x - 1, y)));
        }

        edges
    }

    fn get_node_from_pos(&self, pos: Pos) -> NodeIndex<DefaultIx> {
        NodeIndex::new(pos.1 * self.map_width() + pos.0)
    }

    fn get_pos_from_node(&self, node: NodeIndex<DefaultIx>) -> Pos {
        (node.index() % self.map_width(), node.index() / self.map_width())
    }

    pub fn minimum_steps_from_any_a(&self) -> usize {
        let mut min_steps: Option<usize> = None;
        let graph = self.get_graph();
        for y in 0..self.map_height() {
            for x in 0..self.map_width() {
                if self.height_at(x, y) == 0 {
                    let steps = self.minimum_steps_from(&graph, (x, y));
                    if let Some(steps) = steps {
                        if let Some(value) = min_steps {
                            if steps < value {
                                min_steps = Some(steps);
                            }
                        } else {
                            min_steps = Some(steps);
                        }
                    }
                }
            }
        }
        min_steps.unwrap()
    }

    pub fn get_graph(&self) -> Graph<usize, Pos> {
        let mut graph: Graph<usize, Pos> = DiGraph::new();

        // one node will be created for each point on the map
        for _ in 0..self.map_width() * self.map_height() {
            graph.add_node(1);
        }

        // create the possible edges
        let mut edges: Vec<(Pos, Pos)> = Vec::new();
        for y in 0..self.map_height() {
            for x in 0..self.map_width() {
                let mut edges_for_pos = self.get_edges_for_pos(x, y);
                edges.append(&mut edges_for_pos);
            }
        }

        // remove edges that are not possible and map them from (pos, pos) to (node, node)
        let edges: Vec<(NodeIndex<DefaultIx>, NodeIndex<DefaultIx>)> = edges
            .into_iter()
            .filter(|(from, to)| {
                let height_from = self.height_at(from.0, from.1) as i64;
                let height_to = self.height_at(to.0, to.1) as i64;
                height_to - height_from <= 1
            })
            .map(|(from, to)| (self.get_node_from_pos(from), self.get_node_from_pos(to)))
            .collect();

        graph.extend_with_edges(&edges);

        graph
    }
    pub fn minimum_steps_from(&self, graph: &Graph<usize, Pos>, start_position: Pos) -> Option<usize> {
        self.minimum_steps_from_dijkstra(graph, start_position)
    }

    pub fn minimum_steps_from_dijkstra(&self, graph: &Graph<usize, Pos>, start_position: Pos) -> Option<usize> {
        let finish_node = self.get_node_from_pos(self.end_position);
        let path = petgraph::algo::dijkstra(&graph, self.get_node_from_pos(start_position), Some(finish_node), |_| 1usize);
        path.get(&finish_node).copied()
    }

    pub fn _minimum_steps_from_astar(&self, graph: &Graph<usize, Pos>, start_position: Pos) -> Option<usize> {
        let finish_node = self.get_node_from_pos(self.end_position);
        let path = astar(
            &graph,
            self.get_node_from_pos(start_position),
            |finish| finish == finish_node,
            |_e| 1,
            |node| {
                let from_pos = self.get_pos_from_node(node);
                let to_pos = self.get_pos_from_node(finish_node);
                from_pos.0.abs_diff(to_pos.0) + from_pos.1.abs_diff(to_pos.1)
            },
        );

        if let Some(path) = path {
            Some(path.0)
        } else {
            None
        }
    }
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
    let game = input.parse::<Game>()?;
    assert_eq!((game.map_width(), game.map_height()), (8, 5));
    assert_eq!(game.start_position, (0, 0));
    assert_eq!(game.end_position, (5, 2));
    assert_eq!(game.height_at(0, 0), 0);
    assert_eq!(game.height_at(5, 2), 25);
    assert_eq!(game.height_at(7, 4), 'i' as usize - 'a' as usize);
    assert_eq!(game.minimum_steps_from(&game.get_graph(), game.start_position), Some(31));
    assert_eq!(game.minimum_steps_from_any_a(), 29);

    let game = std::fs::read_to_string("input/day12")?.parse::<Game>()?;
    assert_eq!(game.minimum_steps_from(&game.get_graph(), game.start_position), Some(517));
    assert_eq!(game.minimum_steps_from_any_a(), 512);

    Ok(())
}

use rand::{thread_rng, Rng};
use std::collections::HashMap;

type Position = (i32, i32);

pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub visited: bool
}

pub struct Maze {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Vec<bool>>,
    pub horizontal_walls: HashMap<Position, bool>,
    pub vertical_walls: HashMap<Position, bool>,
    pub active_position: Position,
    pub completed: bool,
    stack: Vec<Position>,
}

impl Maze {
    pub fn new(width: i32, height: i32) -> Self {
        let cells = vec![vec![false; height as usize]; width as usize];
        let mut horizontal_walls: HashMap<Position, bool> = HashMap::new();
        let mut vertical_walls: HashMap<Position, bool> = HashMap::new();

        for y in 0..height {
            for x in 1..width {
                vertical_walls.insert((x, y), true);
                horizontal_walls.insert((y, x), true);
            }
        }

        Maze {
            width,
            height,
            cells,
            active_position: (0, 0),
            stack: Vec::new(),
            horizontal_walls,
            vertical_walls,
            completed: false,
        }
    }

    pub fn iter(&self) -> MazeIterator {
        MazeIterator::new(self)
    }

    pub fn is_visited(&self, pos: Position) -> bool {
        self.cells[pos.0 as usize][pos.1 as usize]
    }

    pub fn set_visited(&mut self, pos: Position, visited: bool) {
        self.cells[pos.0 as usize][pos.1 as usize] = visited;
    }

    fn pick_random_direction(&self) -> Position {
        let mut rng = thread_rng();
        let n = rng.gen_range(0..4);
        match n {
            0 => (1, 0),
            1 => (-1, 0),
            2 => (0, 1),
            3 => (0, -1),
            _ => panic!("Invalid direction, this is weird and should not happen."),
        }
    }

    fn outside_grid(&self, point: Position) -> bool {
        point.0 < 0 || point.0 > self.width - 1 || point.1 < 0 || point.1 > self.height - 1
    }

    fn is_dead_end(&self, point: Position) -> bool {
        let up = (point.0, point.1 - 1);
        let down = (point.0, point.1 + 1);
        let left = (point.0 - 1, point.1);
        let right = (point.0 + 1, point.1);

        [up, down, left, right]
            .iter()
            .filter(|p| !self.outside_grid(**p))
            .all(|p| self.is_visited(*p))
    }

    fn backtrack(&mut self) {
        if let Some(pos) = self.stack.pop() {
            self.active_position = pos;
        }
    }

    pub fn generate_next(&mut self) -> bool {
        let dir = self.pick_random_direction();
        let next_cell = (
            self.active_position.0 + dir.0,
            self.active_position.1 + dir.1,
        );

        if self.outside_grid(next_cell) {
            return false;
        } else if self.is_dead_end(self.active_position) {
            self.backtrack();
            let backtracked_to_beginning = self.active_position == (0, 0);
            self.completed = backtracked_to_beginning;
            return false;
        } else if !self.is_visited(next_cell) {
            self.set_visited(self.active_position, true);
            self.set_visited(next_cell, true);

            match dir {
                (1, 0) => {
                    self.vertical_walls.insert(next_cell, false);
                }
                (-1, 0) => {
                    self.vertical_walls.insert(self.active_position, false);
                }
                (0, 1) => {
                    self.horizontal_walls.insert(next_cell, false);
                }
                (0, -1) => {
                    self.horizontal_walls.insert(self.active_position, false);
                }
                _ => panic!("Invalid direction, this is weird and should not happen."),
            }

            self.stack.push(self.active_position);
            self.active_position = next_cell;

            return true;
        } else {
            return false;
        }
    }
}

pub struct MazeIterator<'a> {
    maze: &'a Maze,
    x: i32,
    y: i32,
}

impl<'a> MazeIterator<'a> {
    pub fn new(maze: &'a Maze) -> Self {
        MazeIterator { maze, x: 0, y: 0 }
    }
}

impl<'a> Iterator for MazeIterator<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.maze.outside_grid((self.x, self.y)) {
            return None;
        }

        let return_value = Some(Cell {
            x: self.x,
            y: self.y,
            visited: self.maze.is_visited((self.x, self.y)),
        });

        self.x += 1;
        if self.x > self.maze.width - 1 {
            self.x = 0;
            self.y += 1;
        }

        return_value
    }
}

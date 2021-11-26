use rand::{thread_rng, Rng};

type Position = (i32, i32);

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellType {
    Wall,
    Path,
}

pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub cell_type: CellType,
}

pub struct Maze {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Vec<CellType>>,
    pub active_position: Position,
    stack: Vec<Position>,
}

impl Maze {
    pub fn new(width: i32, height: i32) -> Self {
        let cells = vec![vec![CellType::Wall; height as usize]; width as usize];
        Maze {
            width,
            height,
            cells,
            active_position: (0, 0),
            stack: Vec::new(),
        }
    }

    pub fn iter(&self) -> MazeIterator {
        MazeIterator::new(self)
    }

    pub fn get_cell_type(&self, pos: Position) -> CellType {
        self.cells[pos.0 as usize][pos.1 as usize]
    }

    pub fn set_cell_type(&mut self, pos: Position, cell_type: CellType) {
        self.cells[pos.0 as usize][pos.1 as usize] = cell_type;
    }

    fn pick_random_direction(&self) -> Position {
        let mut rng = thread_rng();
        let n = rng.gen_range(0..4);
        match n {
            0 => (2, 0),
            1 => (0, 2),
            2 => (-2, 0),
            3 => (0, -2),
            _ => panic!("Invalid direction, this is weird and should not happen."),
        }
    }

    fn outside_grid(&self, point: Position) -> bool {
        return point.0 < 0 || point.0 > self.width - 1 || point.1 < 0 || point.1 > self.height - 1;
    }

    fn is_dead_end(&self, point: Position) -> bool {
        let up = (point.0, point.1 - 2);
        let down = (point.0, point.1 + 2);
        let left = (point.0 - 2, point.1);
        let right = (point.0 + 2, point.1);

        [up, down, left, right]
            .iter()
            .filter(|p| !self.outside_grid(**p))
            .all(|p| self.get_cell_type(*p) == CellType::Path)
    }

    fn backtrack(&mut self) {
        if let Some(pos) = self.stack.pop() {
            self.active_position = pos;
        } else {
            println!("No more cells to move to.");
        }
    }

    pub fn generate_next(&mut self) {
        let dir = self.pick_random_direction();
        let next_cell = (
            self.active_position.0 + dir.0,
            self.active_position.1 + dir.1,
        );

        if self.outside_grid(next_cell) {
            return;
        } else if self.is_dead_end(self.active_position) {
            self.backtrack();
        } else if self.get_cell_type(next_cell) == CellType::Wall {
            let cell_behind_next_cell = (next_cell.0 - dir.0 / 2, next_cell.1 - dir.1 / 2);
            self.set_cell_type(self.active_position, CellType::Path);
            self.set_cell_type(cell_behind_next_cell, CellType::Path);
            self.set_cell_type(next_cell, CellType::Path);

            self.stack.push(self.active_position);

            self.active_position = next_cell;
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
        if self.x > self.maze.width - 1 || self.y > self.maze.height - 1 {
            return None;
        }

        let return_value = Some(Cell {
            x: self.x,
            y: self.y,
            cell_type: self.maze.cells[self.x as usize][self.y as usize],
        });

        self.x += 1;
        if self.x > self.maze.width - 1 {
            self.x = 0;
            self.y += 1;
        }

        return_value
    }
}

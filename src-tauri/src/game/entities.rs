use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct GameOfLife {
    pub board: Board,
    pub is_playing: bool,
}

impl GameOfLife {
    pub fn new(rows: usize, cols: usize) -> Self {
        GameOfLife {
            board: Board::new(rows, cols),
            is_playing: false,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Board(Vec<Vec<Cell>>);

impl Board {
    pub fn new(rows: usize, cols: usize) -> Board {
        Board(vec![vec![Cell { alive: false }; cols]; rows])
    }

    pub fn toggle_cell(&mut self, coordinates: Coordinates) {
        if coordinates.y >= self.height() || coordinates.x >= self.width() {
            panic!("Out of bounds")
        }

        let alive = self.0[coordinates.y][coordinates.x].alive;
        self.0[coordinates.y][coordinates.x].alive = !alive;
    }

    pub fn tick(&mut self) {
        let mut next_gen = self.0.clone();

        for y in 0..self.height() {
            for x in 0..self.width() {
                let num_neighbors_alive = self.count_alive_neighbors(Coordinates { y, x });
                let is_alive = self.0[y][x].alive;

                next_gen[y][x].alive = match num_neighbors_alive {
                    2 => is_alive,
                    3 => true,
                    _ => false,
                }
            }
        }

        self.0 = next_gen;
    }

    fn height(&self) -> usize {
        self.0.len()
    }
    fn width(&self) -> usize {
        if self.height() == 0 {
            return 0;
        }
        return self.0[0].len();
    }

    fn count_alive_neighbors(&self, coordinates: Coordinates) -> usize {
        let mut count = 0;

        let prev_col_index = coordinates.y.saturating_sub(1);
        let next_col_index = (coordinates.y + 1).min(self.height() - 1);

        let prev_row_index = coordinates.x.saturating_sub(1);
        let next_row_index = (coordinates.x + 1).min(self.width() - 1);

        for col_index in prev_col_index..=next_col_index {
            for row_index in prev_row_index..=next_row_index {
                if (col_index, row_index) == (coordinates.y, coordinates.x) {
                    continue;
                }

                if self.0[col_index][row_index].alive {
                    count += 1;
                }
            }
        }

        return count;
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Cell {
    pub alive: bool,
}

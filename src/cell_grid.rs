use sfml::graphics::{
    Color, Drawable, RectangleShape, RenderStates, RenderTarget, 
    Shape, Transformable, 
};

#[derive(Clone)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Clone)]
pub struct Cell {
    pub state: CellState,
    pub pos: (f32, f32),
}

impl Cell {
    pub fn new(state: CellState, pos: (f32, f32)) -> Cell {
        Self {
            state,
            pos,
        }
    }
}


impl <'s> Drawable for Cell {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        render_target: &mut dyn RenderTarget,
        _: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        use CellState::*;
        let mut square = RectangleShape::with_size((10., 10.).into());
        let opacity: u8 = match self.state {
            Alive => 255,
            Dead => 0,
        };
        square.set_fill_color(Color::WHITE);
        square.set_position(self.pos);
        square.set_fill_color(Color::rgba(255, 255, 255, opacity));
        render_target.draw(&square)
    }
}

#[derive(Clone)]
pub struct CellGrid {
    pub grid: Vec<Vec<Box<Cell>>>,
    pub size: u64,
    pub alive: u64,
    pub dead: u64,
}

impl CellGrid {
    pub fn new(cols: i32, rows: i32) -> CellGrid {
        let mut grid: Vec<Vec<Box<Cell>>> = vec![];
        for i in 1..cols {
            let mut row: Vec<Box<Cell>> = vec![];
            for j in 1..rows {
                let pos = (15.0 * i as f32 - 5.0, 15.0 * j as f32 - 5.0);
                let state = CellState::Alive;
                let cell = Cell::new(state, pos);
                row.push(Box::new(cell));
            }
            grid.push(row);
        }

        let size = cols as u64 * rows as u64;

        Self {
            grid,
            size,
            alive: size,
            dead: 0,
        }
    }

    pub fn change_state_at_pos(&mut self, pos: (f32, f32), toggle: bool) {
        let mut index: Option<(usize, usize)> = None;

        for (i, rows) in self.grid.iter().enumerate() {
            for (j, cell) in rows.iter().enumerate() {
                let cell_pos = cell.as_ref().pos;

                let x_range = cell_pos.0-5.0..=cell_pos.0+5.0;
                let y_range = cell_pos.1-5.0..=cell_pos.1+5.0;

                if x_range.contains(&pos.0) && y_range.contains(&pos.1) {
                    index = Some((i, j));
                }
            }
        }

        if let Some(idx) = index {
            use CellState::*;

            let cell = self.grid[idx.0][idx.1].as_ref();

            match cell.state {
                Alive => {
                    &mut self.change_cell_state(idx, Dead);
                },
                Dead => {
                    if toggle {
                        &mut self.change_cell_state(idx, Alive);
                    }
                }
            }
        }

    }

    fn change_cell_state(&mut self, index: (usize, usize), to: CellState) {
        use CellState::*;

        self.grid[index.0][index.1].as_mut().state = match to {
            Alive => {
                self.alive += 1;
                self.dead -= 1;
                Alive
            },
            Dead => {
                self.alive -= 1;
                self.dead += 1;
                Dead
            }
        }
    }
}

impl <'s> Drawable for CellGrid {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        render_target: &mut dyn RenderTarget,
        _: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        for rows in &self.grid {
            for cell in rows {
                render_target.draw(cell.as_ref());
            }
        }
    }
}
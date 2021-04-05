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
    state: CellState,
    pos: (f32, f32),
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
    pub grid: Vec<Vec<Box<Cell>>>
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

        Self {
            grid,
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
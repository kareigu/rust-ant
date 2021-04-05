use sfml::graphics::{
  Color, Drawable, RectangleShape, RenderStates, RenderTarget, 
  Shape, Transformable, 
};

pub enum CellState {
  Alive,
  Dead,
}

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
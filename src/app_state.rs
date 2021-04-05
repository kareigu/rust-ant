use std::time::Instant;
use sfml::graphics::{
  RenderWindow, Font, Color, Drawable, Text, Transformable, RenderTarget,
  RectangleShape, Shape,
};


pub struct AppState<'s> {
  pub delta_time: f64,
  pub time_elapsed: f64,
  pub fps: f32,
  font: &'s Font,
  pub window: &'s mut RenderWindow,
  pub bg_color: Color,
  prev_update: Instant,
  render_queue: Vec<Box<dyn Drawable + 's>>,
  pub debug_stats: bool,
  vsync: bool,
}

impl <'s> AppState<'s> {
  pub fn new(font: &'s Font, window: &'s mut RenderWindow, vsync: bool) -> AppState<'s> {
      window.set_vertical_sync_enabled(vsync);
      Self {
          delta_time: 0.0,
          time_elapsed: 0.0,
          fps: 0.0,
          font,
          window,
          bg_color: Color::BLACK,
          prev_update: Instant::now(),
          render_queue: vec![],
          debug_stats: false,
          vsync,
      }
  }

  pub fn debug_text(&self, text: String, pos: (f32, f32)) -> Text<'s> {
      let mut debug_text = Text::new(
          text.as_str(), 
          &self.font, 
          24);
      debug_text.set_position(pos);
      debug_text.set_fill_color(Color::MAGENTA);
      debug_text.set_outline_color(Color::BLACK);
      debug_text.set_outline_thickness(2.5);
      debug_text
  }

  pub fn toggle_vsync(&mut self) {
      self.vsync = !self.vsync;
      self.window.set_vertical_sync_enabled(self.vsync);
  }

  pub fn run_update(&mut self) {
      self.delta_time = self.prev_update.elapsed().as_secs_f64();
      self.prev_update = Instant::now();
      self.fps = 1.0 / self.delta_time as f32;
      self.time_elapsed += self.delta_time;
  }

  fn window_clear(&mut self) {
      self.window.clear(self.bg_color);
  }

  pub fn draw_square(&mut self, pos: (f32, f32), rgba: (u8, u8, u8, u8)) {
      let mut square = RectangleShape::with_size((10., 10.).into());
      square.set_fill_color(Color::WHITE);
      square.set_position(pos);
      square.set_fill_color(Color::rgba(rgba.0, rgba.1, rgba.2, rgba.3));
      self.push_to_render_queue(Box::new(square));
  }

  pub fn render(&mut self) {
      self.window_clear();

      if self.debug_stats {
          self.debug_stats();
      }
      for drawable in &self.render_queue {
          self.window.draw(drawable.as_ref());
      }

      self.window.display();
      self.render_queue = vec![];
  }
  

  pub fn push_to_render_queue(&mut self, drawable: Box<dyn Drawable + 's>) {
      self.render_queue.push(drawable);
  }

  fn debug_stats(&mut self) {
      let delta = self.debug_text(format!("{} seconds", self.delta_time), (10.0, 10.0));
      let fps = self.debug_text(format!("{:.1} fps", self.fps), (10.0, 35.0));
      let vsync = self.debug_text(format!("Vsync {}", self.vsync), (10.0, 60.0));
      self.push_to_render_queue(Box::new(delta));
      self.push_to_render_queue(Box::new(fps));
      self.push_to_render_queue(Box::new(vsync));
  }
}
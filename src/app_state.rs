use sfml::graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable};
use std::time::Instant;

use crate::cell_grid::{Ant, CellGrid};

pub struct AppState<'s> {
  pub delta_time: f64,
  pub time_elapsed: f64,
  pub fps: f32,
  font: &'s Font,
  pub bg_color: Color,
  prev_update: Instant,
  pub debug_stats: bool,
  vsync: bool,
  pub cell_grid: Box<CellGrid>,
  pub ant: Box<Ant<'s>>,
  steps: u64,
  pub running: bool,
}

impl<'s> AppState<'s> {
  pub fn new(
    font: &'s Font,
    cell_grid: Box<CellGrid>,
    vsync: bool,
    ant: Box<Ant<'s>>,
  ) -> AppState<'s> {
    Self {
      delta_time: 0.0,
      time_elapsed: 0.0,
      fps: 0.0,
      font,
      bg_color: Color::BLACK,
      prev_update: Instant::now(),
      debug_stats: false,
      vsync,
      cell_grid,
      ant,
      steps: 0,
      running: false,
    }
  }

  pub fn debug_text(&self, text: String, pos: (f32, f32)) -> Text<'s> {
    let mut debug_text = Text::new(text.as_str(), self.font, 24);
    debug_text.set_position(pos);
    debug_text.set_fill_color(Color::MAGENTA);
    debug_text.set_outline_color(Color::BLACK);
    debug_text.set_outline_thickness(2.5);
    debug_text
  }

  pub fn ui_text(&self, text: String, pos: (f32, f32), size: u32) -> Text<'s> {
    let mut ui_text = Text::new(text.as_str(), self.font, size);
    ui_text.set_position(pos);
    ui_text.set_fill_color(Color::WHITE);
    ui_text.set_outline_color(Color::BLACK);
    ui_text.set_outline_thickness(1.5);
    ui_text
  }

  pub fn toggle_vsync(&mut self, window: &mut RenderWindow) {
    self.vsync = !self.vsync;
    window.set_vertical_sync_enabled(self.vsync);
  }

  pub fn run_update(&mut self) {
    self.delta_time = self.prev_update.elapsed().as_secs_f64();
    self.fps = 1.0 / self.delta_time as f32;
    self.time_elapsed += self.delta_time;

    if self.running {
      let ant_pos = self.ant.pos;

      let cell_state = self.cell_grid.change_state_at_pos(ant_pos, true);

      self.ant.handle_move(cell_state);
      self.steps += 1;
    }

    self.prev_update = Instant::now();
  }

  pub fn render(&mut self, window: &mut RenderWindow) {
    window.clear(self.bg_color);

    let grid = self.cell_grid.as_ref();
    window.draw(grid);

    let ant = self.ant.as_ref();
    window.draw(ant);

    if self.debug_stats {
      self.debug_stats(window);
    } else {
      let debug_prompt_text = self.ui_text("F1".to_string(), (5.0, 5.0), 30);
      window.draw(&debug_prompt_text);
    }

    if !self.running {
      let pause_text = self.ui_text("PAUSED".to_string(), (540.0, 850.0), 64);
      let unpause_info_text =
        self.ui_text("Press spacebar to unpause".to_string(), (460.0, 910.0), 32);

      window.draw(&pause_text);
      window.draw(&unpause_info_text);
    }

    window.display();
  }

  fn debug_stats(&mut self, window: &mut RenderWindow) {
    let mut debug_texts: Vec<sfml::graphics::Text<'_>> = vec![];
    let delta = self.debug_text(format!("{} seconds", self.delta_time), (10.0, 10.0));
    let fps = self.debug_text(format!("{:.1} fps", self.fps), (10.0, 35.0));
    let vsync = self.debug_text(
      format!("Vsync {}", if self.vsync { "disabled" } else { "enabled" }),
      (10.0, 60.0),
    );

    debug_texts.push(delta);
    debug_texts.push(fps);
    debug_texts.push(vsync);

    match procinfo::pid::stat_self() {
      Ok(m) => {
        let vmem = self.debug_text(format!("vmem: {} MB", m.vsize as f64 / 1e+6), (10.0, 85.0));
        let pid = self.debug_text(format!("pid: {}", m.pid), (10.0, 110.0));
        debug_texts.push(vmem);
        debug_texts.push(pid);
      }
      Err(e) => {
        let proc_error = self.debug_text(format!("{:?}", e), (10.0, 85.0));
        debug_texts.push(proc_error);
      }
    }

    let cell_count = self.debug_text(format!("{} cells", self.cell_grid.size), (10.0, 185.0));
    let alive_count = self.debug_text(format!("{} alive", self.cell_grid.alive), (10.0, 210.0));
    let dead_count = self.debug_text(format!("{} dead", self.cell_grid.dead), (10.0, 235.0));
    debug_texts.push(cell_count);
    debug_texts.push(alive_count);
    debug_texts.push(dead_count);

    let steps_count = self.debug_text(format!("{} steps", self.steps), (10.0, 265.0));
    debug_texts.push(steps_count);

    let vsync_toggle_info = self.ui_text("F2 to toggle vsync".to_string(), (10.0, 920.0), 30);
    debug_texts.push(vsync_toggle_info);

    for text in debug_texts {
      window.draw(&text);
    }
  }
}

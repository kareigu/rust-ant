use sfml::{
  graphics::{Font, RenderWindow},
  window::{mouse, Event, Key, Style},
};

mod app_state;
use app_state::AppState;

mod cell_grid;
use cell_grid::{Ant, CellGrid};

fn main() {
  let mut window = RenderWindow::new(
    (1280, 960),
    "Rusty Langton's Ant",
    Style::DEFAULT,
    &Default::default(),
  );

  let font = match Font::from_file("assets/FiraSans-Regular.ttf") {
    Some(f) => f,
    None => {
      println!("Couldn't load font from the assets folder");
      std::process::exit(1);
    }
  };
  let cell_grid = Box::new(CellGrid::new(85, 64));
  let ant = Box::new(Ant::new((200.0, 200.0), &font));

  let vsync = true;
  let mut app_state = AppState::new(&font, cell_grid, vsync, ant);
  window.set_vertical_sync_enabled(vsync);

  loop {
    while let Some(event) = window.poll_event() {
      match event {
        Event::Closed
        | Event::KeyPressed {
          code: Key::Escape, ..
        } => return,
        Event::KeyPressed { code: Key::F1, .. } => app_state.debug_stats = !app_state.debug_stats,
        Event::KeyPressed { code: Key::F2, .. } => app_state.toggle_vsync(&mut window),
        Event::KeyPressed {
          code: Key::Space, ..
        } => app_state.running = !app_state.running,
        Event::MouseButtonPressed {
          button: mouse::Button::Left,
          x,
          y,
        } => {
          app_state.ant.set_pos((x as f32, y as f32));
        }
        Event::MouseButtonPressed {
          button: mouse::Button::Right,
          x,
          y,
        } => {
          app_state
            .cell_grid
            .change_state_at_pos((x as f32, y as f32 + 1.0), true);
        }
        _ => {}
      }
    }

    app_state.run_update();
    app_state.render(&mut window);
  }
}

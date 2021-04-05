use sfml::{
    graphics::{
        RenderWindow, Font
    },
    window::{Event, Key, Style},
};

mod app_state;
use app_state::{AppState, RenderQueueObject};

mod cell_grid;
use cell_grid::{Cell, CellState, CellGrid};

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "SFML test",
        Style::CLOSE,
        &Default::default(),
    );

    let font = Font::from_file("assets/FiraSans-Regular.ttf").expect("Couldn't find font file");
    let mut cell_grid = CellGrid::new(58, 40);
    let mut app_state = AppState::new(&font, &mut window, &mut cell_grid, true);


    loop {
        while let Some(event) = app_state.window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::ESCAPE, ..
                } => return,
                Event::KeyPressed {
                    code: Key::F1, ..
                } => app_state.debug_stats = !app_state.debug_stats,
                Event::KeyPressed {
                    code: Key::F2, ..
                } => app_state.toggle_vsync(),
                _ => {}
            }
        }
        

        app_state.run_update();

        app_state.render();
    }
}
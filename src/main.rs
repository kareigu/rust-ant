use sfml::{
    graphics::{
        RenderWindow, Font
    },
    window::{Event, Key, Style},
};

mod app_state;
use app_state::AppState;

mod cell_grid;
use cell_grid::{Cell, CellState};

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "SFML test",
        Style::CLOSE,
        &Default::default(),
    );

    let font = Font::from_file("assets/FiraSans-Regular.ttf").expect("Couldn't find font file");
    let mut app_state = AppState::new(&font, &mut window, true);


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

        for i in 1..53 {
            for j in 1..40 {
                let sin = (f32::sin(app_state.time_elapsed as f32 + i as f32 / 53.0)).powi(2);
                /*let opacity: u8 = (sin * 255.0) as u8;
                app_state.draw_square(
                (15.0 * i as f32 - 5.0, 15.0 * j as f32 - 5.0), 
                (255, 255, 255, opacity)); */
                let pos = (15.0 * i as f32 - 5.0, 15.0 * j as f32 - 5.0);
                let state = if sin > 0.5 { CellState::Alive } else { CellState::Dead };
                let cell = Cell::new(state, pos);
                app_state.push_to_render_queue(Box::new(cell));
            }
        }
        app_state.render();
    }
}
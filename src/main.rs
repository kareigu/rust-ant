use sfml::{
    graphics::{
        RenderWindow, Font
    },
    window::{Event, Key, Style, mouse},
};

mod app_state;
use app_state::{AppState};

mod cell_grid;
use cell_grid::{CellGrid, Ant};

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
    let mut app_state = AppState::new(
        &font, 
        &mut window, 
        cell_grid, 
        true,
        ant
    );

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
                Event::KeyPressed {
                    code: Key::SPACE, ..
                } => app_state.running = !app_state.running,
                Event::MouseButtonPressed {
                    button: mouse::Button::LEFT,
                    x, y
                } => {
                    app_state.ant.set_pos((x as f32, y as f32));
                },
                Event::MouseButtonPressed {
                    button: mouse::Button::RIGHT,
                    x, y
                } => {app_state.cell_grid.change_state_at_pos((x as f32, y as f32 + 1.0), true);},
                _ => {}
            }
        }
        

        app_state.run_update();
        app_state.render();
    }
}
use sfml::{
    graphics::{
        RenderWindow, Font
    },
    window::{Event, Key, Style, mouse},
};

mod app_state;
use app_state::{AppState};

mod cell_grid;
use cell_grid::{CellGrid};

fn main() {
    let mut window = RenderWindow::new(
        (1280, 960),
        "SFML test",
        Style::DEFAULT,
        &Default::default(),
    );

    let font = Font::from_file("assets/FiraSans-Regular.ttf").expect("Couldn't find font file");
    let mut cell_grid = CellGrid::new(85, 64);
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
                Event::MouseButtonPressed {
                    button: mouse::Button::LEFT,
                    x, y
                } => {
                    println!("x: {} y:{}", x, y); 
                    app_state.cell_grid.change_state_at_pos((x as f32, y as f32), false);
                },
                Event::MouseButtonPressed {
                    button: mouse::Button::RIGHT,
                    x, y
                } => app_state.cell_grid.change_state_at_pos((x as f32, y as f32), true),
                _ => {}
            }
        }
        

        app_state.run_update();

        app_state.render();
    }
}
use sfml::{
    graphics::{
        CircleShape, Color, Drawable, RectangleShape, RenderStates, RenderTarget, RenderWindow,
        Shape, Transformable, Text, Font
    },
    window::{Event, Key, Style},
    SfBox
};

mod app_state;
use app_state::AppState;

/// Our custom drawable type. It looks like a bullet.
struct Bullet<'s> {
    head: CircleShape<'s>,
    torso: RectangleShape<'s>,
}

impl<'s> Bullet<'s> {
    pub fn new() -> Self {
        let mut head = CircleShape::new(50.0, 50);
        head.set_position((100.0, 100.0));
        head.set_fill_color(Color::RED);
        let mut torso = RectangleShape::with_size((100., 200.).into());
        torso.set_position((100.0, 150.0));
        torso.set_fill_color(Color::BLUE);

        Self { head, torso }
    }
}

// Implement the Drawable trait for our custom drawable.
impl<'s> Drawable for Bullet<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        render_target: &mut dyn RenderTarget,
        _: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        render_target.draw(&self.head);
        render_target.draw(&self.torso)
    }
}

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
        //app_state.push_to_render_queue(Box::new(Bullet::new()));

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
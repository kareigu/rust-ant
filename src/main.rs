use sfml::{
    graphics::{
        CircleShape, Color, Drawable, RectangleShape, RenderStates, RenderTarget, RenderWindow,
        Shape, Transformable, Text, Font
    },
    window::{Event, Key, Style},
    SfBox
};

use std::time::Instant;

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

pub struct AppState<'s> {
    pub delta_time: f64,
    pub fps: f32,
    pub font: &'s Font,
    pub bg_color: Color,
    prev_update: Instant,
}

impl <'s> AppState<'s> {
    pub fn new(font: &'s Font) -> Self {
        Self {
            delta_time: 0.0,
            fps: 0.0,
            font,
            bg_color: Color::BLACK,
            prev_update: Instant::now(),
        }
    }

    pub fn delta_time_text(&self, pos: (f32, f32)) -> Text<'s> {
        let mut delta_time_text = Text::new(
            format!("{} seconds", self.delta_time).as_str(), 
            &self.font, 
            24);
        delta_time_text.set_position(pos);
        delta_time_text
    }

    pub fn fps_text(&self, pos: (f32, f32)) -> Text<'s> {
        let mut fps_text = Text::new(
            format!("{:.1} fps", self.fps).as_str(), 
            &self.font, 
            24);
        fps_text.set_position(pos);
        fps_text
    }

    pub fn run_update(&mut self) {
        self.delta_time = self.prev_update.elapsed().as_secs_f64();
        self.prev_update = Instant::now();
        self.fps = 1.0 / self.delta_time as f32;
    }
}

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "SFML test",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let bullet = Bullet::new();

    let font = Font::from_file("assets/FiraSans-Regular.ttf").expect("Couldn't find font file");
    let mut app_state = AppState::new(&font);


    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::ESCAPE, ..
                } => return,
                _ => {}
            }
        }

        app_state.run_update();
        
        window.clear(app_state.bg_color);
        window.draw(&bullet);
        window.draw(&app_state.delta_time_text((10.0, 10.0)));
        window.draw(&app_state.fps_text((10.0, 35.0)));
        window.display()
    }
}
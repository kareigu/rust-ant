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
    font: &'s Font,
    window: &'s mut RenderWindow,
    pub bg_color: Color,
    prev_update: Instant,
    render_queue: Vec<Box<dyn Drawable + 's>>,
    debug_stats: bool,
    vsync: bool,
}

impl <'s> AppState<'s> {
    pub fn new(font: &'s Font, window: &'s mut RenderWindow, vsync: bool) -> AppState<'s> {
        window.set_vertical_sync_enabled(vsync);
        Self {
            delta_time: 0.0,
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

    pub fn delta_time_text(&self, pos: (f32, f32)) -> Text<'s> {
        let mut delta_time_text = Text::new(
            format!("{} seconds", self.delta_time).as_str(), 
            &self.font, 
            24);
        delta_time_text.set_position(pos);
        delta_time_text
    }

    pub fn vsync_text(&self, pos: (f32, f32)) -> Text<'s> {
        let mut vsync_text = Text::new(
            format!("Vsync {}", self.vsync).as_str(), 
            &self.font, 
            24);
        vsync_text.set_position(pos);
        vsync_text
    }

    pub fn fps_text(&self, pos: (f32, f32)) -> Text<'s> {
        let mut fps_text = Text::new(
            format!("{:.1} fps", self.fps).as_str(), 
            &self.font, 
            24);
        fps_text.set_position(pos);
        fps_text
    }

    pub fn toggle_vsync(&mut self) {
        self.vsync = !self.vsync;
        self.window.set_vertical_sync_enabled(self.vsync);
    }

    pub fn run_update(&mut self) {
        self.delta_time = self.prev_update.elapsed().as_secs_f64();
        self.prev_update = Instant::now();
        self.fps = 1.0 / self.delta_time as f32;
    }

    fn window_clear(&mut self) {
        self.window.clear(self.bg_color);
    }

    pub fn render(&mut self) {
        if self.debug_stats {
            self.debug_stats();
        }

        self.window_clear();
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
        let delta = self.delta_time_text((10.0, 10.0));
        let fps = self.fps_text((10.0, 35.0));
        let vsync = self.vsync_text((10.0, 60.0));
        self.push_to_render_queue(Box::new(delta));
        self.push_to_render_queue(Box::new(fps));
        self.push_to_render_queue(Box::new(vsync));
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
        app_state.push_to_render_queue(Box::new(Bullet::new()));
        app_state.render();
    }
}
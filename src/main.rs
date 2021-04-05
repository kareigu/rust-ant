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

pub struct AppState<'s> {
    pub delta_time: f64,
    pub time_elapsed: f64,
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
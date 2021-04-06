use std::time::Instant;
use sfml::graphics::{
    RenderWindow, Font, Color, Drawable, Text, Transformable, RenderTarget,
    RectangleShape, Shape,
};

use crate::cell_grid::{CellGrid, Ant};

pub enum RenderQueueObject<'s> {
    Box(Box<dyn Drawable + 's>),
    Ref(&'s dyn Drawable),
}

pub struct AppState<'s> {
    pub delta_time: f64,
    pub time_elapsed: f64,
    pub fps: f32,
    font: &'s Font,
    pub window: &'s mut RenderWindow,
    pub bg_color: Color,
    prev_update: Instant,
    render_queue: Vec<RenderQueueObject<'s>>,
    pub debug_stats: bool,
    vsync: bool,
    pub cell_grid: Box<CellGrid>,
    pub ant: Box<Ant<'s>>,
    steps: u64,
}

impl <'s> AppState<'s> {
    pub fn new(
        font: &'s Font, 
        window: &'s mut RenderWindow, 
        cell_grid: Box<CellGrid>, 
        vsync: bool,
        ant: Box<Ant<'s>>,
    ) -> AppState<'s> {
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
            cell_grid,
            ant,
            steps: 0,
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
        self.fps = 1.0 / self.delta_time as f32;
        self.time_elapsed += self.delta_time;


        let ant_pos = self.ant.pos;

        let cell_state = self.cell_grid.change_state_at_pos(ant_pos, true);

        self.ant.handle_move(cell_state);
        self.steps += 1;

        self.prev_update = Instant::now();
    }

    fn window_clear(&mut self) {
        self.window.clear(self.bg_color);
    }

    pub fn draw_square(&mut self, pos: (f32, f32), rgba: (u8, u8, u8, u8)) {
        let mut square = RectangleShape::with_size((10., 10.).into());
        square.set_fill_color(Color::WHITE);
        square.set_position(pos);
        square.set_fill_color(Color::rgba(rgba.0, rgba.1, rgba.2, rgba.3));
        let render = Box::new(square);
        self.push_to_render_queue(RenderQueueObject::Box(render));
    }

    pub fn render(&mut self) {
        self.window_clear();

        let grid = self.cell_grid.as_ref();
        self.window.draw(grid);

        let ant = self.ant.as_ref();
        self.window.draw(ant);

        if self.debug_stats {
            self.debug_stats();
        }

        /* for drawable in &self.render_queue {
            use RenderQueueObject::*;
            match drawable {
                Box(d) => self.window.draw(d.as_ref()),
                Ref(d) => self.window.draw(*d),
            }

        } */

        self.window.display();
        //self.render_queue = vec![];
    }


    pub fn push_to_render_queue(&mut self, drawable: RenderQueueObject<'s> ) {
        self.render_queue.push(drawable);
    }

    fn debug_stats(&mut self) {
        let mut debug_texts: Vec<sfml::graphics::Text<'_>> = vec![];
        let delta = self.debug_text(format!("{} seconds", self.delta_time), (10.0, 10.0));
        let fps = self.debug_text(format!("{:.1} fps", self.fps), (10.0, 35.0));
        let vsync = self.debug_text(format!("Vsync {}", self.vsync), (10.0, 60.0));

        debug_texts.push(delta);
        debug_texts.push(fps);
        debug_texts.push(vsync);

        match procinfo::pid::stat_self() {
            Ok(m) => {
                let vmem = self.debug_text(format!("vmem: {} MB", m.vsize as f64 / 1e+6), (10.0, 85.0));
                let pid = self.debug_text(format!("pid: {}", m.pid), (10.0, 110.0));
                debug_texts.push(vmem);
                debug_texts.push(pid);
            },
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

        for text in debug_texts {
            self.window.draw(&text);
        }
    }
}
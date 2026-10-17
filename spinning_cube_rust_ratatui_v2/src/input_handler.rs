use crate::display_manager::Panes;
use crate::vec3::Vec3;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use enum_cycling::EnumCycle;
use std::time::{Duration, Instant};

const MAX_PROCESSED_EVENT: u16 = 10;
const MAX_SPIN_SPEED: f32 = 15.0;

pub struct InputHandler {
    pub should_close: bool,
    pub clicked_x: u16,
    pub clicked_y: u16,
    pub drag_dir: Vec3,
    pub is_dragging: bool,
    pub time_since_drag: Instant,
    pub selected_pane: Panes,
    drag_prev_x: u16,
    drag_prev_y: u16,
}

impl InputHandler {
    pub fn default() -> Self {
        Self {
            should_close: false,
            clicked_x: 0,
            clicked_y: 0,
            is_dragging: false,
            selected_pane: Panes::Renderer,
            drag_dir: Vec3::default(),
            time_since_drag: Instant::now(),
            drag_prev_x: 0,
            drag_prev_y: 0,
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind == event::KeyEventKind::Press {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => self.should_close = true,
                KeyCode::Char('a') => self.selected_pane = self.selected_pane.up(),
                KeyCode::Char('d') => self.selected_pane = self.selected_pane.down(),
                _ => {}
            }
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        match mouse_event.kind {
            MouseEventKind::Drag(MouseButton::Left) => self.update_drag(mouse_event),
            MouseEventKind::Down(MouseButton::Left) => {
                self.clicked_x = mouse_event.column;
                self.clicked_y = mouse_event.row;
                self.update_drag(mouse_event)
            }
            MouseEventKind::Up(MouseButton::Left) => self.reset_drag(),
            _ => {}
        }
    }

    fn update_drag(&mut self, mouse_event: MouseEvent) {
        if self.drag_prev_x != 0 && self.drag_prev_y != 0 {
            self.is_dragging = true;
            let mut dx = mouse_event.column as f32 - self.drag_prev_x as f32;
            let mut dy = mouse_event.row as f32 - self.drag_prev_y as f32;

            dx = dx.clamp(-MAX_SPIN_SPEED, MAX_SPIN_SPEED);
            dy = dy.clamp(-MAX_SPIN_SPEED, MAX_SPIN_SPEED);

            self.drag_dir = Vec3::default_vec(dy, -dx, 0.0);
        }

        self.drag_prev_x = mouse_event.column;
        self.drag_prev_y = mouse_event.row;
    }

    fn reset_drag(&mut self) {
        self.time_since_drag = Instant::now();
        self.is_dragging = false;
        self.drag_prev_x = 0;
        self.drag_prev_y = 0;
        self.drag_dir = Vec3::default();
    }

    pub fn get_input(&mut self) -> Result<()> {
        let mut processed = 0;

        while processed < MAX_PROCESSED_EVENT && event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(key_event) => self.handle_key_event(key_event),
                Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
                _ => {}
            }

            processed += 1;
        }
        Ok(())
    }
}

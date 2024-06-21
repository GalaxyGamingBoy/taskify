use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::actions::{Action, Event};
use crate::keybindings::{Keybinding, Keybindings};
use crate::states::{AppState, RenderState};

#[derive(Debug, Default)]
pub struct Project {}

impl AppState for Project {
    fn display_name(&self) -> &str {
        "PROJECTS"
    }

    fn action(&mut self, action: Action) -> Event {
        match action {
            _ => Event::None
        }
    }

    fn tick(&mut self) {}
}

impl Keybindings for Project {
    fn keybindings(&self) -> Vec<Keybinding> {
        vec![]
    }
}

impl RenderState for Project {
    fn render(&self, area: Rect, buf: &mut Buffer) {}
}
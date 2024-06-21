use std::rc::Rc;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::actions::{Action, Event};
use crate::app::App;
use crate::keybindings::{Keybinding, Keybindings};
use crate::states::{AppState, RenderState};

#[derive(Debug, Default, Clone)]
pub struct Project {}

impl AppState for Project {
    fn display_name(&self) -> &str {
        "PROJECTS"
    }

    fn action(&mut self, action: Action) {
        match action {
            _ => {}
        }
    }

    fn tick(&mut self) -> Rc<Event> {
        Rc::new(Event::None)
    }
}

impl Keybindings for Project {
    fn keybindings(&self) -> Vec<Keybinding> {
        todo!()
    }
}

impl RenderState for Project {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}
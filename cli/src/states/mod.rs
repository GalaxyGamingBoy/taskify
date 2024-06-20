//! This file contains the app states and render methods

use std::fmt::Debug;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::actions::Action;
use crate::keybindings::Keybindings;

pub mod home;

#[allow(unused_variables)]
pub trait RenderState {
    fn render(&self, area: Rect, buf: &mut Buffer) {}
}

#[allow(unused_variables)]
pub trait AppState: Keybindings + RenderState + Debug {
    fn display_name(&self) -> &str { "" }
    fn action(&mut self, action: Action) {}
}

#![allow(unused_variables)]

use std::fmt::Debug;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::actions::Action;
use crate::keybindings::Keybindings;

pub mod home;

pub trait RenderState {
    fn render(&self, area: Rect, buf: &mut Buffer) {}
}

pub trait AppState: Keybindings + RenderState + Debug {
    fn action(&mut self, action: Action) {}
}

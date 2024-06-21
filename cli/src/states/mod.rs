//! This file contains the app states and render methods

use std::fmt::Debug;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::actions::{Action, Event};
use crate::keybindings::Keybindings;

pub mod home;
pub mod project;

#[allow(unused_variables)]
pub trait RenderState {
    fn render(&self, area: Rect, buf: &mut Buffer) {}
}

#[allow(unused_variables)]
pub trait AppState: Keybindings + RenderState + Debug {
    fn display_name(&self) -> &str { "APP_STATE" }
    fn action(&mut self, action: Action) -> Event { Event::None }
    fn tick(&mut self) { }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum AppStates {
    #[default]
    Home,
    Project
}
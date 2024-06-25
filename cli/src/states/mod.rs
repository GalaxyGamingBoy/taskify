//! This file contains the app states and render methods

use crate::actions::{Action, Event};
use crate::keybindings::Keybindings;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use sqlx::SqliteConnection;
use std::fmt::Debug;

pub mod home;
pub mod project;

#[allow(unused_variables)]
pub trait RenderState {
    fn render(&self, area: Rect, buf: &mut Buffer) {}
}

#[allow(unused_variables)]
pub trait AppState: Keybindings + RenderState + Debug {
    fn display_name(&self) -> &str {
        "APP_STATE"
    }

    async fn init(&mut self, exec: &SqliteConnection) {}

    async fn tick(&mut self, exec: &SqliteConnection) {}

    fn action(&mut self, action: Action) -> Event {
        Event::None
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum AppStates {
    #[default]
    Home,
    Project,
}

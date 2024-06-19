use ratatui::widgets::Widget;
use crate::app::App;
use crate::states::{AppState};

pub trait Keybindings {
    fn keybindings() -> Vec<Box<Keybinding>> { vec![] }
}

pub struct Keybinding {
    pub key: char,
    pub name: String,
    // pub action: Fn()
}
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use crate::keybindings::Keybinding;
use crate::states::home::Home;

pub mod home;

pub trait Keybindings {
    fn keybindings() -> Vec<Keybinding>;
}

pub trait State {}

#[derive(Default, Debug)]
pub enum AppStates<T> where T: State + Widget {
    #[default]
    Home(T)
}

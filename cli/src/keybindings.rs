use crate::actions::Action;

pub trait Keybindings {
    fn keybindings(&self) -> Vec<Keybinding> { vec![] }
}

pub struct Keybinding {
    pub key: char,
    pub name: String,
    pub action: Action
}
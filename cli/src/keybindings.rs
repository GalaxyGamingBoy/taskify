use crate::app::App;

pub struct Keybinding {
    pub key: char,
    pub name: String,
    pub action: dyn Fn(&App)
}
use crate::states::AppState;

#[derive(Debug, Copy, Clone, Default)]
pub enum Action {
    Home_GotoProjects,
    Escape,
    #[default]
    None
}

#[derive(Debug, Default)]
pub enum Event {
    #[default]
    None,
    Goto(dyn AppState)
}
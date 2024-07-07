use crate::states::AppStates;

#[derive(Debug, Copy, Clone, Default)]
pub enum Action {
    #[default]
    None,
    Escape,
    HomeGotoProjects,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum Event {
    #[default]
    None,
    Goto(AppStates)
}
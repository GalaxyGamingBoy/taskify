
use ratatui::widgets::Widget;
use crate::states::home::Home;

pub mod home;

pub trait AppState {}

#[derive(Debug, Clone)]
pub enum AppStates {
    Home(Home)
}
//! This file contains all of the handlers, i.e. key presses

use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::Widget;
use crate::keybindings::Keybindings;
use crate::states::AppState;

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        _ => {}
    }
    Ok(())
}

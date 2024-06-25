use crate::actions::{Action, Event};
use crate::keybindings::{Keybinding, Keybindings};
use crate::states::{AppState, RenderState};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use sqlx::SqliteConnection;
use taskify::db::projects::Project as DBProject;

#[derive(Debug, Default)]
pub struct Project {
    projects: Vec<DBProject>,
    db_page: u64,
}

impl AppState for Project {
    fn display_name(&self) -> &str {
        "PROJECTS"
    }

    async fn init(&mut self, exec: &SqliteConnection) {
        self.projects = DBProject::from_list_db(self.db_page, 12, exec)
            .await
            .unwrap()
    }

    async fn tick(&mut self, exec: &SqliteConnection) {}

    fn action(&mut self, action: Action) -> Event {
        match action {
            _ => Event::None,
        }
    }
}

impl Keybindings for Project {
    fn keybindings(&self) -> Vec<Keybinding> {
        vec![]
    }
}

impl RenderState for Project {
    fn render(&self, area: Rect, buf: &mut Buffer) {}
}

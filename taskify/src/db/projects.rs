//! Project Database Entity
//! This file contains the database entity for taskify.

use chrono::{DateTime, Utc};
use sea_query::{enum_def, Query, SqliteQueryBuilder};
use sea_query_binder::{SqlxBinder, SqlxValues};
use uuid::Uuid;

/// The database entity for taskify
#[enum_def]
#[derive(Debug, Default, Clone)]
pub struct Project {
    id: Uuid,
    name: String,
    description: String,
    author: String,
    created: DateTime<Utc>,
    modified: DateTime<Utc>
}

impl Project {
    /// New Project
    ///
    /// Create a new project entity with a default id, created and edited.
    /// # Arguments:
    /// * `name` - The project name
    /// * `description` - The project description
    pub fn new(name: String, description: String, author: String) -> Self {
        Self {name, description, author, ..Default::default()}
    }

    /// ID Assignment
    ///
    /// Assigns a random uuid v4 id to the project.
    pub fn assign_id(&mut self) -> &mut Self {
        self.id = Uuid::new_v4();
        self
    }

    /// Created Assignment
    ///
    /// Assigns the project `created` and `modified` field to the current datetime.
    pub fn assign_created(&mut self) -> &mut Self {
        self.created = Utc::now();
        self.modified = self.created;
        self
    }

    /// Set Id
    ///
    /// Sets the project id
    /// # Arguments:
    /// * `id` - The uuid v4 id to use
    pub fn set_id(&mut self, id: Uuid) -> &mut Self {
        self.id = id;
        self
    }

    /// Set Name
    ///
    /// Sets the project display name
    /// # Arguments
    /// * `name` - The string display name to use
    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.edited();
    }

    /// Set Description
    ///
    /// Sets the project description
    /// # Arguments
    /// * `description` - The strign description to use
    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.edited();
    }

    /// Set Edited
    ///
    /// Sets the project edited field to the current datetime
    fn edited(&mut self) {
        self.modified = Utc::now();
    }

    // Database Interactions

    /// Insert Project to DB
    ///
    /// # Examples
    /// ```
    /// # #[tokio::test]
    /// # async fn test() -> Result<(), Box<dyn std::error::Error>> {
    /// taskify::db::projects::Project::new("Name".into(), "Desc".into()).insert();
    /// # Ok(())
    /// # }
    /// ```
    pub fn insert(&self) -> (String, SqlxValues) {
        Query::insert()
            .into_table(ProjectIden::Table)
            .columns([ProjectIden::Id, ProjectIden::Name, ProjectIden::Description, ProjectIden::Author, ProjectIden::Created, ProjectIden::Modified])
            .values([self.id.into(), self.name.clone().into(), self.description.clone().into(), self.author.clone().into(), self.created.into(), self.modified.into()])
            .unwrap().build_sqlx(SqliteQueryBuilder)
    }

    /// Update Project on DB
    pub fn update(&mut self) {}

    /// Delete Project on DB
    pub fn delete(&mut self) {}

    /// Find a Project on DB
    ///
    /// Finds a project in the DB by providing the id (uuid) value.
    /// # Arguments
    /// * `id` - The uuid v4 id to search for
    pub fn find(_id: Uuid) -> Self { Default::default() }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn insert() {
        let mut config = crate::init().await.unwrap();
        let project = crate::db::projects::Project::new("PROJECT_NAME".into(), "PROJECT_DESCRIPTION".into())
            .insert();

        sqlx::query_with(&project.0, project.1).execute(&mut config.1).await.unwrap();

        assert_eq!(project.0, "INSERT INTO \"project\" (\"id\", \"name\", \"description\", \"created\", \"edited\") VALUES (?, ?, ?, ?, ?)");
    }
}

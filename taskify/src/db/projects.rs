//! Project Database Entity
//! This file contains the database entity for taskify.

use chrono::{DateTime, Utc};
use sea_query::{enum_def, Expr, Query, SqliteQueryBuilder};
use sea_query_binder::{SqlxBinder, SqlxValues};
use sqlx::{Error, FromRow, SqliteConnection};
use sqlx::sqlite::SqliteQueryResult;
use uuid::Uuid;

/// The database entity for taskify
#[enum_def]
#[derive(Debug, Default, Clone, PartialEq, FromRow)]
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

    /// Load a project from the DB
    ///
    /// Finds a project in the DB by providing the id (uuid) value and wraps it in a Project{} struct.
    /// # Arguments
    /// * `id` - The uuid v4 id to search for
    /// * `conn` - The SQLite database connection
    pub async fn from_db(id: Uuid, conn: &mut SqliteConnection) -> Result<Self, Error> {
        let query = Project::select_query(id);

         sqlx::query_as_with::<_, Project, _>(&query.0, query.1).fetch_one(conn).await
    }

    /// Get Project Name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get Project Description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get Project Author
    pub fn author(&self) -> &String {
        &self.author
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
    /// * `description` - The string description to use
    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.edited();
    }

    // Database Interactions

    /// Inserts Project to DB
    ///
    /// # Arguments
    /// * `conn` - The SQLite database connection
    pub async fn insert(&self, conn: &mut SqliteConnection) -> Result<SqliteQueryResult, Error> {
        let query = self.insert_query();

        sqlx::query_with(&query.0, query.1).execute(conn).await
    }

    /// Generates a sqlx query to Insert Project to DB
    ///
    /// # Examples
    /// ```
    /// # #[tokio::test]
    /// # async fn test() -> Result<(), Box<dyn std::error::Error>> {
    /// taskify::db::projects::Project::new("Name".into(), "Desc".into(), "Author".into()).insert_query();
    /// # Ok(())
    /// # }
    /// ```
    pub fn insert_query(&self) -> (String, SqlxValues) {
        Query::insert()
            .into_table(ProjectIden::Table)
            .columns([ProjectIden::Id, ProjectIden::Name, ProjectIden::Description, ProjectIden::Author, ProjectIden::Created, ProjectIden::Modified])
            .values([self.id.into(), self.name.clone().into(), self.description.clone().into(), self.author.clone().into(), self.created.into(), self.modified.into()])
            .unwrap().build_sqlx(SqliteQueryBuilder)
    }

    /// Updates a Project on DB
    ///
    /// # Arguments
    /// * `conn` - The SQLite database connection
    pub async fn update(&self, conn: &mut SqliteConnection) -> Result<SqliteQueryResult, Error> {
        let query = self.update_query();

        sqlx::query_with(&query.0, query.1).execute(conn).await
    }

    /// Generates a sqlx query to Update Project on DB
    ///
    /// # Examples
    /// ```
    /// # #[tokio::test]
    /// # async fn test() -> Result<(), Box<dyn std::error::Error>> {
    /// taskify::db::projects::Project::new("Name".into(), "Desc".into(), "Author".into()).update_query();
    /// # Ok(())
    /// # }
    /// ```
    pub fn update_query(&self) -> (String, SqlxValues) {
        Query::update()
            .table(ProjectIden::Table)
            .values([
                (ProjectIden::Name, self.name.clone().into()),
                (ProjectIden::Description, self.description.clone().into()),
                (ProjectIden::Author, self.author.clone().into()),
                (ProjectIden::Modified, self.modified.into())])
            .and_where(Expr::col(ProjectIden::Id).eq(self.id.clone())).build_sqlx(SqliteQueryBuilder)
    }

    /// Deletes a Project on DB
    ///
    /// # Arguments
    /// * `conn` - The SQLite database connection
    pub async fn delete(&self, conn: &mut SqliteConnection) ->Result<SqliteQueryResult, Error> {
        let query = self.delete_query();

        sqlx::query_with(&query.0, query.1).execute(conn).await
    }

    /// Generates a sqlx query to Delete Project on DB
    ///
    /// # Examples
    /// ```
    /// # #[tokio::test]
    /// # async fn test() -> Result<(), Box<dyn std::error::Error>> {
    /// taskify::db::projects::Project::new("Name".into(), "Desc".into(), "Author".into()).delete_query();
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_query(&self) -> (String, SqlxValues) {
        Query::delete()
            .from_table(ProjectIden::Table)
            .and_where(Expr::col(ProjectIden::Id).eq(self.id.clone())).build_sqlx(SqliteQueryBuilder)
    }

    /// Generates a sqlx query to Find a Project on DB
    ///
    /// Finds a project in the DB by providing the id (uuid) value.
    /// # Arguments
    /// * `id` - The uuid v4 id to search for
    pub fn select_query(id: Uuid) -> (String, SqlxValues) {
        Query::select()
            .columns([
                ProjectIden::Id,
                ProjectIden::Name,
                ProjectIden::Description,
                ProjectIden::Author,
                ProjectIden::Created,
                ProjectIden::Modified
            ])
            .from(ProjectIden::Table)
            .and_where(Expr::col(ProjectIden::Id).eq(id))
            .limit(1)
            .build_sqlx(SqliteQueryBuilder)
    }

    // Private Functions

    /// Set Edited
    ///
    /// Sets the project edited field to the current datetime
    fn edited(&mut self) {
        self.modified = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use sqlx::Error;
    use uuid::Uuid;
    use crate::config::init_memory_db;
    use super::Project;
    fn create_project() -> Project {
        Project::new("PROJECT_NAME".into(), "PROJECT_DESCRIPTION".into(), "PROJECT_AUTHOR".into())
    }

    #[test]
    fn insert() {
        let query = create_project().insert_query();

        assert_eq!(query.0, "INSERT INTO \"project\" (\"id\", \"name\", \"description\", \"author\", \"created\", \"modified\") VALUES (?, ?, ?, ?, ?, ?)");
    }

    #[test]
    fn update() {
        let query = create_project().update_query();

        assert_eq!(query.0, "UPDATE \"project\" SET \"name\" = ?, \"description\" = ?, \"author\" = ?, \"modified\" = ? WHERE \"id\" = ?")
    }

    #[test]
    fn delete() {
        let query = create_project().delete_query();

        assert_eq!(query.0, "DELETE FROM \"project\" WHERE \"id\" = ?");
    }

    #[test]
    fn query() {
        let query = Project::select_query(Uuid::default());

        assert_eq!(query.0, "SELECT \"name\", \"description\", \"author\", \"created\", \"modified\" FROM \"project\" WHERE \"id\" = ? LIMIT ?")
    }

    #[tokio::test]
    async fn insert_db() {
        let query = create_project()
            .insert(&mut init_memory_db().await.unwrap())
            .await.unwrap();

        assert_eq!(query.rows_affected(), 1);
        assert_eq!(query.last_insert_rowid(), 1);
    }

    #[tokio::test]
    async fn update_db() {
        let mut conn = init_memory_db().await.unwrap();
        let project = create_project();

        let query = project.update(&mut conn).await.unwrap();
        assert_eq!(query.rows_affected(), 0);
        assert_eq!(query.last_insert_rowid(), 1);

        let query = project.insert(&mut conn).await.unwrap();
        assert_eq!(query.rows_affected(), 1);
        assert_eq!(query.last_insert_rowid(), 1);

        let query = project.update(&mut conn).await.unwrap();
        assert_eq!(query.rows_affected(), 1);
        assert_eq!(query.last_insert_rowid(), 1)
    }

    #[tokio::test]
    async fn delete_db() {
        let mut conn = init_memory_db().await.unwrap();
        let project = create_project();

        let query = project.delete(&mut conn).await.unwrap();
        assert_eq!(query.rows_affected(), 0);
        assert_eq!(query.last_insert_rowid(), 1);

        let query = project.insert(&mut conn).await.unwrap();
        assert_eq!(query.rows_affected(), 1);
        assert_eq!(query.last_insert_rowid(), 1);

        let query = project.delete(&mut conn).await.unwrap();
        assert_eq!(query.rows_affected(), 1);
        assert_eq!(query.last_insert_rowid(), 1)
    }

    #[tokio::test]
    async fn query_db() {
        let mut conn = init_memory_db().await.unwrap();
        let mut project = create_project();
        project.assign_id().assign_created();

        let query = Project::from_db(project.id, &mut conn).await;
        assert_eq!(query.unwrap_err().to_string(), Error::RowNotFound.to_string());

        let query = project.insert(&mut conn).await.unwrap();
        assert_eq!(query.rows_affected(), 1);
        assert_eq!(query.last_insert_rowid(), 1);

        let query = Project::from_db(project.id, &mut conn).await.unwrap();
        assert_eq!(project, query);
    }
}

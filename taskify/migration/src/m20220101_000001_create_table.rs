use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Project::Id)
                            .uuid()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Project::Name).string_len(32).not_null())
                    .col(ColumnDef::new(Project::Desc).text())
                    .col(ColumnDef::new(Project::Author).text().not_null())
                    .col(ColumnDef::new(Project::Created).date_time())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Project::Table)
                    .name("idx-id")
                    .col(Project::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Project {
    Table,
    Id,
    Name,
    Desc,
    Author,
    Created,
}

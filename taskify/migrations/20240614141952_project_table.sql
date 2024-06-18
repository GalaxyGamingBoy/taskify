CREATE TABLE IF NOT EXISTS project (
    id VARCHAR(36) UNIQUE NOT NULL PRIMARY KEY,
    name VARCHAR(32),
    description TEXT,
    author TEXT NOT NULL,
    created TEXT NOT NULL,
    modified TEXT NOT NULL
)
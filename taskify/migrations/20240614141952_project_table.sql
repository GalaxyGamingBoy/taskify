CREATE TABLE IF NOT EXISTS projects (
    id VARCHAR(25) UNIQUE NOT NULL PRIMARY KEY,
    name VARCHAR(32),
    description TEXT,
    author TEXT NOT NULL,
    created TEXT NOT NULL,
    modified TEXT NOT NULL
)
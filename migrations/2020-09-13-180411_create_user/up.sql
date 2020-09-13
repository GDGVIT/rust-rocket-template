CREATE TABLE IF NOT EXISTS users
(
    id uuid,
    username VARCHAR NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT 'f',
    PRIMARY KEY (id)
)
-- Add up migration script here

CREATE TABLE IF NOT EXISTS posts (
    title VARCHAR(64) NOT NULL,
    content TEXT NOT NULL
);

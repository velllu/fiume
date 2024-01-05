CREATE DATABASE fiume;
\c fiume;

CREATE TABLE users(
    id SERIAL PRIMARY KEY UNIQUE,
    username VARCHAR(16) NOT NULL UNIQUE,
    password VARCHAR(128) NOT NULL, -- 128 is the size of a sha512 hash
    session_id VARCHAR(128) UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Add migration script here
CREATE TABLE IF NOT EXISTS customers (
    member_id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE
);

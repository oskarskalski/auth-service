-- Your SQL goes here
CREATE TABLE users (
    user_id VARCHAR PRIMARY KEY,
    username VARCHAR NOT NULL,
    e_mail VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    created_at BigInt NOT NULL
);
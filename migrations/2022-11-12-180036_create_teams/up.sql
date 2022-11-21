-- Your SQL goes here

CREATE TABLE teams(
    team_id VARCHAR PRIMARY KEY NOT NULL,
    creator_id VARCHAR NOT NULL,
    team_name VARCHAR NOT NULL UNIQUE,
    description VARCHAR NOT NULL
)
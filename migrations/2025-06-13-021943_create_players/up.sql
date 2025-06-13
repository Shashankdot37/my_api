-- Your SQL goes here

CREATE TABLE players (
    id SERIAL PRIMARY KEY,
    pname VARCHAR NOT NULL,
    jersey_no INT NOT NULL,
    available BOOLEAN NOT NULL DEFAULT 't'
)
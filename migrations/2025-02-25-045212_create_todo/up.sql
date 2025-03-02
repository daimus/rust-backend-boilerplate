-- Your SQL goes here
CREATE TABLE todos (
   id SERIAL PRIMARY KEY,
   activity VARCHAR NOT NULL,
   completed BOOLEAN NOT NULL DEFAULT FALSE
);
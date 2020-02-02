-- Your SQL goes here
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    email TEXT NOT NULL
)
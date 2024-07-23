-- Your SQL goes here

CREATE TABLE users (
  id serial PRIMARY KEY,
  name varchar NOT NULL,
  email varchar NOT NULL UNIQUE,
  phone varchar NOT NULL,
  address varchar NOT NULL
);

CREATE INDEX index_users_on_email ON users(email);

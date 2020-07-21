-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL
)


CREATE  TABLE posts(
    id INTEGER  PRIMARY KEY NOT NULL,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY(creator) REFERENCES users(id)
)
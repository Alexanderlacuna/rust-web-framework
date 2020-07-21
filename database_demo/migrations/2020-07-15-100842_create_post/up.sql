-- Your SQL goes here
-- Your SQL goes here
-- Your SQL goes here
-- CREATE TABLE users (
--     id INTEGER PRIMARY KEY NOT NULL,
--     username VARCHAR NOT NULL,
--     email VARCHAR NOT NULL
-- )


CREATE  TABLE posts(
    postId INTEGER  PRIMARY KEY NOT NULL,
    creator INTEGER,

    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY(creator) REFERENCES users(id)
)
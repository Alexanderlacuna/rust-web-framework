-- Your SQL goes here

CREATE  TABLE contents(
    postId INTEGER  PRIMARY KEY NOT NULL,
    user_id INTEGER,

    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
)